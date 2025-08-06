#![allow(dead_code)]
use std::{cmp::Ordering, collections::HashSet, fmt::Debug};

use diesel_async::{
    AsyncConnection, AsyncPgConnection,
    pooled_connection::{
        AsyncDieselConnectionManager,
        deadpool::{Object, Pool},
    },
    scoped_futures::ScopedFutureExt,
};
use pretty_assertions::assert_eq;
use rand::{
    SeedableRng,
    rngs::StdRng,
    seq::{IndexedRandom, IteratorRandom},
};
use rstest::fixture;
use scamplers_core::model::{
    chromium_run::{
        ChromiumRun, NewChipLoadingCommon, NewChromiumRun, NewChromiumRunCommon, NewGemsCommon,
        NewPoolMultiplexChipLoading, NewPoolMultiplexChromiumRun, NewPoolMultiplexGems,
        PoolMultiplexChromiumChip,
    },
    dataset::DatasetSummary,
    institution::{Institution, NewInstitution},
    lab::{Lab, NewLab},
    nucleic_acid::{CdnaHandle, LibraryHandle},
    person::{NewPerson, Person},
    sequencing_run::SequencingRunSummary,
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
    db::{
        model::{FetchByQuery, WriteToDb},
        seed_data::SeedData,
    },
    result::ScamplersError,
    server::{run_migrations, util::DevContainer},
};

trait ChooseUnwrap<T> {
    fn choose_unwrap(&self, rng: &mut StdRng) -> &T;
}
impl<T> ChooseUnwrap<T> for Vec<T> {
    fn choose_unwrap(&self, rng: &mut StdRng) -> &T {
        self.choose(rng).unwrap()
    }
}
trait ChooseUnwrapOwned<T> {
    fn choose_unwrap_owned(self, rng: &mut StdRng) -> T;
}
impl ChooseUnwrapOwned<i64> for std::ops::Range<i64> {
    fn choose_unwrap_owned(self, rng: &mut StdRng) -> i64 {
        self.choose(rng).unwrap()
    }
}

const N_INSTITUTIONS: usize = 10;
const N_PEOPLE: usize = 250;
const N_LABS: usize = 50;
pub const N_LAB_MEMBERS: usize = 5;

pub const N_SPECIMENS: usize = 1000;

const N_MULTIPLEXING_TAGS: usize = 100;

// 25% of the specimens will be pooled
const N_SUSPENSION_POOLS: usize = N_SPECIMENS / 4;
const N_SUSPENSIONS_PER_POOL: usize = 2;

// The remaining specimens will become singular suspensions
const N_SUSPENSIONS: usize = N_SPECIMENS - N_SUSPENSIONS_PER_POOL;

const N_GEMS_PER_NONOCM_CHROMIUM_RUN: usize = 8;
const N_GEMS_PER_OCM_CHROMIUM_RUN: usize = 2;
const N_SUSPENSIONS_PER_OCM_GEMS: usize = 4;

// Every suspension can be used both for singleplex and OCM runs
const N_SINGLEPLEX_CHROMIUM_RUNS: usize = N_SUSPENSIONS / N_GEMS_PER_NONOCM_CHROMIUM_RUN;
const N_OCM_CHROMIUM_RUNS: usize =
    N_SUSPENSIONS / (N_GEMS_PER_OCM_CHROMIUM_RUN * N_SUSPENSIONS_PER_OCM_GEMS);

// Every suspension pool can be used for a pool multiplex chromium run
const N_POOL_MULTIPLEX_CHROMIUM_RUNS: usize = N_SUSPENSION_POOLS / N_GEMS_PER_NONOCM_CHROMIUM_RUN;

const N_CDNA: usize = (N_SINGLEPLEX_CHROMIUM_RUNS * N_GEMS_PER_NONOCM_CHROMIUM_RUN)
    + (N_OCM_CHROMIUM_RUNS * N_GEMS_PER_OCM_CHROMIUM_RUN)
    + (N_POOL_MULTIPLEX_CHROMIUM_RUNS * N_GEMS_PER_NONOCM_CHROMIUM_RUN);

