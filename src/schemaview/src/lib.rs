pub mod io;
pub mod schemaview;
pub mod curie;
pub mod resolve;
pub mod identifier;
extern crate linkml_meta;

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use std::path::Path;
#[cfg(feature = "python")]
use pyo3::exceptions::PyException;
use crate::schemaview::SchemaView as RustSchemaView;

/// Formats the sum of two numbers as string.
#[cfg(feature = "python")]
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[cfg(feature = "python")]
#[pyclass(name = "SchemaView")]
pub struct PySchemaView {
    inner: RustSchemaView,
}

#[cfg(feature = "python")]
#[pymethods]
impl PySchemaView {
    #[new]
    fn new(path: Option<&str>) -> PyResult<Self> {
        let mut sv = RustSchemaView::new();
        if let Some(p) = path {
            let schema = crate::io::from_yaml(Path::new(p))
                .map_err(|e| PyException::new_err(e.to_string()))?;
            sv.add_schema(schema)
                .map_err(|e| PyException::new_err(e))?;
        }
        Ok(Self { inner: sv })
    }

    fn get_unresolved_schemas(&self) -> Vec<String> {
        self.inner.get_unresolved_schemas()
    }
}

/// A Python module implemented in Rust.
#[cfg(feature = "python")]
#[pymodule(name="schemaview")]
pub fn schemaview_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<PySchemaView>()?;
    Ok(())
}
