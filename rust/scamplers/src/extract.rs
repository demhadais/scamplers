mod base64_json_query;
mod path;
mod user;
mod valid_json;
pub use base64_json_query::Base64JsonQuery;
pub use valid_json::ValidJsonBody;

pub trait RequestExtractorExt<T> {
    fn inner(self) -> T;
    fn request_builder() -> impl Fn(reqwest::RequestBuilder, &T) -> reqwest::RequestBuilder + Send;
}
