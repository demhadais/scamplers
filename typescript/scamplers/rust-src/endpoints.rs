use axum::{
    Json,
    extract::Path,
    http::{Method, StatusCode},
};

use crate::{
    db::models::{
        chromium_run::{ChromiumRun, ChromiumRunId, ChromiumRunQuery, NewChromiumRun},
        institution::{Institution, InstitutionId, InstitutionQuery, NewInstitution},
        lab::{Lab, LabId, LabQuery, LabUpdate, NewLab},
        multiplexing_tag::{MultiplexingTag, MultiplexingTagId},
        person::{CreatedUser, NewPerson, Person, PersonId, PersonQuery, PersonUpdate},
        sequencing_run::{NewSequencingRun, SequencingRun, SequencingRunId, SequencingRunQuery},
        specimen::{NewSpecimen, Specimen, SpecimenId, SpecimenQuery},
        suspension::{
            pool::{NewSuspensionPool, SuspensionPool, SuspensionPoolId, SuspensionPoolQuery},
            suspension::{NewSuspension, Suspension, SuspensionId, SuspensionQuery},
        },
    },
    extract::{Base64JsonQuery, RelativesQuery, ValidJsonBody},
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
        $(creation = $creation:ident,)?
        id = $id:ty,
        $(query = $query:ty,)?
        $(update = $update:ty,)?
        $(relative = {path = $relative_path:expr, query = $relative_query:ty, response = $relatives:ty},)*
        response = $response:ty
    ) => {
        $(impl Endpoint<$creation, $response> for Api {
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
        })?

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

        $(impl Endpoint<$query, Vec<$response>> for Api {
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
        })?

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

        $(impl Endpoint<($id, $relative_query), $relatives> for Api {
            type RequestExtractor = RelativesQuery<$id, $relative_query>;
            type ResponseWrapper = Json<$relatives>;

            const METHOD: Method = Method::GET;
            const PATH: &str = concat!($path, "/", "{id}", $relative_path);
            const SUCCESS_STATUS_CODE: StatusCode = StatusCode::OK;

            fn request(
                client: &reqwest::Client,
                base_url: &str,
                (id, relatives_query): ($id, $relative_query),
            ) -> reqwest::RequestBuilder {
                use crate::db::models::Jsonify;
                let path = <Self as Endpoint<($id, $relative_query), $relatives>>::PATH;
                let path = path.replace("{id}", &id.to_string());
                client.get(format!("{base_url}{path}?{}", relatives_query.to_base64_json()))
            }
        })*
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
    relative = {path = "/specimens", query = SpecimenQuery, response = Vec<Specimen>},
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

impl_basic_endpoints!(
    path = "/sequencing-runs",
    creation = NewSequencingRun,
    id = SequencingRunId,
    query = SequencingRunQuery,
    response = SequencingRun
);

impl_basic_endpoints! {
    path = "/multiplexing-tags",
    id = MultiplexingTagId,
    query = (),
    response = MultiplexingTag
}

impl_basic_endpoints! {
    path = "/suspensions",
    creation = NewSuspension,
    id = SuspensionId,
    query = SuspensionQuery,
    response = Suspension
}

impl_basic_endpoints! {
    path = "/suspension-pools",
    creation = NewSuspensionPool,
    id = SuspensionPoolId,
    query = SuspensionPoolQuery,
    response = SuspensionPool
}

impl_basic_endpoints! {
    path = "/chromium-runs",
    creation = NewChromiumRun,
    id = ChromiumRunId,
    query = ChromiumRunQuery,
    response = ChromiumRun
}

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
