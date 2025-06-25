use crate::{
    db::{
        self,
        model::{
            AsDieselFilter, AsDieselQueryBase, FetchById, FetchByQuery, FetchRelatives, WriteToDb,
        },
        util::{AsIlike, BoxedDieselExpression, NewBoxedDieselExpression},
    },
    fetch_by_query,
};
use diesel::{dsl::AssumeNotNull, prelude::*};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use scamplers_core::model::specimen::{
    BlockEmbeddingMatrix, Fixative, NewSpecimen, NewSpecimenMeasurement, Specimen, SpecimenCore,
    SpecimenMeasurement, SpecimenQuery, SpecimenSummary, block::NewBlock, tissue::NewTissue,
};
use scamplers_schema::{
    lab, person,
    specimen::{
        self, cryopreserved as cryopreserved_col, embedded_in as embedding_col,
        fixative as fixative_col, frozen as frozen_col, id as id_col, name as name_col,
        notes as notes_col, received_at as received_at_col, species as species_col,
        storage_buffer as buffer_col, type_ as type_col,
    },
    specimen_measurement,
};
use uuid::Uuid;

macro_rules! write_specimen_variant {
    ($specimen_variant:ident, $db_conn:ident) => {{
        diesel::insert_into(specimen::table)
            .values($specimen_variant)
            .returning(id_col)
            .get_result($db_conn)
            .await?
    }};
}

#[diesel::dsl::auto_type]
fn specimen_measurement_query_base() -> _ {
    specimen_measurement::table.inner_join(person::table)
}

impl FetchRelatives<SpecimenMeasurement> for specimen::table {
    type Id = Uuid;

    async fn fetch_relatives(
        id: &Self::Id,
        db_conn: &mut AsyncPgConnection,
    ) -> db::error::Result<Vec<SpecimenMeasurement>> {
        Ok(specimen_measurement_query_base()
            .filter(specimen_measurement::specimen_id.eq(id))
            .select(SpecimenMeasurement::as_select())
            .load(db_conn)
            .await?)
    }
}

impl WriteToDb for Vec<NewSpecimenMeasurement> {
    type Returns = Vec<SpecimenMeasurement>;

    async fn write(self, db_conn: &mut AsyncPgConnection) -> db::error::Result<Self::Returns> {
        let specimen_ids: Vec<Uuid> = diesel::insert_into(specimen_measurement::table)
            .values(&self)
            .returning(specimen_measurement::specimen_id)
            .get_results(db_conn)
            .await?;

        if specimen_ids.is_empty() {
            return Ok(vec![]);
        }

        specimen::table::fetch_relatives(&specimen_ids[0], db_conn).await
    }
}

impl WriteToDb for NewSpecimen {
    type Returns = Specimen;

    async fn write(
        self,
        db_conn: &mut diesel_async::AsyncPgConnection,
    ) -> crate::db::error::Result<Self::Returns> {
        let id = match &self {
            Self::Block(block) => match block {
                NewBlock::Fixed(block) => write_specimen_variant!(block, db_conn),
                NewBlock::Frozen(block) => write_specimen_variant!(block, db_conn),
            },
            Self::Suspension(suspension) => write_specimen_variant!(suspension, db_conn),
            Self::Tissue(tissue) => match tissue {
                NewTissue::Cryopreserved(tissue) => write_specimen_variant!(tissue, db_conn),
                NewTissue::Fixed(tissue) => write_specimen_variant!(tissue, db_conn),
                NewTissue::Frozen(tissue) => write_specimen_variant!(tissue, db_conn),
            },
        };

        let new_measurements = self.measurements(id);
        new_measurements.write(db_conn).await?;

        Specimen::fetch_by_id(&id, db_conn).await
    }
}

diesel::alias!(person as returned_by: ReturnedByAlias);

#[diesel::dsl::auto_type]
#[must_use]
fn core_query_base() -> _ {
    let submitter_join_condition = specimen::submitted_by.eq(person::id);
    let returner_join_condition =
        specimen::returned_by.eq(returned_by.field(person::id).nullable());

    summary_query_base()
        .inner_join(person::table.on(submitter_join_condition))
        .left_join(returned_by.on(returner_join_condition))
        .inner_join(lab::table)
}

impl FetchById for Specimen {
    type Id = Uuid;
    async fn fetch_by_id(
        id: &Self::Id,
        db_conn: &mut AsyncPgConnection,
    ) -> db::error::Result<Self> {
        let specimen_core = core_query_base()
            .select(SpecimenCore::as_select())
            .filter(id_col.eq(id))
            .first(db_conn)
            .await?;

        let measurements = specimen::table::fetch_relatives(specimen_core.id(), db_conn).await?;

        Ok(Specimen::builder()
            .core(specimen_core)
            .measurements(measurements)
            .build())
    }
}