const N_LIBRARIES: usize = N_CDNA;

const N_SEQUENCING_RUNS: usize = 1;

const N_DATASETS: usize = N_LIBRARIES;

const CHEMISTRY: &str = "MFRP-RNA";

pub struct TestState {
    _container: DevContainer,
    rng: StdRng,
    db_pool: Pool<AsyncPgConnection>,
    institutions: Vec<Institution>,
    people: Vec<Person>,
    labs: Vec<Lab>,
    specimens: Vec<Specimen>,
    multiplexing_tags: Vec<MultiplexingTag>,
    suspension_pools: Vec<SuspensionPoolHandle>,
    chromium_runs: Vec<ChromiumRun>,
    cdna: Vec<CdnaHandle>,
    libraries: Vec<LibraryHandle>,
    sequencing_runs: Vec<SequencingRunSummary>,
    datasets: Vec<DatasetSummary>,
}
impl TestState {
    fn random_time(&mut self) -> OffsetDateTime {
        // These numbers correspond to the first second of the year -4000 and the last second of the year 4000 (https://www.postgresql.org/docs/current/datatype-datetime.html)
        OffsetDateTime::from_unix_timestamp(
            (-188_395_009_438..64_092_229_199).choose_unwrap_owned(&mut self.rng),
        )
        .unwrap()
    }

