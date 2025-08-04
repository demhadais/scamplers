use std::{cmp::Ordering, fmt::Debug};

use diesel_async::{
    AsyncConnection, AsyncPgConnection,
    pooled_connection::{
        AsyncDieselConnectionManager,
        deadpool::{Object, Pool},
    },
    scoped_futures::ScopedFutureExt,
};
use pretty_assertions::assert_eq;
use rand::seq::IndexedRandom;
use rstest::fixture;
use scamplers_core::model::{
    institution::{Institution, NewInstitution},
    lab::{Lab, NewLab},
    person::{NewPerson, Person},
    specimen::{
        self, NewSpecimen, Specimen,
        block::{
            BlockFixative, FixedBlockEmbeddingMatrix, FrozenBlockEmbeddingMatrix, NewFixedBlock,
            NewFrozenBlock,
        },
        common::{NewSpecimenCommon, NewSpecimenMeasurement, Species},
        tissue::{NewCryopreservedTissue, NewFixedTissue, NewFrozenTissue, TissueFixative},
    },
    suspension::{
        self, BiologicalMaterial, MultiplexingTagType, NewMultiplexingTag, NewSuspension,
        NewSuspensionMeasurement, NewSuspensionPool, NewSuspensionPoolMeasurement,
        SuspensionMeasurementData, SuspensionPoolHandle, SuspensionPoolMeasurementData,
    },
    units::VolumeUnit,
};
use strum::VariantArray;
use time::OffsetDateTime;
use tokio::sync::OnceCell;
use uuid::Uuid;

use crate::{
    db::model::{FetchByQuery, WriteToDb},
    result::ScamplersError,
    server::{run_migrations, util::DevContainer},
};

const N_INSTITUTIONS: usize = 20;
const N_PEOPLE: usize = 100;
const N_LABS: usize = 25;
pub const N_LAB_MEMBERS: usize = 5;

const N_SPECIMENS: usize = 400;

const N_MULTIPLEXING_TAGS: usize = 100;

const N_SUSPENSION_POOLS: usize = N_SPECIMENS / 4;
const N_SUSPENSIONS_PER_POOL: usize = 2;
const N_SUSPENSIONS: usize = N_SPECIMENS - N_SUSPENSIONS_PER_POOL;

const N_GEMS_PER_NONOCM_CHROMIUM_RUN: usize = 8;
const N_NONPOOL_GEMS: usize = N_SUSPENSIONS - (N_SUSPENSION_POOLS * N_SUSPENSIONS_PER_POOL) / 2;

const N_SINGLEPLEX_CHROMIUM_RUNS: usize = N_NONPOOL_GEMS / N_GEMS_PER_NONOCM_CHROMIUM_RUN;

const N_GEMS_PER_OCM_CHROMIUM_RUN: usize = 2;
const N_OCM_CHROMIUM_RUNS: usize = N_NONPOOL_GEMS / N_GEMS_PER_OCM_CHROMIUM_RUN;

const N_POOL_MULTIPLEX_CHROMIUM_RUNS: usize = N_SUSPENSION_POOLS / N_GEMS_PER_NONOCM_CHROMIUM_RUN;

pub struct TestState {
    _container: DevContainer,
    db_pool: Pool<AsyncPgConnection>,
    institutions: Vec<Institution>,
    people: Vec<Person>,
    labs: Vec<Lab>,
    specimens: Vec<Specimen>,
    suspension_pools: Vec<SuspensionPoolHandle>,
}
impl TestState {
    async fn new() -> Self {
        let name = "scamplers-backend_unit_test";
        let container = DevContainer::new(name, false).await.unwrap();

        let db_config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(
            container.db_url().await.unwrap(),
        );
        let db_pool = Pool::builder(db_config).build().unwrap();

        let mut test_state = Self {
            _container: container,
            db_pool,
            institutions: Vec::with_capacity(N_INSTITUTIONS),
            people: Vec::with_capacity(N_PEOPLE),
            labs: Vec::with_capacity(N_LABS),
            specimens: Vec::with_capacity(N_SPECIMENS),
            suspension_pools: Vec::with_capacity(N_SUSPENSION_POOLS),
        };

        test_state.populate_db().await;

        test_state
    }

