use diesel::{PgTextExpressionMethods, RunQueryDsl, prelude::*};
use scamplers_schema::institution;

use super::{Institution, InstitutionOrderBy, InstitutionQuery};
use crate::{
    apply_eq_any_filters, apply_ilike_filters,
    db::{DbOperation, models::institution::InstitutionId},
    impl_id_db_operation, init_stmt,
    result::ScamplersResult,
};

impl DbOperation<Vec<Institution>> for InstitutionQuery {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> ScamplersResult<Vec<Institution>> {
        let mut stmt = init_stmt!(stmt = institution::table, query = &self, output = Institution; InstitutionOrderBy::Name => institution::name);

        let Self { ids, name, .. } = &self;

        stmt = apply_eq_any_filters!(stmt; institution::id, ids);
        stmt = apply_ilike_filters!(stmt; institution::name, name);

        Ok(stmt.load(db_conn)?)
    }
}

impl_id_db_operation!(InstitutionId, InstitutionQuery, Institution);
