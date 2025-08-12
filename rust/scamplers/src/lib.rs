#![allow(uncommon_codepoints)]

#[cfg(any(target_arch = "wasm32", feature = "python"))]
mod client;
pub mod db;
pub mod result;
#[cfg(feature = "app")]
pub mod server;
