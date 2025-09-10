use diesel::prelude::*;
use scamplers_schema::{library_type_specification, tenx_assay};
use uuid::Uuid;

use crate::{
    db::{
        DbOperation,
        models::tenx_assay::{
            TenxAssay, TenxAssayOrderBy, TenxAssayQuery, chromium::LibraryTypeSpecification,
        },
    },
    init_stmt,
};

#[macro_export]
macro_rules! apply_tenx_assay_query {
    ($stmt:expr, $query:expr) => {{
        $stmt = $crate::apply_eq_any_filters!(
            $stmt,
            filters = {
                scamplers_schema::tenx_assay::id => $query.ids,
                scamplers_schema::tenx_assay::sample_multiplexing => $query.sample_multiplexing,
                scamplers_schema::tenx_assay::chromium_chip => $query.chromium_chips
            }
        );

        $stmt = $crate::apply_ilike_filters!(
            $stmt,
            filters = {
                scamplers_schema::tenx_assay::name => &$query.names,
                scamplers_schema::tenx_assay::chemistry_version => &$query.chemistry_versions
            }
        );

        for mut lib_type_group in $query.library_types {
            lib_type_group.sort();
            $stmt = $stmt.or_filter(scamplers_schema::tenx_assay::library_types.eq(lib_type_group));
        }

        $stmt
    }};
}

impl DbOperation<Vec<TenxAssay>> for TenxAssayQuery {
    fn execute(
        self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<Vec<TenxAssay>> {
        let mut stmt = init_stmt!(
            stmt = tenx_assay::table,
            query = self, output_type = TenxAssay,
            orderby_spec = { TenxAssayOrderBy::Name => tenx_assay::name }
        );

        stmt = apply_tenx_assay_query!(stmt, self);

        Ok(stmt.load(db_conn)?)
    }
}

impl LibraryTypeSpecification {
    // This is a temporary function which will eventually be moved into the
    // `TenxAssayQuery`
    pub fn list_by_assay_id(
        assay_id: Uuid,
        db_conn: &mut PgConnection,
    ) -> crate::result::ScamplersResult<Vec<Self>> {
        Ok(library_type_specification::table
            .filter(library_type_specification::assay_id.eq(assay_id))
            .order_by(library_type_specification::library_type)
            .select(Self::as_select())
            .load(db_conn)?)
    }
}
