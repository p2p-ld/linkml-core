use crate::turtle::{turtle_to_string, TurtleOptions};
use crate::{load_json_str, load_yaml_str, LinkMLValue};
use curies::Converter;
use linkml_meta::{ClassDefinition, SchemaDefinition};
use linkml_schemaview::identifier::Identifier;
use linkml_schemaview::io;
use linkml_schemaview::schemaview::SchemaView;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyAnyMethods;
use pyo3::types::{PyAny, PyModule};
use pyo3::Bound;
use pyo3::{wrap_pyfunction, wrap_pymodule};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
fn py_filelike_or_string_to_string(obj: &Bound<'_, PyAny>) -> PyResult<String> {
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

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyclass(name = "SchemaView")]
pub struct PySchemaView {
    inner: Arc<SchemaView>,
}

impl PySchemaView {
    pub fn as_rust(&self) -> &SchemaView {
        self.inner.as_ref()
    }
}

#[pyclass(name = "ClassView")]
pub struct PyClassView {
    class_name: String,
    sv: Arc<SchemaView>,
}

#[pyclass(name = "SlotView")]
pub struct PySlotView {
    slot_name: String,
    sv: Arc<SchemaView>,
}

#[pymethods]
impl PySchemaView {
    #[new]
    #[pyo3(signature = (source=None))]
    pub fn new(source: Option<&Bound<'_, PyAny>>) -> PyResult<Self> {
        let mut sv = SchemaView::new();
        if let Some(obj) = source {
            let text = py_filelike_or_string_to_string(obj)?;
            let deser = serde_yml::Deserializer::from_str(&text);
            let schema: SchemaDefinition = serde_path_to_error::deserialize(deser)
                .map_err(|e| PyException::new_err(e.to_string()))?;
            sv.add_schema(schema).map_err(|e| PyException::new_err(e))?;
        }
        Ok(Self {
            inner: Arc::new(sv),
        })
    }

    fn add_schema_from_path(&mut self, path: &str) -> PyResult<()> {
        let schema =
            io::from_yaml(Path::new(path)).map_err(|e| PyException::new_err(e.to_string()))?;
        if let Some(inner) = Arc::get_mut(&mut self.inner) {
            inner
                .add_schema(schema)
                .map_err(|e| PyException::new_err(e))
        } else {
            Err(PyException::new_err("SchemaView already shared"))
        }
    }

    fn add_schema_str(&mut self, data: &str) -> PyResult<()> {
        let deser = serde_yml::Deserializer::from_str(data);
        let schema: SchemaDefinition = serde_path_to_error::deserialize(deser)
            .map_err(|e| PyException::new_err(e.to_string()))?;
        if let Some(inner) = Arc::get_mut(&mut self.inner) {
            inner
                .add_schema(schema)
                .map_err(|e| PyException::new_err(e))
        } else {
            Err(PyException::new_err("SchemaView already shared"))
        }
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

    fn get_class_view(&self, id: &str) -> PyResult<Option<PyClassView>> {
        let conv = self.inner.converter();
        Ok(self
            .inner
            .get_class(&Identifier::new(id), &conv)
            .map_err(|e| PyException::new_err(format!("{:?}", e)))?
            .map(|cv| PyClassView {
                class_name: cv.class.name.clone(),
                sv: self.inner.clone(),
            }))
    }

    fn get_slot_view(&self, id: &str) -> PyResult<Option<PySlotView>> {
        let conv = self.inner.converter();
        Ok(self
            .inner
            .get_slot(&Identifier::new(id), &conv)
            .map_err(|e| PyException::new_err(format!("{:?}", e)))?
            .map(|svw| PySlotView {
                slot_name: svw.name,
                sv: self.inner.clone(),
            }))
    }
}

#[pymethods]
impl PyClassView {
    #[getter]
    pub fn name(&self) -> String {
        self.class_name.clone()
    }

    fn slots(&self) -> PyResult<Vec<PySlotView>> {
        let conv = self.sv.converter();
        let opt = self
            .sv
            .get_class(&Identifier::new(&self.class_name), &conv)
            .map_err(|e| PyException::new_err(format!("{:?}", e)))?;
        match opt {
            Some(cv) => Ok(cv
                .slots()
                .iter()
                .map(|s| PySlotView {
                    slot_name: s.name.clone(),
                    sv: self.sv.clone(),
                })
                .collect()),
            None => Ok(Vec::new()),
        }
    }
}

#[pymethods]
impl PySlotView {
    #[getter]
    pub fn name(&self) -> String {
        self.slot_name.clone()
    }
}

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

#[pyfunction]
#[pyo3(signature = (source=None))]
fn make_schema_view(source: Option<&Bound<'_, PyAny>>) -> PyResult<PySchemaView> {
    PySchemaView::new(source)
}

/// Python bindings for `linkml_runtime`.
#[pymodule(name = "linkml_runtime")]
pub fn runtime_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(schemaview_module))?;
    m.add_function(wrap_pyfunction!(make_schema_view, m)?)?;
    m.add_function(wrap_pyfunction!(load_yaml, m)?)?;
    m.add_function(wrap_pyfunction!(load_json, m)?)?;
    m.add_class::<PyLinkMLValue>()?;
    Ok(())
}

