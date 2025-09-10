#[cfg(feature = "python")]
use std::sync::Arc;

#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::de::DeserializeOwned;
#[cfg(feature = "python")]
use tokio::runtime::Runtime;
#[cfg(target_arch = "wasm32")]
use uuid::Uuid;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::db::models::person::CreatedUser;
#[cfg(feature = "python")]
use crate::db::models::{
    chromium_run::NewChromiumRun,
    dataset::chromium::NewChromiumDataset,
    institution::NewInstitution,
    multiplexing_tag::MultiplexingTag,
    nucleic_acid::{cdna::NewCdnaGroup, library::NewLibrary},
    person::PersonUpdate,
    sequencing_run::NewSequencingRun,
    specimen::NewSpecimen,
    suspension::{pool::NewSuspensionPool, suspension::NewSuspension},
};
#[cfg(any(target_arch = "wasm32", feature = "python"))]
use crate::db::models::{
    chromium_run::{ChromiumRun, ChromiumRunId, ChromiumRunQuery},
    dataset::chromium::{ChromiumDataset, ChromiumDatasetId, ChromiumDatasetQuery},
    institution::{Institution, InstitutionId, InstitutionQuery},
    lab::{Lab, LabId, LabQuery, LabUpdate, NewLab},
    nucleic_acid::{
        cdna::{Cdna, CdnaId, CdnaQuery},
        library::{Library, LibraryId, LibraryQuery},
    },
    person::{NewPerson, Person, PersonId, PersonQuery},
    sequencing_run::{SequencingRun, SequencingRunId, SequencingRunQuery},
    specimen::{Specimen, SpecimenId, SpecimenQuery},
    suspension::{
        pool::{SuspensionPool, SuspensionPoolId, SuspensionPoolQuery},
        suspension::{Suspension, SuspensionId, SuspensionQuery},
    },
};
use crate::{
    endpoints::{Api, Endpoint},
    result::{ScamplersErrorResponse, ServerError},
};

#[allow(dead_code)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[cfg_attr(feature = "python", pyclass)]
#[derive(Clone)]
pub struct ScamplersClient {
    backend_base_url: String,
    client: reqwest::Client,
    api_key: Option<String>,
    #[cfg(feature = "python")]
    runtime: Arc<Runtime>,
}

