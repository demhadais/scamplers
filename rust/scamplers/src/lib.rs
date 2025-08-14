#![allow(uncommon_codepoints)]

#[cfg(feature = "app")]
mod auth;
#[cfg(any(target_arch = "wasm32", feature = "python"))]
mod client;
#[cfg(feature = "app")]
mod config;
pub mod db;
#[cfg(feature = "app")]
pub mod dev_container;
mod endpoints;
mod extract;
pub mod result;
#[cfg(feature = "app")]
pub mod server;
#[cfg(feature = "app")]
mod state;
