use {
    crate::{
        api_path::ToApiPath,
        model::{
            institution::{Institution, NewInstitution},
            lab::{Lab, NewLab},
            person::{CreatedUser, NewPerson, Person},
        },
    },
    serde::{Serialize, de::DeserializeOwned},
};

#[cfg(target_arch = "wasm32")]
use {crate::model::person::NewMsLogin, wasm_bindgen::prelude::*};

#[cfg(not(target_arch = "wasm32"))]
use pyo3::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg_attr(not(target_arch = "wasm32"), pyclass)]
struct Client {
    backend_base_url: String,
    client: reqwest::Client,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg_attr(not(target_arch = "wasm32"), pymethods)]
impl Client {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    #[cfg_attr(not(target_arch = "wasm32"), new)]
    pub fn new(backend_base_url: String, token: &str) -> Self {
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
        }
    }
}

impl Client {
    async fn send_request<Req, Resp>(
        &self,
        data: &Req,
        api_key: Option<String>,
    ) -> Result<Resp, JsValue>
    where
        Req: Serialize,
        Resp: DeserializeOwned,
        (Req, Resp): ToApiPath,
    {
        let Self {
            backend_base_url,
            client,
        } = self;

        let route = <(Req, Resp)>::to_api_path();

        let mut request = client.post(format!("{backend_url}{route}")).json(data);

        if let Some(api_key) = api_key {
            request = request.header("X-API-Key", api_key);
        }

        let response = request
            .send()
            .await
            .unwrap_throw()
            .bytes()
            .await
            .unwrap_throw();

        let Ok(response) = serde_json::from_slice(&response) else {
            let error: serde_json::Value = serde_json::from_slice(&response).unwrap_throw();
            let error = serde_wasm_bindgen::to_value(&error).unwrap_throw();

            return Err(error);
        };

        Ok(response)
    }
}

#[cfg(target_arch = "wasm32")]
impl Client {
    #[wasm_bindgen]
    pub async fn send_new_ms_login(
        &self,
        data: NewMsLogin,
    ) -> Result<CreatedUser, wasm_bindgen::JsValue> {
        self.send_request(&data, None).await
    }
}