impl ScamplersClient {
    #[must_use]
    pub fn new(
        api_base_url: String,
        frontend_token: Option<String>,
        api_key: Option<String>,
        #[allow(unused_variables)] accept_invalid_certificates: bool,
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
            client = client.danger_accept_invalid_certs(accept_invalid_certificates);
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

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl ScamplersClient {
    #[new]
    #[pyo3(signature = (*, api_base_url, api_key=None, accept_invalid_certificates=false))]
    fn py_new(
        api_base_url: String,
        api_key: Option<String>,
        accept_invalid_certificates: bool,
    ) -> Self {
        Self::new(api_base_url, None, api_key, accept_invalid_certificates)
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl ScamplersClient {
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn js_new(
        api_base_url: String,
        frontend_token: Option<String>,
        api_key: Option<String>,
    ) -> Self {
        Self::new(api_base_url, frontend_token, api_key, false)
    }
}

impl ScamplersClient {
    pub async fn send_request<Req, Resp>(&self, data: Req) -> Result<Resp, ScamplersErrorResponse>
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

#[cfg(target_arch = "wasm32")]
macro_rules! wasm_client_methods {
    {$($method_name:ident($request_type:path) -> $response_type:path);*} => {
        $(
            #[cfg(target_arch = "wasm32")]
            #[wasm_bindgen::prelude::wasm_bindgen]
            impl ScamplersClient {
                pub async fn $method_name(
                    &self,
                    data: $request_type,
                ) -> Result<$response_type, ScamplersErrorResponse> {
                    self.send_request_wasm(data).await
                }
            }
        )*
    };
}

#[cfg(target_arch = "wasm32")]
macro_rules! wasm_wrapped_data_methods {
    {$($method_name:ident($wrapper:ident($request_type:path)) -> $response_type:path);*} => {
        $(
            #[wasm_bindgen::prelude::wasm_bindgen]
            impl ScamplersClient {
                pub async fn $method_name(
                    &self,
                    data: $request_type,
                ) -> Result<$response_type, ScamplersErrorResponse> {
                    self.send_request_wasm($wrapper(data)).await
                }
            }
        )*
    };
}

#[cfg(target_arch = "wasm32")]
macro_rules! wasm_list_relatives_methods {
    {$($method_name:ident($wrapper:ident($id_type:path), $query_type:ty) -> $response_type:path);*} => {
        $(
            #[wasm_bindgen::prelude::wasm_bindgen]
            impl ScamplersClient {
                pub async fn $method_name(
                    &self,
                    id: $id_type,
                    query: $query_type,
                ) -> Result<$response_type, ScamplersErrorResponse> {
                    self.send_request_wasm(($wrapper(id), query)).await
                }
            }
        )*
    };
}

#[cfg(feature = "python")]
macro_rules! python_client_methods {
    {$($method_name:ident($request_type:ty) -> $response_type:path);*} => {
        $(
            #[pyo3_stub_gen::derive::gen_stub_pymethods]
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

#[cfg(feature = "python")]
macro_rules! python_client_list_relatives_methods {
    {$($method_name:ident($id_type:ty, $query_type:ty) -> $response_type:path;)*} => {
        $(
            #[pyo3_stub_gen::derive::gen_stub_pymethods]
            #[pyo3::pymethods]
            impl ScamplersClient {
                #[pyo3(signature = (id, query = $query_type::default()))]
                async fn $method_name(
                    &self,
                    id: $id_type,
                    query: $query_type,
                ) -> Result<$response_type, ScamplersErrorResponse> {
                    let client = self.clone();
                    client.send_request_python((id, query)).await
                }
            }
        )*
    };
}

// We put the #[cfg(...)] here so that auto-imports are automatically gated
#[cfg(target_arch = "wasm32")]
wasm_client_methods! {
    list_institutions(InstitutionQuery) -> Vec<Institution>;
    ms_login(NewPerson) -> CreatedUser;
    list_people(PersonQuery) -> Vec<Person>;
    create_lab(NewLab) -> Lab;
    list_labs(LabQuery) -> Vec<Lab>;
    update_lab(LabUpdate) -> Lab;
    list_specimens(SpecimenQuery) -> Vec<Specimen>;
    list_sequencing_runs(SequencingRunQuery) -> Vec<SequencingRun>;
    list_suspensions(SuspensionQuery) -> Vec<Suspension>;
    list_suspension_pools(SuspensionPoolQuery) -> Vec<SuspensionPool>;
    list_chromium_runs(ChromiumRunQuery) -> Vec<ChromiumRun>;
    list_cdna(CdnaQuery) -> Vec<Cdna>;
    list_libraries(LibraryQuery) -> Vec<Library>;
    list_chromium_datasets(ChromiumDatasetQuery) -> Vec<ChromiumDataset>
}

#[cfg(target_arch = "wasm32")]
wasm_wrapped_data_methods! {
    fetch_institution(InstitutionId(Uuid)) -> Institution;
    fetch_person(PersonId(Uuid)) -> Person;
    fetch_lab(LabId(Uuid)) -> Lab;
    fetch_specimen(SpecimenId(Uuid)) -> Specimen;
    fetch_sequencing_run(SequencingRunId(Uuid)) -> SequencingRun;
    fetch_suspension(SuspensionId(Uuid)) -> Suspension;
    fetch_suspension_pool(SuspensionPoolId(Uuid)) -> SuspensionPool;
    fetch_chromium_run(ChromiumRunId(Uuid)) -> ChromiumRun;
    fetch_cdna(CdnaId(Uuid)) -> Cdna;
    fetch_library(LibraryId(Uuid)) -> Library;
    fetch_chromium_dataset(ChromiumDatasetId(Uuid)) -> ChromiumDataset
}

#[cfg(target_arch = "wasm32")]
wasm_list_relatives_methods! {
    list_person_specimens(PersonId(Uuid), SpecimenQuery) -> Vec<Specimen>
}

#[cfg(feature = "python")]
python_client_methods! {
    create_institution(NewInstitution) -> Institution;
    fetch_institution(InstitutionId) -> Institution;
    list_institutions(InstitutionQuery) -> Vec<Institution>;
    create_person(NewPerson) -> Person;
    fetch_person(PersonId) -> Person;
    list_people(PersonQuery) -> Vec<Person>;
    update_person(PersonUpdate) -> Person;
    create_lab(NewLab) -> Lab;
    fetch_lab(LabId) -> Lab;
    list_labs(LabQuery) -> Vec<Lab>;
    update_lab(LabUpdate) -> Lab;
    create_specimen(NewSpecimen) -> Specimen;
    fetch_specimen(SpecimenId) -> Specimen;
    list_specimens(SpecimenQuery) -> Vec<Specimen>;
    create_sequencing_run(NewSequencingRun) -> SequencingRun;
    fetch_sequencing_run(SequencingRunId) -> SequencingRun;
    list_sequencing_runs(SequencingRunQuery) -> Vec<SequencingRun>;
    create_suspension(NewSuspension) -> Suspension;
    fetch_suspension(SuspensionId) -> Suspension;
    list_suspensions(SuspensionQuery) -> Vec<Suspension>;
    create_suspension_pool(NewSuspensionPool) -> SuspensionPool;
    fetch_suspension_pool(SuspensionPoolId) -> SuspensionPool;
    list_suspension_pools(SuspensionPoolQuery) -> Vec<SuspensionPool>;
    create_chromium_run(NewChromiumRun) -> ChromiumRun;
    fetch_chromium_run(ChromiumRunId) -> ChromiumRun;
    list_chromium_runs(ChromiumRunQuery) -> Vec<ChromiumRun>;
    create_cdna(NewCdnaGroup) -> Vec<Cdna>;
    fetch_cdna(CdnaId) -> Cdna;
    list_cdna(CdnaQuery) -> Vec<Cdna>;
    create_library(NewLibrary) -> Library;
    fetch_library(LibraryId) -> Library;
    list_libraries(LibraryQuery) -> Vec<Library>;
    create_chromium_dataset(NewChromiumDataset) -> ChromiumDataset;
    fetch_chromium_dataset(ChromiumDatasetId) -> ChromiumDataset;
    list_chromium_datasets(ChromiumDatasetQuery) -> Vec<ChromiumDataset>
}

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pyo3::pymethods]
impl ScamplersClient {
    async fn list_multiplexing_tags(&self) -> Result<Vec<MultiplexingTag>, ScamplersErrorResponse> {
        let client = self.clone();
        client.send_request_python(()).await
    }
}

#[cfg(feature = "python")]
python_client_list_relatives_methods! {
    list_person_specimens(PersonId, SpecimenQuery) -> Vec<Specimen>;
}

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
