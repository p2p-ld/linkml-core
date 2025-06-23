use pyo3::prelude::*;
use runtime::runtime_module as inner_module;

#[pymodule(name = "linkml_runtime")]
fn linkml_runtime(m: &Bound<'_, PyModule>) -> PyResult<()> {
    inner_module(m)
}
