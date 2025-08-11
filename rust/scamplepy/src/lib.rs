use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;

#[pymodule]
mod scamplepy {
    use super::*;

    #[pymodule]
    mod requests {
        #[pymodule_export]
        use scamplers::routes::institution::NewInstitution;
    }
}

define_stub_info_gatherer!(stub_info);
