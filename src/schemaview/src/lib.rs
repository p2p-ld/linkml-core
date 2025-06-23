pub mod classview;
pub mod curie;
pub mod identifier;
pub mod io;
pub mod resolve;
pub mod schemaview;
pub mod slotview;
extern crate linkml_meta;

#[cfg(feature = "python")]
use crate::identifier::Identifier;
#[cfg(feature = "python")]
use crate::schemaview::SchemaView as RustSchemaView;
#[cfg(feature = "python")]
use linkml_meta::{ClassDefinition, SchemaDefinition};
#[cfg(feature = "python")]
use pyo3::exceptions::PyException;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::{PyAny, PyAnyMethods};
#[cfg(feature = "python")]
use pyo3::IntoPyObjectExt;
#[cfg(feature = "python")]
use pyo3::PyRef;
#[cfg(feature = "python")]
use serde_path_to_error;
#[cfg(feature = "python")]
use serde_yml;
#[cfg(feature = "python")]
use std::fs;
#[cfg(feature = "python")]
use std::path::Path;

/// Formats the sum of two numbers as string.
#[cfg(feature = "python")]
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[cfg(feature = "python")]
fn py_any_to_string(obj: &Bound<'_, PyAny>) -> PyResult<String> {
    if let Ok(s) = obj.extract::<String>() {
        if Path::new(&s).exists() {
            return fs::read_to_string(&s).map_err(|e| PyException::new_err(e.to_string()));
        }
        return Ok(s);
    }
    if obj.hasattr("__fspath__")? {
        let p: String = obj.call_method0("__fspath__")?.extract()?;
        return fs::read_to_string(&p).map_err(|e| PyException::new_err(e.to_string()));
    }
    if obj.hasattr("read")? {
        return obj.call_method0("read")?.extract();
    }
    Err(PyException::new_err("expected string or file-like object"))
}

#[cfg(feature = "python")]
#[pyclass(name = "SchemaView")]
pub struct PySchemaView {
    inner: RustSchemaView,
}

impl PySchemaView {
    pub fn as_rust(&self) -> &RustSchemaView {
        &self.inner
    }
}

#[cfg(feature = "python")]
#[pyclass(name = "ClassView")]
pub struct PyClassView {
    class_name: String,
    sv: Py<PySchemaView>,
}

#[cfg(feature = "python")]
#[pyclass(name = "SlotView")]
pub struct PySlotView {
    slot_name: String,
    sv: Py<PySchemaView>,
}

#[cfg(feature = "python")]
#[pymethods]
impl PySchemaView {
    #[new]
    #[pyo3(signature = (source=None))]
    pub fn new(source: Option<&Bound<'_, PyAny>>) -> PyResult<Self> {
        let mut sv = RustSchemaView::new();
        if let Some(obj) = source {
            let text = py_any_to_string(obj)?;
            let deser = serde_yml::Deserializer::from_str(&text);
            let schema: SchemaDefinition = serde_path_to_error::deserialize(deser)
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

    fn get_class_view<'py>(
        slf: PyRef<'py, Self>,
        py: Python<'py>,
        id: &str,
    ) -> PyResult<Option<PyClassView>> {
        let conv = slf.inner.converter();
        let opt = slf
            .inner
            .get_class(&Identifier::new(id), &conv)
            .map_err(|e| PyException::new_err(format!("{:?}", e)))?;
        let tmp = (&slf).into_pyobject_or_pyerr(py)?;
        let py_self = tmp.to_owned().unbind();
        Ok(opt.map(|cv| PyClassView {
            class_name: cv.class.name.clone(),
            sv: py_self.clone_ref(py),
        }))
    }

    fn get_slot_view<'py>(
        slf: PyRef<'py, Self>,
        py: Python<'py>,
        id: &str,
    ) -> PyResult<Option<PySlotView>> {
        let conv = slf.inner.converter();
        let opt = slf
            .inner
            .get_slot(&Identifier::new(id), &conv)
            .map_err(|e| PyException::new_err(format!("{:?}", e)))?;
        let tmp = (&slf).into_pyobject_or_pyerr(py)?;
        let py_self = tmp.to_owned().unbind();
        Ok(opt.map(|svw| PySlotView {
            slot_name: svw.name,
            sv: py_self.clone_ref(py),
        }))
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl PyClassView {
    #[getter]
    pub fn name(&self) -> String {
        self.class_name.clone()
    }

    fn slots(&self, py: Python<'_>) -> PyResult<Vec<PySlotView>> {
        let sv = self.sv.bind(py).borrow();
        let conv = sv.inner.converter();
        let opt = sv
            .inner
            .get_class(&Identifier::new(&self.class_name), &conv)
            .map_err(|e| PyException::new_err(format!("{:?}", e)))?;
        match opt {
            Some(cv) => Ok(cv
                .slots()
                .iter()
                .map(|s| PySlotView {
                    slot_name: s.name.clone(),
                    sv: self.sv.clone_ref(py),
                })
                .collect()),
            None => Ok(Vec::new()),
        }
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl PySlotView {
    #[getter]
    pub fn name(&self) -> String {
        self.slot_name.clone()
    }
}

/// A Python module implemented in Rust.
#[cfg(feature = "python")]
#[pymodule(name = "linkml_schemaview")]
pub fn schemaview_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<PySchemaView>()?;
    m.add_class::<PyClassView>()?;
    m.add_class::<PySlotView>()?;
    m.add_class::<SchemaDefinition>()?;
    m.add_class::<ClassDefinition>()?;
    Ok(())
}
