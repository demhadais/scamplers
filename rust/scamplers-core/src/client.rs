use reqwest::Method;
use serde::{Serialize, de::DeserializeOwned};
#[cfg(target_arch = "wasm32")]
use {
    crate::model::person::{CreatedUser, NewMsLogin},
    wasm_bindgen::prelude::*,
};
#[cfg(feature = "python")]
use {
    crate::model::{
        institution::{Institution, NewInstitution},
        lab::{Lab, NewLab},
        person::{NewPerson, Person},
        specimen::{NewSpecimen, Specimen},
        suspension::NewSuspension,
    },
    pyo3::prelude::*,
    std::sync::Arc,
    tokio::runtime::Runtime,
};

#[cfg(feature = "python")]
use crate::model::{
    chromium_run::{ChromiumRun, NewChromiumRun},
    suspension::Suspension,
};
use crate::{
    api_path::ToApiPath,
    result::{ClientError, ScamplersCoreError, ScamplersCoreErrorResponse, ServerError},
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
    #[pyo3(signature = (*, api_base_url, api_key=None))]
    fn py_new(api_base_url: String, api_key: Option<String>) -> Self {
        Self::new(api_base_url, None, api_key)
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
    ) -> Self {
        use reqwest::{
            ClientBuilder,
            header::{AUTHORIZATION, HeaderMap, HeaderValue},
        };

        let token = frontend_token.unwrap_or_default();

        let mut auth = HeaderValue::from_str(&format!("Bearer {token}")).unwrap();
        auth.set_sensitive(true);

        let headers = HeaderMap::from_iter([(AUTHORIZATION, auth)]);

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

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
    async fn send_request_with_body<Req, Resp>(
        &self,
        data: Req,
        method: Method,
    ) -> Result<Resp, ScamplersCoreErrorResponse>
    where
        Req: Serialize,
        Resp: DeserializeOwned,
        (Req, Resp): ToApiPath,
    {
        let Self {
            backend_base_url,
            client,
            api_key,
            ..
        } = self;

        let route = <(Req, Resp)>::to_api_path();

        let mut request = match method {
            Method::POST => client
                .post(format!("{backend_base_url}{route}"))
                .json(&data),
            _ => {
                return Err(ScamplersCoreErrorResponse::builder()
                    .error(ClientError {
                        message: format!("unexpected HTTP method {method}"),
                    })
                    .build());
            }
        };

        if let Some(api_key) = api_key {
            request = request.header("X-API-Key", api_key);
        }

        let response = request.send().await.unwrap();
        let status = Some(response.status().as_u16());
        let raw_response = response.bytes().await.unwrap();

        let deserialized_success_response = serde_json::from_slice(&raw_response);

        let Err(deserialization_failure1) = deserialized_success_response else {
            return Ok(deserialized_success_response.unwrap());
        };

        let deserialized_failure_response = serde_json::from_slice(&raw_response);

        let Err(deserialization_failure2) = deserialized_failure_response else {
            return Err(deserialized_failure_response.unwrap());
        };

        let inner_error = ServerError {
            message: format!(
                "failed to deserialize response body as success and as failure: \
                 {deserialization_failure1} / {deserialization_failure2}"
            ),
            raw_response_body: String::from_utf8(raw_response.to_vec()).unwrap_or_default(),
        };

        Err(ScamplersCoreErrorResponse {
            status,
            error: ScamplersCoreError::Server(inner_error),
        })
    }

    #[cfg(target_arch = "wasm32")]
    async fn send_request_wasm<Req, Resp>(
        &self,
        data: Req,
        method: Method,
    ) -> Result<Resp, ScamplersCoreErrorResponse>
    where
        Req: Serialize,
        Resp: DeserializeOwned,
        (Req, Resp): ToApiPath,
    {
        self.send_request_with_body(data, method).await
    }

    #[cfg(feature = "python")]
    async fn send_request_python<Req, Resp>(
        self,
        data: Req,
        method: Method,
    ) -> Result<Resp, ScamplersCoreErrorResponse>
    where
        Req: Serialize + Send + 'static,
        Resp: DeserializeOwned + Send + 'static,
        (Req, Resp): ToApiPath,
    {
        let runtime = self.runtime.clone();

        runtime
            .spawn(async move { self.send_request_with_body(data, method).await })
            .await
            .unwrap()
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl ScamplersClient {
    /// # Errors
    #[wasm_bindgen]
    pub async fn ms_login(
        &self,
        data: NewMsLogin,
    ) -> Result<CreatedUser, ScamplersCoreErrorResponse> {
        self.send_request_wasm(data, Method::POST).await
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl ScamplersClient {
    async fn create_institution(
        &self,
        data: NewInstitution,
    ) -> Result<Institution, ScamplersCoreErrorResponse> {
        let client = self.clone();
        client.send_request_python(data, Method::POST).await
    }

    async fn create_person(&self, data: NewPerson) -> Result<Person, ScamplersCoreErrorResponse> {
        let client = self.clone();
        client.send_request_python(data, Method::POST).await
    }

    async fn create_lab(&self, data: NewLab) -> Result<Lab, ScamplersCoreErrorResponse> {
        let client = self.clone();
        client.send_request_python(data, Method::POST).await
    }

    async fn create_specimen(
        &self,
        data: NewSpecimen,
    ) -> Result<Specimen, ScamplersCoreErrorResponse> {
        let client = self.clone();
        client.send_request_python(data, Method::POST).await
    }

    async fn create_suspension(
        &self,
        data: NewSuspension,
    ) -> Result<Suspension, ScamplersCoreErrorResponse> {
        let client = self.clone();
        client.send_request_python(data, Method::POST).await
    }

    async fn create_chromium_run(
        &self,
        data: NewChromiumRun,
    ) -> Result<ChromiumRun, ScamplersCoreErrorResponse> {
        let client = self.clone();
        client.send_request_python(data, Method::POST).await
    }
}
