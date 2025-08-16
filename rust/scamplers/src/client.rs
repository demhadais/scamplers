#[cfg(feature = "python")]
use std::sync::Arc;

#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::de::DeserializeOwned;
#[cfg(feature = "python")]
use tokio::runtime::Runtime;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "python")]
use crate::db::models::institution::NewInstitution;
#[cfg(target_arch = "wasm32")]
use crate::db::models::person::CreatedUser;
use crate::{
    db::models::{
        institution::{Institution, InstitutionId, InstitutionQuery},
        lab::{Lab, LabId, LabQuery, LabUpdate, NewLab},
        person::{NewPerson, Person, PersonId, PersonQuery, PersonUpdate},
    },
    endpoints::{Api, Endpoint},
    result::{ScamplersErrorResponse, ServerError},
};

#[allow(dead_code)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg_attr(feature = "python", pyclass)]
#[derive(Clone)]
pub struct ScamplersClient {
    backend_base_url: String,
    client: reqwest::Client,
    api_key: Option<String>,
    #[cfg(feature = "python")]
    runtime: Arc<Runtime>,
}

#[cfg(feature = "python")]
#[pymethods]
impl ScamplersClient {
    #[new]
    #[pyo3(signature = (*, api_base_url, api_key=None, accept_invalid_certificates=None))]
    fn py_new(
        api_base_url: String,
        api_key: Option<String>,
        accept_invalid_certificates: Option<bool>,
    ) -> Self {
        Self::new(api_base_url, None, api_key, accept_invalid_certificates)
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl ScamplersClient {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    #[must_use]
    pub fn new(
        api_base_url: String,
        frontend_token: Option<String>,
        api_key: Option<String>,
        #[allow(unused_variables)] accept_invalid_certificates: Option<bool>,
    ) -> Self {
        use reqwest::{
            ClientBuilder,
            header::{AUTHORIZATION, HeaderMap, HeaderValue},
        };

        let token = frontend_token.unwrap_or_default();

        let mut auth = HeaderValue::from_str(&format!("Bearer {token}")).unwrap();
        auth.set_sensitive(true);

        let headers = HeaderMap::from_iter([(AUTHORIZATION, auth)]);

        #[allow(unused_mut)]
        let mut client = ClientBuilder::new().default_headers(headers);

        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(accept_invalid_certs) = accept_invalid_certificates {
                client = client.danger_accept_invalid_certs(accept_invalid_certs);
            }
        }

        let client = client.build().unwrap();

        #[cfg(not(feature = "python"))]
        return Self {
            backend_base_url: api_base_url,
            client,
            api_key,
        };

        #[cfg(feature = "python")]
        return Self {
            backend_base_url: api_base_url,
            client,
            api_key,
            runtime: Arc::new(Runtime::new().unwrap()),
        };
    }
}

impl ScamplersClient {
    #[allow(dead_code)]
    async fn send_request<Req, Resp>(&self, data: Req) -> Result<Resp, ScamplersErrorResponse>
    where
        Api: Endpoint<Req, Resp>,
        Resp: DeserializeOwned,
    {
        let Self {
            backend_base_url,
            client,
            api_key,
            ..
        } = self;

        let mut request = Api::request(client, backend_base_url, data);

        if let Some(api_key) = api_key {
            request = request.header("X-API-Key", api_key);
        }

        let response = request.send().await.unwrap();
        let status = response.status().as_u16();
        let raw_response = response.bytes().await.unwrap();

        let deserialized_success_response = serde_json::from_slice(&raw_response);

        let Err(deserialization_failure1) = deserialized_success_response else {
            return Ok(deserialized_success_response.unwrap());
        };

        let deserialized_failure_response = serde_json::from_slice(&raw_response);

        let Err(deserialization_failure2) = deserialized_failure_response else {
            return Err(deserialized_failure_response.unwrap());
        };

        let error = ServerError {
            message: format!(
                "failed to deserialize response body as success and as failure: \
                 {deserialization_failure1} / {deserialization_failure2}"
            ),
            raw_response_body: String::from_utf8(raw_response.to_vec()).unwrap(),
        };

        Err(ScamplersErrorResponse::builder()
            .status(status)
            .error(error)
            .build())
    }

