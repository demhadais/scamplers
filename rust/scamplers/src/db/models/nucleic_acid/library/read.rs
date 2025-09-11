use diesel::prelude::*;
use scamplers_schema::{cdna, library};

use crate::{
    apply_eq_any_filters, attach_children_to_parents,
    db::{
        DbOperation,
        models::nucleic_acid::library::{
            Library, LibraryId, LibraryMeasurement, LibraryOrderBy, LibraryPreparer, LibraryQuery,
            LibrarySummaryWithParents,
        },
    },
    group_children, group_preparers, impl_id_db_operation, init_stmt,
};

impl DbOperation<Vec<Library>> for LibraryQuery {
    fn execute(
        self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<Vec<Library>> {
        let base_stmt = library::table.inner_join(cdna::table);

        let mut stmt = init_stmt!(stmt = base_stmt, query = &self, output_type = LibrarySummaryWithParents, orderby_spec = {LibraryOrderBy::PreparedAt => library::prepared_at, LibraryOrderBy::ReadableId => library::readable_id});

        let Self {
            ids, library_types, ..
        } = &self;

        stmt = apply_eq_any_filters!(
            stmt,
            filters = {
                library::id => ids,
                cdna::library_type => library_types
            }
        );

        let summaries_with_parents = stmt.load(db_conn)?;

        let preparers = LibraryPreparer::belonging_to(&summaries_with_parents)
            .select(LibraryPreparer::as_select())
            .load(db_conn)?;
        let grouped_preparers =
            group_preparers!(parents = summaries_with_parents, children = preparers);

        let measurements = LibraryMeasurement::belonging_to(&summaries_with_parents)
            .select(LibraryMeasurement::as_select())
            .load(db_conn)?;
        let grouped_measurements =
            group_children!(parents = summaries_with_parents, children = measurements);

        let libraries = attach_children_to_parents!(
            parents = summaries_with_parents,
            children = [grouped_preparers, grouped_measurements],
            transform_fn = |((info, prepared_by), measurements)| Library {
                info,
                prepared_by,
                measurements
            }
        );

        Ok(libraries)
    }
}

impl_id_db_operation!(
    id_type = LibraryId,
    delegate_to = LibraryQuery,
    returns = Library
);
