use axum::http::{Method, StatusCode};

use crate::result::ScamplersResult;

pub mod extract;

pub trait DbOperation<Output>: Sized {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> ScamplersResult<Output>;
}

pub trait ApiEndpoint {
    const PATH: &str;
    const METHOD: Method;
    const SUCCESS_STATUS_CODE: StatusCode;
    type RequestExtractor;
    type ResponseWrapper;
}
