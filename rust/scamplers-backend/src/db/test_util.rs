#![allow(dead_code)]
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
        self, BiologicalMaterial, MultiplexingTag, MultiplexingTagType, NewMultiplexingTag,
        NewSuspension, NewSuspensionMeasurement, NewSuspensionPool, NewSuspensionPoolMeasurement,
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

trait ChooseUnwrap<T> {
    fn choose_unwrap(&self) -> &T;
}
impl<T> ChooseUnwrap<T> for Vec<T> {
    fn choose_unwrap(&self) -> &T {
        self.choose(&mut rand::rng()).unwrap()
    }
}

const N_INSTITUTIONS: usize = 20;
const N_PEOPLE: usize = 500;
const N_LABS: usize = 50;
pub const N_LAB_MEMBERS: usize = 10;

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
    multiplexing_tags: Vec<MultiplexingTag>,
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
            multiplexing_tags: Vec::with_capacity(N_MULTIPLEXING_TAGS),
        };

        test_state.populate_db().await;

        test_state
    }

    async fn insert_institutions(&mut self, db_conn: &mut DbConnection) {
        for i in 0..N_INSTITUTIONS {
            let new_institution = NewInstitution::builder()
                .id(Uuid::now_v7())
                .name(format!("institution{i}"))
                .build()
                .write_to_db(db_conn)
                .await
                .unwrap();

            self.institutions.push(new_institution);
        }
    }

    fn random_institution_id(&mut self) -> Uuid {
        self.institutions.choose_unwrap().handle.id
    }

    async fn insert_people(&mut self, db_conn: &mut DbConnection) {
        for i in 0..N_PEOPLE {
            let new_person = NewPerson::builder()
                .name(format!("person{i}"))
                .email(format!("person{i}@example.com"))
                .institution_id(self.random_institution_id())
                .build()
                .write_to_db(db_conn)
                .await
                .unwrap();

            self.people.push(new_person);
        }
    }

    fn random_person_id(&self) -> Uuid {
        self.people.choose_unwrap().core.summary.handle.id
    }

    fn random_people_ids(&self, n: usize) -> Vec<Uuid> {
        (0..n).map(|_| self.random_person_id()).collect()
    }

    async fn insert_labs(&mut self, db_conn: &mut DbConnection) {
        for i in 0..N_LABS {
            let pi_id = self.random_person_id();
            let name = format!("lab{i}");
            // Use `N_LAB_MEMBERS - 1` because we're expecting to add the PI, so using this
            // constant later can be correct
            let member_ids = self.random_people_ids(N_LAB_MEMBERS - 1);

            let new_lab = NewLab::builder()
                .name(name.as_str())
                .pi_id(pi_id)
                .delivery_dir(format!("{name}_dir"))
                .member_ids(member_ids)
                .build()
                .write_to_db(db_conn)
                .await
                .unwrap();

            self.labs.push(new_lab);
        }
    }

    fn random_lab_id(&self) -> Uuid {
        self.labs.choose_unwrap().core.summary.handle.id
    }

    async fn insert_specimens(&mut self, db_conn: &mut DbConnection) {
        let rng = rand::rng();
        let random_species = || Species::VARIANTS.choose(&mut rng.clone()).cloned().unwrap();

        let inner_specimen = NewSpecimenCommon::builder()
            .submitted_by(self.random_person_id())
            .lab_id(self.random_lab_id())
            .received_at(OffsetDateTime::now_utc())
            .species([random_species()])
            .measurements([NewSpecimenMeasurement::builder()
                .measured_by(self.random_person_id())
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

            self.specimens
                .push(new_specimen.write_to_db(db_conn).await.unwrap());
        }
    }

    fn random_specimen_id(&self) -> Uuid {
        self.specimens.choose_unwrap().core.summary.handle.id
    }

    async fn insert_multiplexing_tags(&mut self, db_conn: &mut DbConnection) {
        let random_multiplexing_tag_type = || {
            MultiplexingTagType::VARIANTS
                .choose(&mut rand::rng().clone())
                .cloned()
                .unwrap()
        };

        for i in 0..N_MULTIPLEXING_TAGS {
            let multiplexing_tag = NewMultiplexingTag::builder()
                .tag_id(format!("{i}"))
                .type_(random_multiplexing_tag_type())
                .build()
                .write_to_db(db_conn)
                .await
                .unwrap();

            self.multiplexing_tags.push(multiplexing_tag);
        }
    }

    fn random_multiplexing_tag_id(&self) -> Uuid {
        self.multiplexing_tags.choose_unwrap().id
    }

    fn new_suspensions(&self, n: usize, for_pool: bool) -> Vec<NewSuspension> {
        let new_suspension_measurements: Vec<_> = (0..2)
            .map(|i| {
                NewSuspensionMeasurement::builder()
                    .measured_by(self.random_person_id())
                    .data(SuspensionMeasurementData {
                        core: suspension::MeasurementDataCore::Volume {
                            measured_at: OffsetDateTime::now_utc(),
                            value: 10.0 * i as f32,
                            unit: VolumeUnit::Microliter,
                        },
                        is_post_hybridization: for_pool,
                    })
                    .build()
            })
            .collect();

        (0..n)
            .map(|i| {
                let new_suspension = NewSuspension::builder()
                    .biological_material(BiologicalMaterial::Cells)
                    .readable_id(format!("S{i}"))
                    .parent_specimen_id(self.random_specimen_id())
                    .target_cell_recovery(5_000.0 + i as f32)
                    .target_reads_per_cell(50_000 + i as i32)
                    .measurements(new_suspension_measurements.clone())
                    .preparer_ids(self.random_people_ids(2));

                if for_pool {
                    new_suspension
                        .multiplexing_tag_id(self.random_multiplexing_tag_id())
                        .build()
                } else {
                    new_suspension.build()
                }
            })
            .collect()
    }

    async fn insert_suspension_pools(&mut self, db_conn: &mut DbConnection) {
        let new_suspension_pool_measurement = NewSuspensionPoolMeasurement::builder()
            .measured_by(self.random_person_id())
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
                .preparer_ids(self.random_people_ids(2))
                .suspensions(self.new_suspensions(N_SUSPENSIONS_PER_POOL, true))
                .measurements([new_suspension_pool_measurement.clone()])
                .build();

            self.suspension_pools
                .push(new_suspension_pool.write_to_db(db_conn).await.unwrap());
        }
    }

    async fn populate_db(&mut self) {
        let Self { db_pool, .. } = self;

        let db_conn = db_pool.get().await.unwrap();
        run_migrations(db_conn).await.unwrap();

        let db_conn = &mut db_pool.get().await.unwrap();

        self.insert_institutions(db_conn).await;
        self.insert_people(db_conn).await;
        self.insert_labs(db_conn).await;
        self.insert_specimens(db_conn).await;
        self.insert_multiplexing_tags(db_conn).await;
        self.insert_suspension_pools(db_conn).await;
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
