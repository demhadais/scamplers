use {
    crate::api_path::ToApiPath,
    reqwest::Method,
    serde::{Serialize, de::DeserializeOwned},
};

#[cfg(target_arch = "wasm32")]
use {
    crate::model::person::{CreatedUser, NewMsLogin},
    wasm_bindgen::prelude::*,
};

#[cfg(feature = "python")]
use {
    crate::model::person::NewPerson,
    pyo3::{exceptions::PyException, prelude::*},
};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg_attr(feature = "python", pyclass)]
pub struct Client {
    backend_base_url: String,
    client: reqwest::Client,
    api_key: Option<String>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg_attr(feature = "python", pymethods)]
impl Client {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    #[cfg_attr(feature = "python", new)]
    pub fn new(backend_base_url: String, token: &str, api_key: Option<String>) -> Self {
        use reqwest::{
            ClientBuilder,
            header::{AUTHORIZATION, HeaderMap, HeaderValue},
        };

        let mut auth = HeaderValue::from_str(&format!("Bearer {token}")).unwrap();
        auth.set_sensitive(true);

        let headers = HeaderMap::from_iter([(AUTHORIZATION, auth)]);

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            backend_base_url,
            client,
            api_key,
        }
    }
}

impl Client {
    async fn send_request_with_body<Req, Resp>(
        &self,
        data: &Req,
        method: Method,
    ) -> Result<Resp, Vec<u8>>
    where
        Req: Serialize,
        Resp: DeserializeOwned,
        (Req, Resp): ToApiPath,
    {
        let Self {
            backend_base_url,
            client,
            api_key,
        } = self;

        let route = <(Req, Resp)>::to_api_path();

        let mut request = match method {
            Method::POST => client.post(format!("{backend_base_url}{route}")).json(data),
            _ => return Err(vec![]),
        };

        if let Some(api_key) = api_key {
            request = request.header("X-API-Key", api_key);
        }

        let response = request.send().await.unwrap().bytes().await.unwrap();

        let Ok(response) = serde_json::from_slice(&response) else {
            return Err(response.to_vec());
        };

        Ok(response)
    }

    #[cfg(target_arch = "wasm32")]
    async fn send_request_wasm<Req, Resp>(
        &self,
        data: &Req,
        method: Method,
    ) -> Result<Resp, wasm_bindgen::JsValue>
    where
        Req: Serialize,
        Resp: DeserializeOwned,
        (Req, Resp): ToApiPath,
    {
        fn bytes_to_wasm_value(bytes: Vec<u8>) -> wasm_bindgen::JsValue {
            let as_json: serde_json::Value = serde_json::from_slice(&bytes).unwrap_throw();
            serde_wasm_bindgen::to_value(&as_json).unwrap_throw()
        }

        self.send_request_with_body(data, method)
            .await
            .map_err(bytes_to_wasm_value)
    }

    #[cfg(feature = "python")]
    async fn send_request_python<Req, Resp>(&self, data: &Req, method: Method) -> PyResult<Resp>
    where
        Req: Serialize,
        Resp: DeserializeOwned,
        (Req, Resp): ToApiPath,
    {
        fn bytes_to_python_exception(bytes: Vec<u8>) -> PyErr {
            let as_json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
            PyException::new_err(serde_json::to_string(&as_json).unwrap())
        }

        self.send_request_with_body(data, method)
            .await
            .map_err(bytes_to_python_exception)
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Client {
    #[wasm_bindgen]
    pub async fn ms_login(&self, data: &NewMsLogin) -> Result<CreatedUser, wasm_bindgen::JsValue> {
        self.send_request_wasm(data, Method::POST).await
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl Client {
    async fn create_person(&self, data: NewPerson) -> PyResult<Person> {
        self.send_request_python(&data, Method::POST).await
    }
}
