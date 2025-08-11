use diesel::{PgTextExpressionMethods, RunQueryDsl, SelectableHelper, prelude::*};
use scamplers_schema::institution;

use super::InstitutionOrderBy;
use crate::{
    app::DbOperation,
    db::util::AsIlike,
    init_stmt,
    result::ScamplersResult,
    routes::institution::{Institution, InstitutionQuery},
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
