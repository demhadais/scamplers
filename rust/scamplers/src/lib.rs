#![allow(uncommon_codepoints)]

#[cfg(feature = "app")]
pub mod app;
#[cfg(any(target_arch = "wasm32", feature = "python"))]
mod client;
#[cfg(feature = "app")]
pub mod db;
mod macros;
pub mod result;
pub mod routes;