    async fn insert_seed_data(&mut self, db_conn: &mut DbConnection) {
        let seed_data: SeedData =
            serde_json::from_str(include_str!("../../../../seed_data.sample.json")).unwrap();

        seed_data
            .write(db_conn, reqwest::Client::new())
            .await
            .unwrap();
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
        self.institutions.choose_unwrap(&mut self.rng).handle.id
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

    fn random_person_id(&mut self) -> Uuid {
        self.people
            .choose_unwrap(&mut self.rng)
            .core
            .summary
            .handle
            .id
    }

    fn random_people_ids(&mut self, n: usize) -> Vec<Uuid> {
        let set: HashSet<_> = (0..n).map(|_| self.random_person_id()).collect();
        set.into_iter().collect()
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

    fn random_lab_id(&mut self) -> Uuid {
        self.labs
            .choose_unwrap(&mut self.rng)
            .core
            .summary
            .handle
            .id
    }

    async fn insert_specimens(&mut self, db_conn: &mut DbConnection) {
        for i in 0..N_SPECIMENS {
            let measurement = NewSpecimenMeasurement::builder()
                .measured_by(self.random_person_id())
                .data(specimen::common::MeasurementData::Rin {
                    measured_at: self.random_time(),
                    instrument_name: "mayonnaise".into(),
                    value: 5.0,
                })
                .build();

            let random_species = Species::VARIANTS.choose(&mut self.rng).copied().unwrap();
            let inner_specimen = NewSpecimenCommon::builder()
                .readable_id(Uuid::now_v7().to_string())
                .submitted_by(self.random_person_id())
                .lab_id(self.random_lab_id())
                .received_at(self.random_time())
                .species([random_species])
                .measurements([measurement])
                .name(format!("specimen{i}"))
                .build();

            let new_specimen: NewSpecimen = if i % 5 == 0 {
                NewCryopreservedTissue::builder()
                    .inner(inner_specimen)
                    .cryopreserved(true)
                    .storage_buffer("buffer")
                    .build()
                    .into()
            } else if i % 4 == 0 {
                NewFixedTissue::builder()
                    .inner(inner_specimen)
                    .fixative(TissueFixative::DithiobisSuccinimidylropionate)
                    .build()
                    .into()
            } else if i % 3 == 0 {
                NewFrozenTissue::builder()
                    .inner(inner_specimen)
                    .frozen(true)
                    .build()
                    .into()
            } else if i % 2 == 0 {
                NewFixedBlock::builder()
                    .inner(inner_specimen)
                    .fixative(BlockFixative::FormaldehydeDerivative)
                    .embedded_in(FixedBlockEmbeddingMatrix::Paraffin)
                    .build()
                    .into()
            } else {
                let random_embedding_matrix = FrozenBlockEmbeddingMatrix::VARIANTS
                    .choose(&mut self.rng)
                    .copied()
                    .unwrap();

                NewFrozenBlock::builder()
                    .inner(inner_specimen)
                    .frozen(true)
                    .embedded_in(random_embedding_matrix)
                    .build()
                    .into()
            };

            self.specimens
                .push(new_specimen.write_to_db(db_conn).await.unwrap());
        }
    }

    fn random_specimen_id(&mut self) -> Uuid {
        self.specimens
            .choose_unwrap(&mut self.rng)
            .core
            .summary
            .handle
            .id
    }

    async fn insert_multiplexing_tags(&mut self, db_conn: &mut DbConnection) {
        for i in 0..N_MULTIPLEXING_TAGS {
            let random_multiplexing_tag_type = MultiplexingTagType::VARIANTS
                .choose(&mut self.rng)
                .copied()
                .unwrap();

            let multiplexing_tag = NewMultiplexingTag::builder()
                .tag_id(format!("{i}"))
                .type_(random_multiplexing_tag_type)
                .build()
                .write_to_db(db_conn)
                .await
                .unwrap();

            self.multiplexing_tags.push(multiplexing_tag);
        }
    }

    fn random_multiplexing_tag_id(&mut self) -> Uuid {
        self.multiplexing_tags.choose_unwrap(&mut self.rng).id
    }

    fn new_suspensions(&mut self, n: usize, for_pool: bool) -> Vec<NewSuspension> {
        (0..n)
            .map(|i| {
                let new_suspension_measurements: Vec<_> = (0..2)
                    .map(|i| {
                        NewSuspensionMeasurement::builder()
                            .measured_by(self.random_person_id())
                            .data(SuspensionMeasurementData {
                                core: suspension::MeasurementDataCore::Volume {
                                    measured_at: self.random_time(),
                                    value: 10.0 * i as f32,
                                    unit: VolumeUnit::Microliter,
                                },
                                is_post_hybridization: for_pool,
                            })
                            .build()
                    })
                    .collect();

                let new_suspension = NewSuspension::builder()
                    .biological_material(BiologicalMaterial::Cells)
                    .readable_id(Uuid::now_v7().to_string())
                    .parent_specimen_id(self.random_specimen_id())
                    .target_cell_recovery(5_000.0 + i as f32)
                    .target_reads_per_cell(50_000 + i as i32)
                    .measurements(new_suspension_measurements)
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

    fn suspension_volume(&mut self) -> suspension::MeasurementDataCore {
        suspension::MeasurementDataCore::Volume {
            measured_at: self.random_time(),
            value: 10.0,
            unit: VolumeUnit::Microliter,
        }
    }

    async fn insert_suspension_pools(&mut self, db_conn: &mut DbConnection) {
        for i in 0..N_SUSPENSION_POOLS {
            let new_suspension_pool_measurement = NewSuspensionPoolMeasurement::builder()
                .measured_by(self.random_person_id())
                .data(SuspensionPoolMeasurementData {
                    data: self.suspension_volume(),
                    is_post_storage: false,
                })
                .build();

            let new_suspension_pool = NewSuspensionPool::builder()
                .readable_id(Uuid::now_v7().to_string())
                .name(format!("pool{i}"))
                .pooled_at(self.random_time())
                .preparer_ids(self.random_people_ids(2))
                .suspensions(self.new_suspensions(N_SUSPENSIONS_PER_POOL, true))
                .measurements([new_suspension_pool_measurement])
                .build();

            self.suspension_pools
                .push(new_suspension_pool.write_to_db(db_conn).await.unwrap());
        }
    }

    fn random_suspension_pool_id(&mut self) -> Uuid {
        self.suspension_pools.choose_unwrap(&mut self.rng).id
    }

    async fn insert_pool_multiplexed_chromium_runs(&mut self, db_conn: &mut DbConnection) {
        for i in 0..N_POOL_MULTIPLEX_CHROMIUM_RUNS {
            let chromium_run_common = NewChromiumRunCommon::builder()
                .readable_id(format!("PMCR{i}"))
                .run_at(self.random_time())
                .run_by(self.random_person_id())
                .succeeded(true)
                .build();

            let chip_loading_common = NewChipLoadingCommon::builder()
                .suspension_volume_loaded(self.suspension_volume())
                .buffer_volume_loaded(self.suspension_volume())
                .build();

            let gems: Vec<_> = (0..N_GEMS_PER_NONOCM_CHROMIUM_RUN)
                .map(|j| {
                    let chip_loading = NewPoolMultiplexChipLoading::builder()
                        .inner(chip_loading_common.clone())
                        .suspension_pool_id(self.random_suspension_pool_id())
                        .build();

                    NewPoolMultiplexGems::builder()
                        .loading(chip_loading)
                        .inner(
                            NewGemsCommon::builder()
                                .chemistry(CHEMISTRY)
                                .readable_id(format!("G{}", i + j))
                                .build(),
                        )
                        .build()
                })
                .collect();

            let chromium_run = NewPoolMultiplexChromiumRun::builder()
                .inner(chromium_run_common)
                .chip(PoolMultiplexChromiumChip::GemxFx)
                .gems(gems)
                .build();

            self.chromium_runs.push(
                NewChromiumRun::PoolMultiplex(chromium_run)
                    .write_to_db(db_conn)
                    .await
                    .unwrap(),
            );
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
        self.insert_pool_multiplexed_chromium_runs(db_conn).await;
    }

    async fn new() -> Self {
        let name = "scamplers-backend_unit_test";
        let container = DevContainer::new(name, false).await.unwrap();

        let db_config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(
            container.db_url().await.unwrap(),
        );
        let db_pool = Pool::builder(db_config).build().unwrap();

        let mut test_state = Self {
            _container: container,
            rng: StdRng::from_os_rng(),
            db_pool,
            institutions: Vec::with_capacity(N_INSTITUTIONS),
            people: Vec::with_capacity(N_PEOPLE),
            labs: Vec::with_capacity(N_LABS),
            specimens: Vec::with_capacity(N_SPECIMENS),
            suspension_pools: Vec::with_capacity(N_SUSPENSION_POOLS),
            multiplexing_tags: Vec::with_capacity(N_MULTIPLEXING_TAGS),
            chromium_runs: Vec::with_capacity(
                N_SINGLEPLEX_CHROMIUM_RUNS + N_OCM_CHROMIUM_RUNS + N_POOL_MULTIPLEX_CHROMIUM_RUNS,
            ),
            cdna: Vec::with_capacity(N_CDNA),
            libraries: Vec::with_capacity(N_LIBRARIES),
            sequencing_runs: Vec::with_capacity(N_SEQUENCING_RUNS),
            datasets: Vec::with_capacity(N_DATASETS),
        };

        test_state.populate_db().await;

        test_state
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

                assert_eq!(
                    loaded_records.len(),
                    data.len(),
                    "filter returned different number of records"
                );

                for loaded in &loaded_records {
                    assert!(data.contains(loaded));
                }

                for expected in &data {
                    assert!(loaded_records.contains(expected));
                }

                for (i, (loaded, expected)) in loaded_records.iter().zip(&data).enumerate() {
                    assert_eq!(loaded, expected, "comparison failed at record {i}");
                }

                Ok(())
            }
            .scope_boxed()
        })
        .await;
}
