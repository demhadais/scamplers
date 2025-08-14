use axum::{
    Json,
    extract::Path,
    http::{Method, StatusCode},
};
use serde::Serialize;

use crate::{
    db::models::{
        Jsonify,
        institution::{Institution, InstitutionId, InstitutionQuery, NewInstitution},
        person::{NewPerson, Person, PersonId, PersonQuery},
    },
    extract::{Base64JsonQuery, ValidJsonBody},
};

pub trait ApiEndpoint {
    const METHOD: Method;
    const PATH: &str;
    const SUCCESS_STATUS_CODE: StatusCode;
    type RequestExtractor;
    type ResponseWrapper;
}

macro_rules! impl_basic_endpoints {
    (path = $path:expr, creation = $creation:ty, id = $id:ty, query = $query:ty, response = $response:ty) => {
        impl ApiEndpoint for ($creation, $response) {
            const METHOD: Method = Method::POST;
            const PATH: &str = $path;
            const SUCCESS_STATUS_CODE: StatusCode = StatusCode::CREATED;
            type RequestExtractor = ValidJsonBody<$creation>;
            type ResponseWrapper = Json<$response>;
        }

        impl ApiEndpoint for ($id, $response) {
            const METHOD: Method = Method::GET;
            const PATH: &str = concat!($path, "/", "{id}");
            const SUCCESS_STATUS_CODE: StatusCode = StatusCode::OK;
            type RequestExtractor = Path<$id>;
            type ResponseWrapper = Json<$response>;
        }

        impl ApiEndpoint for ($query, Vec<$response>) {
            const METHOD: Method = Method::GET;
            const PATH: &str = $path;
            const SUCCESS_STATUS_CODE: StatusCode = StatusCode::OK;
            type RequestExtractor = Base64JsonQuery<$query>;
            type ResponseWrapper = Json<Vec<$response>>;
        }

        impl ApiEndpoint for ($id, ()) {
            const METHOD: Method = Method::DELETE;
            const PATH: &str = concat!($path, "/", "{id}");
            const SUCCESS_STATUS_CODE: StatusCode = StatusCode::NO_CONTENT;
            type RequestExtractor = Path<$id>;
            type ResponseWrapper = Json<()>;
        }
    };
}

impl_basic_endpoints!(
    path = "/institutions",
    creation = NewInstitution,
    id = InstitutionId,
    query = InstitutionQuery,
    response = Institution
);

impl_basic_endpoints!(
    path = "/people",
    creation = NewPerson,
    id = PersonId,
    query = PersonQuery,
    response = Person
);
