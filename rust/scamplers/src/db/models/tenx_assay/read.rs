use diesel::prelude::*;
use scamplers_schema::tenx_assay;

use crate::{
    apply_eq_any_filters, apply_ilike_filters,
    db::{
        DbOperation,
        models::tenx_assay::{TenxAssay, TenxAssayOrderBy, TenxAssayQuery},
    },
    init_stmt,
};

#[macro_export]
macro_rules! apply_tenx_assay_query {
    ($stmt:expr, $query:expr) => {{
        for lib_type_group in &mut $query.library_types {
            lib_type_group.sort();
        }

        let crate::db::models::tenx_assay::TenxAssayQuery {
            ids,
            names,
            library_types,
            sample_multiplexing,
            chromium_chips,
            cmdlines,
            ..
        } = &$query;

        $stmt = apply_eq_any_filters!(
            $stmt,
            filters = {
                tenx_assay::id => ids,
                tenx_assay::library_types => library_types,
                tenx_assay::sample_multiplexing => sample_multiplexing,
                tenx_assay::chromium_chip => chromium_chips,
                tenx_assay::cmdline => cmdlines
            }
        );

        $stmt = apply_ilike_filters!($stmt, filters = {tenx_assay::name => names});

        $stmt
    }};
}

impl DbOperation<Vec<TenxAssay>> for TenxAssayQuery {
    fn execute(
        mut self,
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