#[derive(Clone)]
enum LinkMLValueOwned {
    Scalar {
        value: JsonValue,
        slot: Option<String>,
    },
    List {
        values: Vec<LinkMLValueOwned>,
        slot: Option<String>,
    },
    Map {
        values: HashMap<String, LinkMLValueOwned>,
        class: Option<String>,
    },
}

impl LinkMLValueOwned {
    fn from_linkml<'a>(v: &LinkMLValue<'a>) -> Self {
        match v {
            LinkMLValue::Scalar { value, slot, .. } => LinkMLValueOwned::Scalar {
                value: value.clone(),
                slot: slot.as_ref().map(|s| s.name.clone()),
            },
            LinkMLValue::List { values, slot, .. } => LinkMLValueOwned::List {
                values: values.iter().map(Self::from_linkml).collect(),
                slot: slot.as_ref().map(|s| s.name.clone()),
            },
            LinkMLValue::Map { values, class, .. } => LinkMLValueOwned::Map {
                values: values
                    .iter()
                    .map(|(k, v)| (k.clone(), Self::from_linkml(v)))
                    .collect(),
                class: class.as_ref().map(|c| c.class.name.clone()),
            },
        }
    }

    fn to_json(&self) -> JsonValue {
        match self {
            LinkMLValueOwned::Scalar { value, .. } => value.clone(),
            LinkMLValueOwned::List { values, .. } => {
                JsonValue::Array(values.iter().map(|v| v.to_json()).collect())
            }
            LinkMLValueOwned::Map { values, .. } => JsonValue::Object(
                values
                    .iter()
                    .map(|(k, v)| (k.clone(), v.to_json()))
                    .collect(),
            ),
        }
    }

    fn to_linkml<'a>(&self, sv: &'a SchemaView) -> LinkMLValue<'a> {
        fn inner<'a>(v: &LinkMLValueOwned, sv: &'a SchemaView, conv: &Converter) -> LinkMLValue<'a> {
            match v {
                LinkMLValueOwned::Scalar { value, slot } => {
                    let slot_view = slot
                        .as_ref()
                        .and_then(|n| sv.get_slot(&Identifier::new(n), conv).ok().flatten());
                    LinkMLValue::Scalar {
                        value: value.clone(),
                        slot: slot_view,
                        sv,
                    }
                }
                LinkMLValueOwned::List { values, slot } => {
                    let slot_view = slot
                        .as_ref()
                        .and_then(|n| sv.get_slot(&Identifier::new(n), conv).ok().flatten());
                    LinkMLValue::List {
                        values: values.iter().map(|v| inner(v, sv, conv)).collect(),
                        slot: slot_view,
                        sv,
                    }
                }
                LinkMLValueOwned::Map { values, class } => {
                    let class_view = class
                        .as_ref()
                        .and_then(|n| sv.get_class(&Identifier::new(n), conv).ok().flatten());
                    LinkMLValue::Map {
                        values: values
                            .iter()
                            .map(|(k, v)| (k.clone(), inner(v, sv, conv)))
                            .collect(),
                        class: class_view,
                        sv,
                    }
                }
            }
        }
        let conv = sv.converter();
        inner(self, sv, &conv)
    }
}


#[pyclass(name = "LinkMLValue")]
pub struct PyLinkMLValue {
    value: LinkMLValueOwned,
    sv: Py<PySchemaView>,
}

impl PyLinkMLValue {
    fn new(value: LinkMLValueOwned, sv: Py<PySchemaView>) -> Self {
        Self { value, sv }
    }
}

fn json_value_to_py(py: Python<'_>, v: &JsonValue) -> PyObject {
    let s = serde_json::to_string(v).unwrap();
    let json_mod = PyModule::import(py, "json").unwrap();
    json_mod.call_method1("loads", (s,)).unwrap().unbind()
}

impl Clone for PyLinkMLValue {
    fn clone(&self) -> Self {
        Python::with_gil(|py| Self {
            value: self.value.clone(),
            sv: self.sv.clone_ref(py),
        })
    }
}

#[pymethods]
impl PyLinkMLValue {
    fn slot_name(&self) -> Option<String> {
        match &self.value {
            LinkMLValueOwned::Scalar { slot: Some(n), .. } => Some(n.clone()),
            LinkMLValueOwned::List { slot: Some(n), .. } => Some(n.clone()),
            _ => None,
        }
    }

    fn class_name(&self) -> Option<String> {
        match &self.value {
            LinkMLValueOwned::Map { class: Some(n), .. } => Some(n.clone()),
            _ => None,
        }
    }

