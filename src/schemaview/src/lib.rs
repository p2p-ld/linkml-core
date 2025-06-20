pub mod curie;
pub mod identifier;
pub mod io;
pub mod resolve;
pub mod classview;
pub mod slotview;
pub mod schemaview;
extern crate linkml_meta;

#[cfg(feature = "python")]
use crate::identifier::{converter_from_schemas, Identifier};
#[cfg(feature = "python")]
use crate::schemaview::SchemaView as RustSchemaView;
#[cfg(feature = "python")]
use linkml_meta::{ClassDefinition, SchemaDefinition};
#[cfg(feature = "python")]
use pyo3::exceptions::PyException;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use serde_path_to_error;
#[cfg(feature = "python")]
use serde_yml;
#[cfg(feature = "python")]
use std::path::Path;

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
    #[pyo3(signature = (path=None))]
    fn new(path: Option<&str>) -> PyResult<Self> {
        let mut sv = RustSchemaView::new();
        if let Some(p) = path {
            let schema = crate::io::from_yaml(Path::new(p))
                .map_err(|e| PyException::new_err(e.to_string()))?;
            sv.add_schema(schema).map_err(|e| PyException::new_err(e))?;
        }
        Ok(Self { inner: sv })
    }

    fn add_schema_from_path(&mut self, path: &str) -> PyResult<()> {
        let schema = crate::io::from_yaml(Path::new(path))
            .map_err(|e| PyException::new_err(e.to_string()))?;
        self.inner
            .add_schema(schema)
            .map_err(|e| PyException::new_err(e))
    }

    fn add_schema_str(&mut self, data: &str) -> PyResult<()> {
        let deser = serde_yml::Deserializer::from_str(data);
        let schema: SchemaDefinition = serde_path_to_error::deserialize(deser)
            .map_err(|e| PyException::new_err(e.to_string()))?;
        self.inner
            .add_schema(schema)
            .map_err(|e| PyException::new_err(e))
    }

    fn get_unresolved_schemas(&self) -> Vec<String> {
        self.inner.get_unresolved_schemas()
    }

    fn get_schema(&self, uri: &str) -> Option<SchemaDefinition> {
        self.inner.get_schema(uri).cloned()
    }

    fn get_class(&self, id: &str) -> PyResult<Option<ClassDefinition>> {
        let conv = self.inner.converter();
        Ok(self
            .inner
            .get_class_definition(&Identifier::new(id), &conv)
            .map_err(|e| PyException::new_err(format!("{:?}", e)))?
            .cloned())
    }
}

/// A Python module implemented in Rust.
#[cfg(feature = "python")]
#[pymodule(name = "schemaview")]
pub fn schemaview_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<PySchemaView>()?;
    m.add_class::<SchemaDefinition>()?;
    m.add_class::<ClassDefinition>()?;
    Ok(())
}