    #[cfg(target_arch = "wasm32")]
    async fn send_request_wasm<Req, Resp>(&self, data: Req) -> Result<Resp, ScamplersErrorResponse>
    where
        Api: Endpoint<Req, Resp>,
        Resp: DeserializeOwned,
    {
        self.send_request(data).await
    }

    #[cfg(feature = "python")]
    async fn send_request_python<Req, Resp>(self, data: Req) -> Result<Resp, ScamplersErrorResponse>
    where
        Api: Endpoint<Req, Resp>,
        Req: Send + 'static,
        Resp: DeserializeOwned + Send + 'static,
    {
        let runtime = self.runtime.clone();

        runtime
            .spawn(async move { self.send_request(data).await })
            .await
            .unwrap()
    }
}

macro_rules! impl_wasm_client_methods {
    ($(($method_name:ident, $request_type:path, $response_type:path));*) => {
        $(
            #[cfg(target_arch = "wasm32")]
            #[wasm_bindgen::prelude::wasm_bindgen]
            impl ScamplersClient {
                async fn $method_name(
                    &self,
                    data: $request_type,
                ) -> Result<$response_type, ScamplersErrorResponse> {
                    self.send_request_wasm(data).await
                }
            }
        )*
    };
}

macro_rules! impl_python_client_methods {
    ($(($method_name:ident, $request_type:path, $response_type:path));*) => {
        $(
            #[cfg(feature = "python")]
            #[pyo3::pymethods]
            impl ScamplersClient {
                async fn $method_name(
                    &self,
                    data: $request_type,
                ) -> Result<$response_type, ScamplersErrorResponse> {
                    let client = self.clone();
                    client.send_request_python(data).await
                }
            }
        )*
    };
}

impl_wasm_client_methods!(
    (fetch_institution, InstitutionId, Institution);
    (list_institutions, InstitutionQuery, Vec<Institution>);
    (ms_login, NewPerson, CreatedUser);
    (fetch_person, PersonId, Person);
    (list_people, PersonQuery, Vec<Person>);
    (get_lab, LabId, Lab);
    (list_labs, LabQuery, Vec<Lab>)
);

impl_python_client_methods!(
    (create_institution, NewInstitution, Institution);
    (fetch_institution, InstitutionId, Institution);
    (list_institutions, InstitutionQuery, Vec<Institution>);
    (create_person, NewPerson, Person);
    (fetch_person, PersonId, Person);
    (list_people, PersonQuery, Vec<Person>);
    (update_person, PersonUpdate, Person);
    (create_lab, NewLab, Lab);
    (get_lab, LabId, Lab);
    (list_labs, LabQuery, Vec<Lab>);
    (update_lab, LabUpdate, Lab)
);

// #[cfg(feature = "python")]
// macro_rules! impl_chromium_dataset_creation {
//     ($(($method_name:ident, $request_type:path));*) => {
//         $(#[pymethods]
//         impl ScamplersClient {
//             async fn $method_name(
//                 &self,
//                 data: $request_type,
//             ) -> Result<DatasetSummary, ScamplersCoreErrorResponse> {
//                 let client = self.clone();
//                 client.send_request_python(NewDataset::from(data),
// Method::POST).await             }
//         })*
//     };
// }

// #[cfg(feature = "python")]
// impl_chromium_dataset_creation!(
//     (create_cellrangerarc_count_dataset,CellrangerarcCountDataset);
//     (create_cellrangeratac_count_dataset, CellrangeratacCountDataset);
//     (create_cellranger_count_dataset, CellrangerCountDataset);
//     (create_cellranger_multi_dataset, CellrangerMultiDataset);
//     (create_cellranger_vdj_dataset, CellrangerVdjDataset)
// );