    fn __len__(&self) -> PyResult<usize> {
        Ok(match &self.value {
            LinkMLValueOwned::Scalar { .. } => 0,
            LinkMLValueOwned::List { values, .. } => values.len(),
            LinkMLValueOwned::Map { values, .. } => values.len(),
        })
    }

    fn __getitem__<'py>(
        &self,
        py: Python<'py>,
        key: &Bound<'py, PyAny>,
    ) -> PyResult<PyLinkMLValue> {
        match &self.value {
            LinkMLValueOwned::List { values, .. } => {
                let idx: usize = key.extract()?;
                values
                    .get(idx)
                    .map(|v| PyLinkMLValue::new(v.clone(), self.sv.clone_ref(py)))
                    .ok_or_else(|| PyException::new_err("index out of range"))
            }
            LinkMLValueOwned::Map { values, .. } => {
                let k: String = key.extract()?;
                values
                    .get(&k)
                    .map(|v| PyLinkMLValue::new(v.clone(), self.sv.clone_ref(py)))
                    .ok_or_else(|| PyException::new_err("key not found"))
            }
            _ => Err(PyException::new_err("not indexable")),
        }
    }

    fn keys(&self) -> PyResult<Vec<String>> {
        match &self.value {
            LinkMLValueOwned::Map { values, .. } => Ok(values.keys().cloned().collect()),
            _ => Ok(Vec::new()),
        }
    }

    fn values<'py>(&self, py: Python<'py>) -> PyResult<Vec<PyLinkMLValue>> {
        match &self.value {
            LinkMLValueOwned::Map { values, .. } => Ok(values
                .values()
                .cloned()
                .map(|v| PyLinkMLValue::new(v, self.sv.clone_ref(py)))
                .collect()),
            LinkMLValueOwned::List { values, .. } => Ok(values
                .iter()
                .cloned()
                .map(|v| PyLinkMLValue::new(v, self.sv.clone_ref(py)))
                .collect()),
            _ => Ok(Vec::new()),
        }
    }

    fn as_python<'py>(&self, py: Python<'py>) -> PyObject {
        json_value_to_py(py, &self.value.to_json())
    }

    fn as_turtle<'py>(&self, py: Python<'py>, skolem: Option<bool>) -> PyResult<String> {
        let sv_ref = self.sv.bind(py).borrow();
        let rust_sv = sv_ref.as_rust();
        let schema = rust_sv
            .primary_schema()
            .ok_or_else(|| PyException::new_err("no schema loaded"))?;
        let conv = rust_sv.converter();
        let linkml = self.value.to_linkml(rust_sv);
        turtle_to_string(
            &linkml,
            rust_sv,
            schema,
            &conv,
            TurtleOptions {
                skolem: skolem.unwrap_or(false),
            },
        )
        .map_err(|e| PyException::new_err(e.to_string()))
    }
}

#[pyfunction]
fn load_yaml(
    py: Python<'_>,
    source: &Bound<'_, PyAny>,
    sv: Py<PySchemaView>,
    class: Option<Py<PyClassView>>,
) -> PyResult<PyLinkMLValue> {
    let sv_ref = sv.bind(py).borrow();
    let rust_sv = sv_ref.as_rust();
    let conv = rust_sv.converter();
    let class_view = match class {
        Some(cv) => {
            let cv_ref = cv.bind(py).borrow();
            rust_sv
                .get_class(&Identifier::new(&cv_ref.name()), &conv)
                .map_err(|e| PyException::new_err(format!("{:?}", e)))?
        }
        None => None,
    };
    let text = py_filelike_or_string_to_string(source)?;
    let v = load_yaml_str(&text, rust_sv, class_view.as_ref(), &conv)
        .map_err(|e| PyException::new_err(e.to_string()))?;
    Ok(PyLinkMLValue::new(LinkMLValueOwned::from_linkml(&v), sv))
}

#[pyfunction]
fn load_json(
    py: Python<'_>,
    source: &Bound<'_, PyAny>,
    sv: Py<PySchemaView>,
    class: Option<Py<PyClassView>>,
) -> PyResult<PyLinkMLValue> {
    let sv_ref = sv.bind(py).borrow();
    let rust_sv = sv_ref.as_rust();
    let conv = rust_sv.converter();
    let class_view = match class {
        Some(cv) => {
            let cv_ref = cv.bind(py).borrow();
            rust_sv
                .get_class(&Identifier::new(&cv_ref.name()), &conv)
                .map_err(|e| PyException::new_err(format!("{:?}", e)))?
        }
        None => None,
    };
    let text = py_filelike_or_string_to_string(source)?;
    let v = load_json_str(&text, rust_sv, class_view.as_ref(), &conv)
        .map_err(|e| PyException::new_err(e.to_string()))?;
    Ok(PyLinkMLValue::new(LinkMLValueOwned::from_linkml(&v), sv))
}
