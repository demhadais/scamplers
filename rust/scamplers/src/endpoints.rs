use axum::{
    Json,
    extract::Path,
    http::{Method, StatusCode},
};

use crate::{
    db::models::{
        institution::{Institution, InstitutionId, InstitutionQuery, NewInstitution},
        person::{NewPerson, Person},
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

impl ApiEndpoint for (NewInstitution, Institution) {
    const METHOD: Method = Method::POST;
    const PATH: &str = "/institutions";
    const SUCCESS_STATUS_CODE: StatusCode = StatusCode::CREATED;
    type RequestExtractor = ValidJsonBody<NewInstitution>;
    type ResponseWrapper = Json<Institution>;
}

impl ApiEndpoint for (InstitutionId, Institution) {
    const METHOD: Method = Method::GET;
    const PATH: &str = "/institutions/{id}";
    const SUCCESS_STATUS_CODE: StatusCode = StatusCode::OK;
    type RequestExtractor = Path<InstitutionId>;
    type ResponseWrapper = Json<Institution>;
}

impl ApiEndpoint for (InstitutionQuery, Vec<Institution>) {
    const METHOD: Method = Method::GET;
    const PATH: &str = "/institutions";
    const SUCCESS_STATUS_CODE: StatusCode = StatusCode::OK;
    type RequestExtractor = Base64JsonQuery<InstitutionQuery>;
    type ResponseWrapper = Json<Institution>;
}

impl ApiEndpoint for (NewPerson, Person) {
    const METHOD: Method = Method::POST;
    const PATH: &str = "/people";
    const SUCCESS_STATUS_CODE: StatusCode = StatusCode::CREATED;
    type RequestExtractor = ValidJsonBody<NewPerson>;
    type ResponseWrapper = Json<Person>;
}
