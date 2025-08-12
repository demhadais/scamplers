use axum::{
    Json,
    extract::Path,
    http::{Method, StatusCode},
};

use crate::{
    db::models::institution::{Institution, InstitutionId, InstitutionQuery, NewInstitution},
    server::extract::{Base64JsonQuery, ValidJsonBody},
};

pub trait ApiEndpoint {
    const METHOD: Method;
    const PATH: &str;
    const SUCCESS_STATUS_CODE: StatusCode;
}

impl ApiEndpoint for (ValidJsonBody<NewInstitution>, Json<Institution>) {
    const METHOD: Method = Method::POST;
    const PATH: &str = "/institutions";
    const SUCCESS_STATUS_CODE: StatusCode = StatusCode::CREATED;
}

impl ApiEndpoint for (Path<InstitutionId>, Json<Institution>) {
    const METHOD: Method = Method::GET;
    const PATH: &str = "/institutions/{id}";
    const SUCCESS_STATUS_CODE: StatusCode = StatusCode::OK;
}

impl ApiEndpoint for (Base64JsonQuery<InstitutionQuery>, Json<Vec<Institution>>) {
    const METHOD: Method = Method::GET;
    const PATH: &str = "/institutions";
    const SUCCESS_STATUS_CODE: StatusCode = StatusCode::OK;
}
