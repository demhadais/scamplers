use axum::http::{Method, StatusCode};
use diesel::prelude::*;
use scamplers_schema::institution;

use crate::{
    app::{ApiEndpoint, DbOperation, extract::ValidJsonBody},
    result::ScamplersResult,
    routes::institution::{Institution, NewInstitution},
};

impl DbOperation<Institution> for NewInstitution {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> ScamplersResult<Institution> {
        Ok(diesel::insert_into(institution::table)
            .values(self)
            .returning(Institution::as_returning())
            .get_result(db_conn)?)
    }
}

impl ApiEndpoint for (NewInstitution, Institution) {
    type RequestExtractor = ValidJsonBody<NewInstitution>;
    type ResponseWrapper = ValidJsonBody<Institution>;

    const METHOD: Method = Method::POST;
    const PATH: &str = "/institutions";
    const SUCCESS_STATUS_CODE: StatusCode = StatusCode::CREATED;
}
