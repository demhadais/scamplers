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

pub const N_INSTITUTIONS: usize = 20;
pub const N_PEOPLE: usize = 100;
pub const N_LABS: usize = 25;
pub const N_LAB_MEMBERS: usize = 5;
pub const N_SPECIMENS: usize = 200;

pub struct TestState {
    _container: DevContainer,
    db_pool: Pool<AsyncPgConnection>,
    institutions: Vec<Institution>,
    people: Vec<Person>,
    labs: Vec<Lab>,
    specimens: Vec<Specimen>,
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

pub trait IntoSorted<Summary>: IntoIterator {
    fn filter_extract_sort<FilterFn, ExtractFn, CompareFn>(
        self,
        filter: FilterFn,
        extract: ExtractFn,
        compare: CompareFn,
    ) -> Vec<Summary>
    where
        Self: Sized,
        FilterFn: FnMut(&Self::Item) -> bool,
        ExtractFn: Fn(Self::Item) -> Summary,
        CompareFn: FnMut(&Summary, &Summary) -> Ordering;
}

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

                // let loaded_len = loaded_records.len();
                // let expected_len = data.len();

                // let greater_len = loaded_len.max(expected_len);

                // // Compare in a for-loop to have easier-to-read error messages
                // for (i, (loaded, expected)) in
                // loaded_records.into_iter().zip(data).enumerate() {
                //     assert_eq!(loaded, expected, "comparison failed on iteration {i}");
                // }

                // assert_eq!(
                //     loaded_len, expected_len,
                //     "loaded records length: {loaded_len}\nexpected data length:
                // {expected_len}" );

                Ok(())
            }
            .scope_boxed()
        })
        .await;
}
