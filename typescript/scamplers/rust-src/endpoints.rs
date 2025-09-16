use axum::{
    Json,
    extract::Path,
    http::{Method, StatusCode},
};

use crate::{
    db::models::{
        chromium_run::{ChromiumRun, ChromiumRunId, ChromiumRunQuery, NewChromiumRun},
        dataset::chromium::{
            ChromiumDataset, ChromiumDatasetId, ChromiumDatasetQuery, NewChromiumDataset,
        },
        institution::{Institution, InstitutionId, InstitutionQuery, NewInstitution},
        lab::{Lab, LabId, LabQuery, LabUpdate, NewLab},
        multiplexing_tag::{MultiplexingTag, MultiplexingTagId},
        nucleic_acid::{
            cdna::{Cdna, CdnaId, CdnaQuery, NewCdnaGroup},
            library::{Library, LibraryId, LibraryQuery, NewLibrary},
        },
        person::{CreatedUser, NewPerson, Person, PersonId, PersonQuery, PersonUpdate},
        sequencing_run::{NewSequencingRun, SequencingRun, SequencingRunId, SequencingRunQuery},
        specimen::{NewSpecimen, Specimen, SpecimenId, SpecimenQuery},
        suspension::{
            pool::{NewSuspensionPool, SuspensionPool, SuspensionPoolId, SuspensionPoolQuery},
            suspension::{NewSuspension, Suspension, SuspensionId, SuspensionQuery},
        },
        tenx_assay::{NewTenxAssay, TenxAssay, TenxAssayId, TenxAssayQuery},
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
                client.get(format!("{base_url}{path}")).query(&[("query", &query.to_base64_json())])
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
                client.get(format!("{base_url}{path}")).query(&[("query", &relatives_query.to_base64_json())])
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

impl_basic_endpoints!(
    path = "/10x-assays",
    creation = NewTenxAssay,
    id = TenxAssayId,
    query = TenxAssayQuery,
    response = TenxAssay
);

impl_basic_endpoints!(
    path = "/multiplexing-tags",
    id = MultiplexingTagId,
    query = (),
    response = MultiplexingTag
);

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

// cDNA is a unique case, as it is created as a batch, so the creation returns
// `Vec<Cdna>`. However, that means the macro will have an endpoint returning
// `Vec<Vec<Cdna>>` for queries, so we implement `Endpoint` manually
const CDNA_PATH: &str = "/cdna";

impl Endpoint<NewCdnaGroup, Vec<Cdna>> for Api {
    type RequestExtractor = ValidJsonBody<NewCdnaGroup>;
    type ResponseWrapper = Json<Vec<Cdna>>;

    const METHOD: Method = Method::POST;
    const PATH: &str = CDNA_PATH;
    const SUCCESS_STATUS_CODE: StatusCode = StatusCode::CREATED;

    fn request(
        client: &reqwest::Client,
        base_url: &str,
        data: NewCdnaGroup,
    ) -> reqwest::RequestBuilder {
        let path = <Self as Endpoint<NewCdnaGroup, Vec<Cdna>>>::PATH;
        client.post(format!("{base_url}{path}")).json(&data)
    }
}

impl Endpoint<CdnaQuery, Vec<Cdna>> for Api {
    type RequestExtractor = Base64JsonQuery<CdnaQuery>;
    type ResponseWrapper = Json<Vec<Cdna>>;

    const METHOD: Method = Method::GET;
    const PATH: &str = CDNA_PATH;
    const SUCCESS_STATUS_CODE: StatusCode = StatusCode::OK;

    fn request(
        client: &reqwest::Client,
        base_url: &str,
        query: CdnaQuery,
    ) -> reqwest::RequestBuilder {
        use crate::db::models::Jsonify;
        let path = <Self as Endpoint<CdnaQuery, Vec<Cdna>>>::PATH;
        client
            .get(format!("{base_url}{path}"))
            .query(&[("query", &query.to_base64_json())])
    }
}

impl Endpoint<CdnaId, Cdna> for Api {
    type RequestExtractor = Path<CdnaId>;
    type ResponseWrapper = Json<Cdna>;

    const METHOD: Method = Method::GET;
    const PATH: &str = concat!("/cdna", "/", "{id}");
    const SUCCESS_STATUS_CODE: StatusCode = StatusCode::OK;

    fn request(client: &reqwest::Client, base_url: &str, id: CdnaId) -> reqwest::RequestBuilder {
        let path = <Self as Endpoint<CdnaId, Cdna>>::PATH;
        let path = path.replace("{id}", &id.to_string());
        client.get(format!("{base_url}{path}"))
    }
}

impl_basic_endpoints!(
    path = "/libraries",
    creation = NewLibrary,
    id = LibraryId,
    query = LibraryQuery,
    response = Library
);

impl_basic_endpoints!(
    path = "/chromium-datasets",
    creation = NewChromiumDataset,
    id = ChromiumDatasetId,
    query = ChromiumDatasetQuery,
    response = ChromiumDataset
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

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use axum::{
        Json,
        extract::Path,
        http::{Method, StatusCode},
    };
    use rstest::rstest;
    use scamplers_macros::Jsonify;

    use super::{Api, Endpoint};
    use crate::extract::{Base64JsonQuery, ValidJsonBody};
    #[derive(serde::Deserialize, serde::Serialize, Jsonify)]
    struct Creation;
    #[derive(serde::Deserialize, serde::Serialize, Jsonify)]
    struct Id;

    impl Display for Id {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            "id".fmt(f)
        }
    }

    #[derive(serde::Deserialize, serde::Serialize, Jsonify)]
    struct Query;
    #[derive(serde::Deserialize, serde::Serialize, Jsonify)]
    struct Response;

    impl_basic_endpoints!(
        path = "/test",
        creation = Creation,
        id = Id,
        query = Query,
        response = Response
    );

    #[rstest]
    #[tokio::test]
    async fn query_serialization() {
        let client = reqwest::Client::new();
        <Api as Endpoint<Query, Vec<Response>>>::request(
            &client,
            "https://postman-echo.com/get",
            Query,
        )
        .send()
        .await
        .unwrap();
    }
}