    async fn populate_db(&mut self) {
        let Self {
            db_pool,
            institutions,
            people,
            labs,
            specimens,
            suspension_pools,
            ..
        } = self;

        let db_conn = db_pool.get().await.unwrap();
        run_migrations(db_conn).await.unwrap();

        let db_conn = &mut db_pool.get().await.unwrap();

        for i in 0..N_INSTITUTIONS {
            let new_institution = NewInstitution::builder()
                .id(Uuid::now_v7())
                .name(format!("institution{i}"))
                .build()
                .write_to_db(db_conn)
                .await
                .unwrap();

            institutions.push(new_institution);
        }

        let rng = rand::rng();

        for i in 0..N_PEOPLE {
            let institution_id = institutions.choose(&mut rng.clone()).unwrap().handle.id;

            let new_person = NewPerson::builder()
                .name(format!("person{i}"))
                .email(format!("person{i}@example.com"))
                .institution_id(institution_id)
                .build()
                .write_to_db(db_conn)
                .await
                .unwrap();

            people.push(new_person);
        }

        let random_people_ids = |amount| {
            people
                .choose_multiple(&mut rng.clone(), amount)
                .map(|p| p.core.summary.handle.id)
                .collect::<Vec<_>>()
        };

        let random_person_id = || random_people_ids(1)[0];

        for i in 0..N_LABS {
            let pi_id = random_person_id();
            let name = format!("lab{i}");
            // Use `N_LAB_MEMBERS - 1` because we're expecting to add the PI, so using this
            // constant later can be correct
            let member_ids = random_people_ids(N_LAB_MEMBERS - 1);

            let new_lab = NewLab::builder()
                .name(name.as_str())
                .pi_id(pi_id)
                .delivery_dir(format!("{name}_dir"))
                .member_ids(member_ids)
                .build()
                .write_to_db(db_conn)
                .await
                .unwrap();

            labs.push(new_lab);
        }

        let random_lab_id = || {
            labs.choose(&mut rng.clone())
                .unwrap()
                .core
                .summary
                .handle
                .id
        };

        let random_species = || Species::VARIANTS.choose(&mut rng.clone()).cloned().unwrap();

        let inner_specimen = NewSpecimenCommon::builder()
            .submitted_by(random_person_id())
            .lab_id(random_lab_id())
            .received_at(OffsetDateTime::now_utc())
            .species([random_species()])
            .measurements([NewSpecimenMeasurement::builder()
                .measured_by(random_person_id())
                .data(specimen::common::MeasurementData::Rin {
                    measured_at: OffsetDateTime::now_utc(),
                    instrument_name: "mayonnaise".into(),
                    value: 5.0,
                })
                .build()]);

        for i in 0..N_SPECIMENS {
            let inner_specimen = inner_specimen
                .clone()
                .readable_id(format!("SP{i}"))
                .name(format!("specimen{i}"))
                .build();

            let new_specimen: NewSpecimen = if i % 2 == 0 {
                NewCryopreservedTissue::builder()
                    .inner(inner_specimen)
                    .cryopreserved(true)
                    .storage_buffer("buffer")
                    .build()
                    .into()
            } else if i % 3 == 0 {
                NewFixedTissue::builder()
                    .inner(inner_specimen)
                    .fixative(TissueFixative::DithiobisSuccinimidylropionate)
                    .build()
                    .into()
            } else if i % 4 == 0 {
                NewFrozenTissue::builder()
                    .inner(inner_specimen)
                    .frozen(true)
                    .build()
                    .into()
            } else if i % 5 == 0 {
                NewFixedBlock::builder()
                    .inner(inner_specimen)
                    .fixative(BlockFixative::FormaldehydeDerivative)
                    .embedded_in(FixedBlockEmbeddingMatrix::Paraffin)
                    .build()
                    .into()
            } else {
                let random_embedding_matrix = || {
                    FrozenBlockEmbeddingMatrix::VARIANTS
                        .choose(&mut rng.clone())
                        .cloned()
                        .unwrap()
                };

                NewFrozenBlock::builder()
                    .inner(inner_specimen)
                    .frozen(true)
                    .embedded_in(random_embedding_matrix())
                    .build()
                    .into()
            };

            specimens.push(new_specimen.write_to_db(db_conn).await.unwrap());
        }

        let random_multiplexing_tag_type = || {
            MultiplexingTagType::VARIANTS
                .choose(&mut rng.clone())
                .cloned()
                .unwrap()
        };
        let mut multiplexing_tags = Vec::new();
        for i in 0..N_MULTIPLEXING_TAGS {
            multiplexing_tags.push(
                NewMultiplexingTag::builder()
                    .tag_id(format!("{i}"))
                    .type_(random_multiplexing_tag_type())
                    .build()
                    .write_to_db(db_conn)
                    .await
                    .unwrap(),
            );
        }

        let random_specimen_id = || {
            specimens
                .choose(&mut rng.clone())
                .cloned()
                .unwrap()
                .core
                .summary
                .handle
                .id
        };
        let random_multiplexing_tag = || multiplexing_tags.choose(&mut rng.clone()).unwrap().id;
        let new_suspension_measurement = NewSuspensionMeasurement::builder()
            .measured_by(random_person_id())
            .data(SuspensionMeasurementData {
                core: suspension::MeasurementDataCore::Volume {
                    measured_at: OffsetDateTime::now_utc(),
                    value: 10.0,
                    unit: VolumeUnit::Microliter,
                },
                is_post_hybridization: true,
            })
            .build();
        let new_suspension = |i: usize| {
            NewSuspension::builder()
                .biological_material(BiologicalMaterial::Cells)
                .readable_id(format!("S{i}"))
                .parent_specimen_id(random_specimen_id())
                .multiplexing_tag_id(random_multiplexing_tag())
                .target_cell_recovery(5_000.0)
                .target_reads_per_cell(50_000)
                .preparer_ids(random_people_ids(2))
                .measurements([new_suspension_measurement.clone()])
                .build()
        };
        let new_suspension_pool_measurement = NewSuspensionPoolMeasurement::builder()
            .measured_by(random_person_id())
            .data(SuspensionPoolMeasurementData {
                data: suspension::MeasurementDataCore::Volume {
                    measured_at: OffsetDateTime::now_utc(),
                    value: 10.0,
                    unit: VolumeUnit::Microliter,
                },
                is_post_storage: false,
            })
            .build();
        for i in 0..N_SUSPENSION_POOLS {
            let new_suspension_pool = NewSuspensionPool::builder()
                .readable_id(format!("P{i}"))
                .name(format!("pool{i}"))
                .pooled_at(OffsetDateTime::now_utc())
                .preparer_ids(random_people_ids(2))
                .suspensions(
                    (0..N_SUSPENSIONS_PER_POOL)
                        .map(&new_suspension)
                        .collect::<Vec<_>>(),
                )
                .measurements([new_suspension_pool_measurement.clone()])
                .build();

            suspension_pools.push(new_suspension_pool.write_to_db(db_conn).await.unwrap());
        }
    }
}

