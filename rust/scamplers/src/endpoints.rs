use axum::{
    Json,
    extract::Path,
    http::{Method, StatusCode},
};

use crate::{
    db::models::{
        institution::{Institution, InstitutionId, InstitutionQuery, NewInstitution},
        person::{CreatedUser, NewPerson, Person, PersonId, PersonQuery},
    },
    extract::{Base64JsonQuery, ValidJsonBody},
};

pub struct Api;

pub trait Endpoint<Request, Response> {
    const METHOD: Method;
    const PATH: &str;
    const SUCCESS_STATUS_CODE: StatusCode;
    type RequestExtractor;
    type ResponseWrapper;

    fn request(client: &reqwest::Client, base_url: &str, data: Request) -> reqwest::RequestBuilder;
}

macro_rules! impl_basic_endpoints {
    (
        path = $path:expr,
        creation = $creation:ident,
        id = $id:ty,
        query = $query:ty,
        response = $response:ty
    ) => {
        impl Endpoint<$creation, $response> for Api {
            const METHOD: Method = Method::POST;
            const PATH: &str = $path;
            const SUCCESS_STATUS_CODE: StatusCode = StatusCode::CREATED;
            type RequestExtractor = ValidJsonBody<$creation>;
            type ResponseWrapper = Json<$response>;

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
            const METHOD: Method = Method::GET;
            const PATH: &str = concat!($path, "/", "{id}");
            const SUCCESS_STATUS_CODE: StatusCode = StatusCode::OK;
            type RequestExtractor = Path<$id>;
            type ResponseWrapper = Json<$response>;

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
            const METHOD: Method = Method::GET;
            const PATH: &str = $path;
            const SUCCESS_STATUS_CODE: StatusCode = StatusCode::OK;
            type RequestExtractor = Base64JsonQuery<$query>;
            type ResponseWrapper = Json<Vec<$response>>;

            fn request(
                client: &reqwest::Client,
                base_url: &str,
                query: $query,
            ) -> reqwest::RequestBuilder {
                let path = <Self as Endpoint<$query, Vec<$response>>>::PATH;
                client
                    .get(format!("{base_url}{path}"))
                    .query(&Base64JsonQuery(query))
            }
        }

        impl Endpoint<$id, ()> for Api {
            const METHOD: Method = Method::DELETE;
            const PATH: &str = $path;
            const SUCCESS_STATUS_CODE: StatusCode = StatusCode::OK;
            type RequestExtractor = Path<$query>;
            type ResponseWrapper = ();

            fn request(
                client: &reqwest::Client,
                base_url: &str,
                id: $id,
            ) -> reqwest::RequestBuilder {
                let path = <Self as Endpoint<$id, ()>>::PATH;
                client.delete(format!("{base_url}{path}/{id}"))
            }
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

impl Endpoint<NewPerson, CreatedUser> for Api {
    const METHOD: Method = Method::POST;
    const PATH: &str = "/users";
    const SUCCESS_STATUS_CODE: StatusCode = StatusCode::CREATED;
    type RequestExtractor = ValidJsonBody<NewPerson>;
    type ResponseWrapper = Json<CreatedUser>;

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
