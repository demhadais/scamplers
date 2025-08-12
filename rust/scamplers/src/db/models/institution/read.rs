use diesel::{PgTextExpressionMethods, RunQueryDsl, prelude::*};
use scamplers_schema::institution;

use super::{Institution, InstitutionOrderBy, InstitutionQuery};
use crate::{
    db::{DbOperation, models::institution::InstitutionId, util::AsIlike},
    init_stmt,
    result::{ScamplersErrorResponse, ScamplersResult},
};

impl DbOperation<Vec<Institution>> for InstitutionQuery {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> ScamplersResult<Vec<Institution>> {
        let mut stmt = init_stmt!(stmt = institution::table, query = &self, output = Institution; InstitutionOrderBy::Name => institution::name);

        let Self { ids, name, .. } = &self;

        if !ids.is_empty() {
            stmt = stmt.filter(institution::id.eq_any(ids));
        }

        if let Some(name) = name {
            stmt = stmt.filter(institution::name.ilike(name.as_ilike()));
        }

        Ok(stmt.load(db_conn)?)
    }
}

impl DbOperation<Institution> for InstitutionId {
    fn execute(self, db_conn: &mut PgConnection) -> ScamplersResult<Institution> {
        let mut res = InstitutionQuery::builder()
            .ids(self)
            .build()
            .execute(db_conn)?;

        if res.len() > 0 {
            return Ok(res.remove(0));
        }

        todo!()
    }
}
