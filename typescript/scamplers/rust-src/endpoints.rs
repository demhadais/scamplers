use axum::{
    Json,
    extract::Path,
    http::{Method, StatusCode},
};

use crate::{
    db::models::{
        institution::{Institution, InstitutionId, InstitutionQuery, NewInstitution},
        lab::{Lab, LabId, LabQuery, LabUpdate, NewLab},
        person::{CreatedUser, NewPerson, Person, PersonId, PersonQuery, PersonUpdate},
        specimen::{NewSpecimen, Specimen, SpecimenId, SpecimenQuery},
    },
    extract::{Base64JsonQuery, ValidJsonBody},
};

pub struct Api;

pub trait Endpoint<Request, Response> {
    type RequestExtractor;
    type ResponseWrapper;

    const METHOD: Method;
    const PATH: &str;
    const SUCCESS_STATUS_CODE: StatusCode;

    fn request(client: &reqwest::Client, base_url: &str, data: Request) -> reqwest::RequestBuilder;
}

macro_rules! impl_basic_endpoints {
    (
        path = $path:expr,
        creation = $creation:ident,
        id = $id:ty,
        query = $query:ty,
        $(update = $update:ty,)?
        response = $response:ty
    ) => {
        impl Endpoint<$creation, $response> for Api {
            type RequestExtractor = ValidJsonBody<$creation>;
            type ResponseWrapper = Json<$response>;

            const METHOD: Method = Method::POST;
            const PATH: &str = $path;
            const SUCCESS_STATUS_CODE: StatusCode = StatusCode::CREATED;

            fn request(
                client: &reqwest::Client,
                base_url: &str,
                data: $creation,
            ) -> reqwest::RequestBuilder {
                let path = <Self as Endpoint<$creation, $response>>::PATH;
                client.post(format!("{base_url}{path}")).json(&data)
            }
        }

        impl Endpoint<$id, $response> for Api {
            type RequestExtractor = Path<$id>;
            type ResponseWrapper = Json<$response>;

            const METHOD: Method = Method::GET;
            const PATH: &str = concat!($path, "/", "{id}");
            const SUCCESS_STATUS_CODE: StatusCode = StatusCode::OK;

            fn request(
                client: &reqwest::Client,
                base_url: &str,
                id: $id,
            ) -> reqwest::RequestBuilder {
                let path = <Self as Endpoint<$id, $response>>::PATH;
                let path = path.replace("{id}", &id.to_string());
                client.get(format!("{base_url}{path}"))
            }
        }

        impl Endpoint<$query, Vec<$response>> for Api {
            type RequestExtractor = Base64JsonQuery<$query>;
            type ResponseWrapper = Json<Vec<$response>>;

            const METHOD: Method = Method::GET;
            const PATH: &str = $path;
            const SUCCESS_STATUS_CODE: StatusCode = StatusCode::OK;

            fn request(
                client: &reqwest::Client,
                base_url: &str,
                query: $query,
            ) -> reqwest::RequestBuilder {
                use crate::db::models::Jsonify;
                let path = <Self as Endpoint<$query, Vec<$response>>>::PATH;
                client.get(format!("{base_url}{path}?{}", query.to_base64_json()))
            }
        }

        $(impl Endpoint<$update, $response> for Api {
            type RequestExtractor = ValidJsonBody<$update>;
            type ResponseWrapper = Json<$response>;

            const METHOD: Method = Method::PATCH;
            const PATH: &str = $path;
            const SUCCESS_STATUS_CODE: StatusCode = StatusCode::OK;

            fn request(
                client: &reqwest::Client,
                base_url: &str,
                update: $update,
            ) -> reqwest::RequestBuilder {
                let path = <Self as Endpoint<$update, $response>>::PATH;
                client.patch(format!("{base_url}{path}")).json(&update)
            }
        })?
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
    update = PersonUpdate,
    response = Person
);

impl_basic_endpoints!(
    path = "/labs",
    creation = NewLab,
    id = LabId,
    query = LabQuery,
    update = LabUpdate,
    response = Lab
);

impl_basic_endpoints!(
    path = "/specimens",
    creation = NewSpecimen,
    id = SpecimenId,
    query = SpecimenQuery,
    response = Specimen
);

impl Endpoint<NewPerson, CreatedUser> for Api {
    type RequestExtractor = ValidJsonBody<NewPerson>;
    type ResponseWrapper = Json<CreatedUser>;

    const METHOD: Method = Method::POST;
    const PATH: &str = "/users";
    const SUCCESS_STATUS_CODE: StatusCode = StatusCode::CREATED;

    fn request(
        client: &reqwest::Client,
        base_url: &str,
        data: NewPerson,
    ) -> reqwest::RequestBuilder {
        client
            .post(format!(
                "{base_url}{}",
                <Self as Endpoint<NewPerson, CreatedUser>>::PATH
            ))
            .json(&data)
    }
}