static TEST_STATE: OnceCell<TestState> = OnceCell::const_new();
pub type DbConnection = Object<AsyncPgConnection>;

#[fixture]
pub async fn db_conn() -> DbConnection {
    let test_state = TEST_STATE.get_or_init(TestState::new).await;

    test_state.db_pool.get().await.unwrap()
}

macro_rules! data_fixtures {
    ($(($field:ident, $ret_type:path));*) => {
        $(
            #[::rstest::fixture]
            pub async fn $field() -> Vec<$ret_type> {
                TEST_STATE.get_or_init(TestState::new).await.$field.clone()
            }
        )*
    };
}

data_fixtures!((institutions, Institution); (people, Person); (labs, Lab); (specimens, Specimen));

#[bon::builder]
fn extract_filter_sort<Record, Summary>(
    data: Vec<Record>,
    extract: fn(Record) -> Summary,
    filter: Option<fn(&Summary) -> bool>,
    sort_by: Option<fn(&Summary, &Summary) -> Ordering>,
) -> Vec<Summary> {
    fn identity_filter<M>(_: &M) -> bool {
        true
    }

    let filter = filter.unwrap_or(identity_filter);

    let mut data: Vec<_> = data.into_iter().map(extract).filter(filter).collect();

    if let Some(compare) = sort_by {
        data.sort_by(compare);
    }

    data
}

#[bon::builder]
#[builder(finish_fn = run)]
pub async fn test_query<FullRecord, Summary>(
    #[builder(finish_fn)] mut db_conn: DbConnection,
    #[builder(default)] db_query: Summary::QueryParams,
    all_data: Vec<FullRecord>,
    extract: fn(FullRecord) -> Summary,
    filter: Option<fn(&Summary) -> bool>,
    sort_by: Option<fn(&Summary, &Summary) -> Ordering>,
) where
    Summary: FetchByQuery + PartialEq + Debug + Send,
    Summary::QueryParams: Send + Default,
{
    let data = extract_filter_sort()
        .data(all_data)
        .extract(extract)
        .maybe_filter(filter)
        .maybe_sort_by(sort_by)
        .call();

    db_conn
        .test_transaction::<_, ScamplersError, _>(|conn| {
            async move {
                let loaded_records = Summary::fetch_by_query(&db_query, conn).await.unwrap();

                assert_eq!(loaded_records, data);

                Ok(())
            }
            .scope_boxed()
        })
        .await;
}
