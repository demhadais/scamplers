use crate::db::{
    model::{AsDieselFilter, Write},
    util::{AsIlike, BoxedDieselExpression, NewBoxedDieselExpression},
};
use diesel::{dsl::AssumeNotNull, prelude::*};
use diesel_async::RunQueryDsl;
use sample_metadata::dsl::id as id_col;
use scamplers_core::model::sample_metadata::{
    NewSampleMetadata, SampleMetadata, SampleMetadataQuery,
};
use scamplers_schema::{
    committee_approval, institution, lab, person,
    sample_metadata::{
        self, name as name_col, notes as notes_col, received_at as received_at_col,
        species as species_col,
    },
};

#[diesel::dsl::auto_type]
#[must_use]
pub fn summary_query_base() -> _ {
    sample_metadata::table
}

diesel::alias!(person as returned_by: ReturnedByAlias);

#[diesel::dsl::auto_type]
#[must_use]
pub fn query_base() -> _ {
    let submitter_join_condition = sample_metadata::submitted_by.eq(person::id);
    let returner_join_condition =
        sample_metadata::returned_by.eq(returned_by.field(person::id).nullable());

    summary_query_base()
        .inner_join(person::table.on(submitter_join_condition))
        .left_join(returned_by.on(returner_join_condition))
        .inner_join(lab::table)
}

#[diesel::dsl::auto_type]
#[must_use]
pub fn committee_approval_query_base() -> _ {
    committee_approval::table.inner_join(institution::table)
}

impl Write for NewSampleMetadata {
    type Returns = SampleMetadata;

    async fn write(
        mut self,
        db_conn: &mut diesel_async::AsyncPgConnection,
    ) -> crate::db::error::Result<Self::Returns> {
        let id = diesel::insert_into(sample_metadata::table)
            .values(&self)
            .returning(id_col)
            .get_result(db_conn)
            .await?;

        let committee_approvals = self.committee_approvals(id);
        diesel::insert_into(committee_approval::table)
            .values(committee_approvals)
            .execute(db_conn)
            .await?;

        Ok(query_base()
            .filter(id_col.eq(id))
            .select(SampleMetadata::as_select())
            .first(db_conn)
            .await?)
    }
}

impl<QueryExpression> AsDieselFilter<QueryExpression> for SampleMetadataQuery
where
    name_col: SelectableExpression<QueryExpression>,
    received_at_col: SelectableExpression<QueryExpression>,
    species_col: SelectableExpression<QueryExpression>,
    AssumeNotNull<notes_col>: SelectableExpression<QueryExpression>,
{
    fn as_diesel_filter<'a>(&'a self) -> Option<BoxedDieselExpression<'a, QueryExpression>>
    where
        QueryExpression: 'a,
    {
        let Self {
            name,
            received_before,
            received_after,
            species,
            notes,
        } = self;

        let q1 = name.as_ref().map(|name| name_col.ilike(name.as_ilike()));
        let q2 = received_before
            .as_ref()
            .map(|received_before| received_at_col.lt(received_before));
        let q3 = received_after
            .as_ref()
            .map(|received_after| received_at_col.gt(received_after));
        let q4 = (!species.is_empty()).then(|| species_col.overlaps_with(species));
        let q5 = notes
            .as_ref()
            .map(|notes| notes_col.assume_not_null().ilike(notes.as_ilike()));

        BoxedDieselExpression::new_expression()
            .and_condition(q1)
            .and_condition(q2)
            .and_condition(q3)
            .and_condition(q4)
            .and_condition(q5)
            .build()
    }
}
