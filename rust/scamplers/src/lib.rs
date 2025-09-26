#![allow(uncommon_codepoints)]
// A comment to trigger CI

#[cfg(feature = "app")]
mod auth;
pub mod client;
#[cfg(feature = "app")]
pub mod config;
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

#[cfg(feature = "python")]
pub mod module_names {
    pub const PARENT_MODULE_NAME: &str = "scamplepy";
    pub const COMMON_SUBMODULE_NAME: &str = "scamplepy.common";
    pub const CREATE_SUBMODULE_NAME: &str = "scamplepy.create";
    pub const QUERY_SUBMODULE_NAME: &str = "scamplepy.query";
    pub const UPDATE_SUBMODULE_NAME: &str = "scamplepy.update";
    pub const ERRORS_SUBMODULE_NAME: &str = "scamplepy.errors";
    pub const RESPONSES_SUBMODULE_NAME: &str = "scamplepy.responses";
}

#[cfg(feature = "python")]
pub use module_names::*;
