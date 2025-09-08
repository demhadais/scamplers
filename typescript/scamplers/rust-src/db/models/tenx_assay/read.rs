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
        $stmt = apply_eq_any_filters!(
            $stmt,
            filters = {
                tenx_assay::id => $query.ids,
                tenx_assay::sample_multiplexing => $query.sample_multiplexing,
                tenx_assay::chromium_chip => $query.chromium_chips,
                tenx_assay::cmdline => $query.cmdlines
            }
        );

        $stmt = apply_ilike_filters!(
            $stmt,
            filters = {
                tenx_assay::name => &$query.names,
                tenx_assay::chemistry_version => &$query.chemistry_versions
            }
        );

        for mut lib_type_group in $query.library_types {
            lib_type_group.sort();
            $stmt = $stmt.or_filter(tenx_assay::library_types.eq(lib_type_group));
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
