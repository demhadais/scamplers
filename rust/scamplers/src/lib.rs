#[cfg(feature = "app")]
mod app;
#[cfg(any(target_arch = "wasm32", feature = "python"))]
mod client;
mod macros;
mod routes;