impl<QueryExpression> AsDieselFilter<QueryExpression> for SpecimenQuery
where
    id_col: SelectableExpression<QueryExpression>,
    name_col: SelectableExpression<QueryExpression>,
    received_at_col: SelectableExpression<QueryExpression>,
    species_col: SelectableExpression<QueryExpression>,
    AssumeNotNull<notes_col>: SelectableExpression<QueryExpression>,
    type_col: SelectableExpression<QueryExpression>,
    AssumeNotNull<embedding_col>: SelectableExpression<QueryExpression>,
    AssumeNotNull<fixative_col>: SelectableExpression<QueryExpression>,
    AssumeNotNull<buffer_col>: SelectableExpression<QueryExpression>,
    frozen_col: SelectableExpression<QueryExpression>,
    cryopreserved_col: SelectableExpression<QueryExpression>,
{
    fn as_diesel_filter<'a>(
        &'a self,
    ) -> Option<db::util::BoxedDieselExpression<'a, QueryExpression>>
    where
        QueryExpression: 'a,
    {
        let Self {
            ids,
            name,
            received_before,
            received_after,
            species,
            notes,
            type_,
            embedded_in,
            fixative,
            storage_buffer,
            frozen,
            cryopreserved,
            ..
        } = self;

        let q1 = (!ids.is_empty()).then(|| id_col.eq_any(ids));
        let q2 = name.as_ref().map(|name| name_col.ilike(name.as_ilike()));
        let q3 = received_before
            .as_ref()
            .map(|received_before| received_at_col.lt(received_before));
        let q4 = received_after
            .as_ref()
            .map(|received_after| received_at_col.gt(received_after));
        let q5 = (!species.is_empty()).then(|| species_col.overlaps_with(species));
        let q6 = notes
            .as_ref()
            .map(|notes| notes_col.assume_not_null().ilike(notes.as_ilike()));
        let q7 = type_.map(|t| type_col.eq(t));
        let q8 = storage_buffer
            .as_ref()
            .map(|buf| buffer_col.assume_not_null().ilike(buf.as_ilike()));
        let q9 = frozen.map(|f| frozen_col.eq(f));
        let q10 = cryopreserved.map(|c| cryopreserved_col.eq(c));

        let mut query = BoxedDieselExpression::new_expression()
            .and_condition(q1)
            .and_condition(q2)
            .and_condition(q3)
            .and_condition(q4)
            .and_condition(q5)
            .and_condition(q6)
            .and_condition(q7)
            .and_condition(q8)
            .and_condition(q9)
            .and_condition(q10);

        if let Some(embedded_in) = embedded_in {
            match embedded_in {
                BlockEmbeddingMatrix::Fixed(e) => {
                    query = query.and_condition(Some(embedding_col.assume_not_null().eq(e)));
                }
                BlockEmbeddingMatrix::Frozen(e) => {
                    query = query.and_condition(Some(embedding_col.assume_not_null().eq(e)));
                }
            }
        }

        if let Some(fixative) = fixative {
            match fixative {
                Fixative::Block(f) => {
                    query = query.and_condition(Some(fixative_col.assume_not_null().eq(f)));
                }
                Fixative::Suspension(f) => {
                    query = query.and_condition(Some(fixative_col.assume_not_null().eq(f)));
                }
                Fixative::Tissue(f) => {
                    query = query.and_condition(Some(fixative_col.assume_not_null().eq(f)));
                }
            }
        }

        query.build()
    }
}

#[diesel::dsl::auto_type]
#[must_use]
fn summary_query_base() -> _ {
    specimen::table
}

impl AsDieselQueryBase for SpecimenSummary {
    type QueryBase = summary_query_base;

    fn as_diesel_query_base() -> Self::QueryBase {
        summary_query_base()
    }
}

impl FetchByQuery for SpecimenSummary {
    type QueryParams = SpecimenQuery;

    async fn fetch_by_query(
        query: &Self::QueryParams,
        db_conn: &mut AsyncPgConnection,
    ) -> db::error::Result<Vec<Self>> {
        use scamplers_core::model::specimen::SpecimenOrdinalColumn::{Name, ReceivedAt};
        fetch_by_query!(
            query,
            [(Name, name_col), (ReceivedAt, received_at_col)],
            db_conn
        )
    }
}
