use scamplers_macros::base_api_model;

use crate::model::chromium_run::{
    mutiplex::NewMultiplexChromiumRun, ocm::NewOcmChromiumRun, singleplex::NewSingleplexChromiumRun,
};

mod common;
mod mutiplex;
mod ocm;
mod singleplex;

#[base_api_model]
#[serde(tag = "plexy")]
pub enum NewChromiumRun {
    Singleplex(#[garde(dive)] NewSingleplexChromiumRun),
    Ocm(#[garde(dive)] NewOcmChromiumRun),
    PoolMultiplex(#[garde(dive)] NewMultiplexChromiumRun),
}
