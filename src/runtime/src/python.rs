use crate::turtle::{turtle_to_string, TurtleOptions};
use crate::{load_json_str, load_yaml_str, LinkMLValue};
use linkml_meta::{ClassDefinition, EnumDefinition, SchemaDefinition, SlotDefinition};
use linkml_schemaview::identifier::Identifier;
use linkml_schemaview::io;
use linkml_schemaview::schemaview::SchemaView;
use linkml_schemaview::{classview::ClassView, slotview::SlotView};
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
fn py_filelike_or_string_to_string(obj: &Bound<'_, PyAny>) -> PyResult<(String, Option<String>)> {
    if let Ok(s) = obj.extract::<String>() {
        if Path::new(&s).exists() {
            return fs::read_to_string(&s)
                .map_err(|e| PyException::new_err(e.to_string()))
                .map(|res| (res, Some(s)));
        }
        return Ok((s, None));
    }
    if obj.hasattr("__fspath__")? {
        let p: String = obj.call_method0("__fspath__")?.extract()?;
        return fs::read_to_string(&p)
            .map_err(|e| PyException::new_err(e.to_string()))
            .map(|res| (res, Some(p)));
    }
    if obj.hasattr("read")? {
        return obj.call_method0("read")?.extract().map(|res| (res, None));
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
#[derive(Clone)]
pub struct PyClassView {
    inner: ClassView,
}

impl PyClassView {
    pub fn as_rust(&self) -> &ClassView {
        &self.inner
    }
}

#[pyclass(name = "SlotView")]
#[derive(Clone)]
pub struct PySlotView {
    inner: SlotView,
}

impl PySlotView {
    pub fn as_rust(&self) -> &SlotView {
        &self.inner
    }
}

#[pymethods]
impl PySchemaView {
    #[new]
    #[pyo3(signature = (source=None))]
    pub fn new(source: Option<&Bound<'_, PyAny>>) -> PyResult<Self> {
        let mut sv = SchemaView::new();
        if let Some(obj) = source {
            let (text, uri) = py_filelike_or_string_to_string(obj)?;
            let deser = serde_yml::Deserializer::from_str(&text);
            let schema: SchemaDefinition = serde_path_to_error::deserialize(deser)
                .map_err(|e| PyException::new_err(e.to_string()))?;
            sv.add_schema_with_import_ref(schema, uri.map(|u| ("".to_owned(), u)))
                .map_err(|e| PyException::new_err(e))?;
        }
        Ok(Self {
            inner: Arc::new(sv),
        })
    }

    fn _get_resolved_schema_imports(&self) -> HashMap<(String, String), String> {
        self.inner._get_resolved_schema_imports()
    }

    fn add_schema_from_path(&mut self, path: &str) -> PyResult<bool> {
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

    fn add_schema_str(&mut self, data: &str) -> PyResult<bool> {
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
        self.inner
            .get_unresolved_schemas()
            .iter()
            .map(|(_, uri)| uri.clone())
            .collect()
    }

    fn get_unresolved_schema_refs(&self) -> Vec<(String, String)> {
        self.inner.get_unresolved_schemas()
    }

    fn get_resolution_uri_of_schema(&self, id: &str) -> Option<String> {
        self.inner.get_resolution_uri_of_schema(id)
    }

    fn add_schema_str_with_import_ref(
        &mut self,
        data: &str,
        schema_id: &str,
        uri: &str,
    ) -> PyResult<()> {
        let deser = serde_yml::Deserializer::from_str(data);
        let schema: SchemaDefinition = serde_path_to_error::deserialize(deser)
            .map_err(|e| PyException::new_err(e.to_string()))?;
        if let Some(inner) = Arc::get_mut(&mut self.inner) {
            inner
                .add_schema_with_import_ref(schema, Some((schema_id.to_string(), uri.to_string())))
                .map_err(PyException::new_err)?;
            Ok(())
        } else {
            Err(PyException::new_err("SchemaView already shared"))
        }
    }

    fn get_schema(&self, uri: &str) -> Option<SchemaDefinition> {
        self.inner.get_schema(uri).cloned()
    }

    fn get_class_view(&self, id: &str) -> PyResult<Option<PyClassView>> {
        let conv = self.inner.converter();
        Ok(self
            .inner
            .get_class(&Identifier::new(id), &conv)
            .map_err(|e| PyException::new_err(format!("{:?}", e)))?
            .map(|cv| PyClassView { inner: cv }))
    }

    fn get_slot_view(&self, id: &str) -> PyResult<Option<PySlotView>> {
        let conv = self.inner.converter();
        Ok(self
            .inner
            .get_slot(&Identifier::new(id), &conv)
            .map_err(|e| PyException::new_err(format!("{:?}", e)))?
            .map(|svw| PySlotView { inner: svw }))
    }

    fn schema_definitions(&self) -> Vec<SchemaDefinition> {
        self.inner.iter_schemas().map(|(_, s)| s.clone()).collect()
    }

    fn class_definitions(&self) -> Vec<PyClassView> {
        let mut defs = Vec::new();
        let conv = self.inner.converter();
        for (_, schema) in self.inner.iter_schemas() {
            if let Some(classes) = &schema.classes {
                for name in classes.keys() {
                    if let Ok(Some(cv)) = self.inner.get_class(&Identifier::new(name), &conv) {
                        defs.push(PyClassView { inner: cv });
                    }
                }
            }
        }
        defs
    }

    fn slot_definitions(&self) -> Vec<PySlotView> {
        let mut defs = Vec::new();
        let conv = self.inner.converter();
        for (_, schema) in self.inner.iter_schemas() {
            if let Some(slots) = &schema.slot_definitions {
                for name in slots.keys() {
                    if let Ok(Some(sv)) = self.inner.get_slot(&Identifier::new(name), &conv) {
                        defs.push(PySlotView { inner: sv });
                    }
                }
            }
        }
        defs
    }
}

#[pymethods]
impl PyClassView {
    #[getter]
    pub fn name(&self) -> String {
        self.inner.name().to_string()
    }

    #[getter]
    pub fn definition(&self) -> ClassDefinition {
        self.inner.def().clone()
    }

    fn slots(&self) -> PyResult<Vec<PySlotView>> {
        Ok(self
            .inner
            .slots()
            .iter()
            .cloned()
            .map(|s| PySlotView { inner: s })
            .collect())
    }

    fn parent_class(&self) -> PyResult<Option<PyClassView>> {
        self.inner
            .parent_class()
            .map_err(|e| PyException::new_err(format!("{:?}", e)))
            .map(|opt| opt.map(|cv| PyClassView { inner: cv }))
    }

    fn identifier_slot(&self) -> Option<PySlotView> {
        self.inner
            .identifier_slot()
            .map(|sv| PySlotView { inner: sv.clone() })
    }

    fn key_or_identifier_slot(&self) -> Option<PySlotView> {
        self.inner
            .key_or_identifier_slot()
            .map(|sv| PySlotView { inner: sv.clone() })
    }

    fn get_descendants(&self, recurse: bool, include_mixins: bool) -> PyResult<Vec<PyClassView>> {
        self.inner
            .get_descendants(recurse, include_mixins)
            .map_err(|e| PyException::new_err(format!("{:?}", e)))
            .map(|v| v.into_iter().map(|cv| PyClassView { inner: cv }).collect())
    }

    fn canonical_uri(&self) -> String {
        self.inner.canonical_uri().to_string()
    }
}

#[pymethods]
impl PySlotView {
    #[getter]
    pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    #[getter]
    pub fn definition(&self) -> SlotDefinition {
        self.inner.definition().clone()
    }

    fn range_class(&self) -> Option<PyClassView> {
        self.inner
            .get_range_class()
            .map(|cv| PyClassView { inner: cv })
    }

    fn range_enum(&self) -> Option<EnumDefinition> {
        self.inner.get_range_enum()
    }

    fn is_range_scalar(&self) -> bool {
        self.inner.is_range_scalar()
    }

    fn container_mode(&self) -> String {
        format!("{:?}", self.inner.determine_slot_container_mode())
    }

    fn inline_mode(&self) -> String {
        format!("{:?}", self.inner.determine_slot_inline_mode())
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
    m.add_class::<SlotDefinition>()?;
    m.add_class::<EnumDefinition>()?;
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (source=None))]
fn make_schema_view(source: Option<&Bound<'_, PyAny>>) -> PyResult<PySchemaView> {
    PySchemaView::new(source)
}

/// Python bindings for `linkml_runtime`.
#[pymodule(name = "_native")]
pub fn runtime_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(schemaview_module))?;
    m.add_function(wrap_pyfunction!(make_schema_view, m)?)?;
    m.add_function(wrap_pyfunction!(load_yaml, m)?)?;
    m.add_function(wrap_pyfunction!(load_json, m)?)?;
    m.add_class::<PyLinkMLValue>()?;
    Ok(())
}

#[pyclass(name = "LinkMLValue")]
pub struct PyLinkMLValue {
    value: LinkMLValue,
    sv: Py<PySchemaView>,
}

impl PyLinkMLValue {
    fn new(value: LinkMLValue, sv: Py<PySchemaView>) -> Self {
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
            LinkMLValue::Scalar { slot, .. } => Some(slot.name.clone()),
            LinkMLValue::List { slot, .. } => Some(slot.name.clone()),
            _ => None,
        }
    }

    fn class_name(&self) -> Option<String> {
        match &self.value {
            LinkMLValue::Map { class, .. } => Some(class.def().name.clone()),
            LinkMLValue::Scalar { class: Some(c), .. } => Some(c.def().name.clone()),
            LinkMLValue::List { class: Some(c), .. } => Some(c.def().name.clone()),
            _ => None,
        }
    }

    fn __len__(&self) -> PyResult<usize> {
        Ok(match &self.value {
            LinkMLValue::Scalar { .. } => 0,
            LinkMLValue::List { values, .. } => values.len(),
            LinkMLValue::Map { values, .. } => values.len(),
        })
    }

    fn __getitem__<'py>(
        &self,
        py: Python<'py>,
        key: &Bound<'py, PyAny>,
    ) -> PyResult<PyLinkMLValue> {
        match &self.value {
            LinkMLValue::List { values, .. } => {
                let idx: usize = key.extract()?;
                values
                    .get(idx)
                    .map(|v| PyLinkMLValue::new(v.clone(), self.sv.clone_ref(py)))
                    .ok_or_else(|| PyException::new_err("index out of range"))
            }
            LinkMLValue::Map { values, .. } => {
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
            LinkMLValue::Map { values, .. } => Ok(values.keys().cloned().collect()),
            _ => Ok(Vec::new()),
        }
    }

    fn values<'py>(&self, py: Python<'py>) -> PyResult<Vec<PyLinkMLValue>> {
        match &self.value {
            LinkMLValue::Map { values, .. } => Ok(values
                .values()
                .cloned()
                .map(|v| PyLinkMLValue::new(v, self.sv.clone_ref(py)))
                .collect()),
            LinkMLValue::List { values, .. } => Ok(values
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
        turtle_to_string(
            &self.value,
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
    let class_ref = match class {
        Some(cv) => {
            let bound = cv.bind(py);
            Some(bound.borrow())
        }
        None => None,
    };
    let (text, _) = py_filelike_or_string_to_string(source)?;
    let cv = class_ref
        .ok_or_else(|| PyException::new_err("class not found, please provide a valid class"))?;
    let v = load_yaml_str(&text, rust_sv, cv.as_rust(), &conv)
        .map_err(|e| PyException::new_err(e.to_string()))?;
    Ok(PyLinkMLValue::new(v, sv))
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
    let class_ref = match class {
        Some(cv) => {
            let bound = cv.bind(py);
            Some(bound.borrow())
        }
        None => None,
    };
    let cv = class_ref
        .ok_or_else(|| PyException::new_err("class not found, please provide a valid class"))?;
    let (text, _) = py_filelike_or_string_to_string(source)?;
    let v = load_json_str(&text, rust_sv, cv.as_rust(), &conv)
        .map_err(|e| PyException::new_err(e.to_string()))?;
    Ok(PyLinkMLValue::new(v, sv))
}
