use std::fmt::Display;

use axum::extract::Path;
use reqwest::{Request, RequestBuilder};

use crate::extract::RequestExtractorExt;

impl<T: Display> RequestExtractorExt<T> for Path<T> {
    fn inner(self) -> T {
        self.0
    }

    fn request_builder() -> impl Fn(reqwest::RequestBuilder, &T) -> reqwest::RequestBuilder + Send {
        |request, path_param| {
            let (client, request) = request.build_split();

            let request = request.unwrap();
            let url = request.url().as_str();
            let url = url.replace("{id}", &path_param.to_string());
            client.request(request.method().clone(), url)
        }
    }
}
