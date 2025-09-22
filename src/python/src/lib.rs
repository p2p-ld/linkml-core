use linkml_meta::{ClassDefinition, EnumDefinition, SchemaDefinition, SlotDefinition};
use linkml_runtime::diff::{diff as diff_internal, patch as patch_internal, Delta};
use linkml_runtime::turtle::{turtle_to_string, TurtleOptions};
use linkml_runtime::{load_json_str, load_yaml_str, LinkMLInstance};
use linkml_schemaview::identifier::Identifier;
use linkml_schemaview::io;
use linkml_schemaview::schemaview::SchemaView;
use linkml_schemaview::{classview::ClassView, enumview::EnumView, slotview::SlotView};
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

#[pyclass(name = "EnumView")]
#[derive(Clone)]
pub struct PyEnumView {
    inner: EnumView,
}

impl PyEnumView {
    pub fn as_rust(&self) -> &EnumView {
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
                .map_err(PyException::new_err)?;
        }
        Ok(Self {
            inner: Arc::new(sv),
        })
    }

    fn _get_resolved_schema_imports(&self) -> HashMap<(String, String), String> {
        self.inner._get_resolved_schema_imports()
    }

    fn get_default_prefix_for_schema(&self, schema_id: &str, expand: bool) -> Option<String> {
        self.inner.get_default_prefix_for_schema(schema_id, expand)
    }

    fn add_schema_from_path(&mut self, path: &str) -> PyResult<bool> {
        let schema =
            io::from_yaml(Path::new(path)).map_err(|e| PyException::new_err(e.to_string()))?;
        if let Some(inner) = std::sync::Arc::get_mut(&mut self.inner) {
            inner.add_schema(schema).map_err(PyException::new_err)
        } else {
            Err(PyException::new_err("SchemaView already shared"))
        }
    }

    fn add_schema_str(&mut self, data: &str) -> PyResult<bool> {
        let deser = serde_yml::Deserializer::from_str(data);
        let schema: SchemaDefinition = serde_path_to_error::deserialize(deser)
            .map_err(|e| PyException::new_err(e.to_string()))?;
        if let Some(inner) = std::sync::Arc::get_mut(&mut self.inner) {
            inner.add_schema(schema).map_err(PyException::new_err)
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
        if let Some(inner) = std::sync::Arc::get_mut(&mut self.inner) {
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

    fn get_enum_view(&self, id: &str) -> PyResult<Option<PyEnumView>> {
        let conv = self.inner.converter();
        Ok(self
            .inner
            .get_enum(&Identifier::new(id), &conv)
            .map_err(|e| PyException::new_err(format!("{:?}", e)))?
            .map(|ev| PyEnumView { inner: ev }))
    }

    fn schema_ids(&self) -> Vec<String> {
        self.inner
            .all_schema_definitions()
            .map(|x| x.0.clone())
            .collect()
    }

    fn get_class_ids(&self) -> Vec<String> {
        self.inner.get_class_ids()
        /*
        let mut ids: HashSet<String> = HashSet::new();
        for (_, schema) in self.inner.iter_schemas() {
            if let Some(classes) = &schema.classes {
                classes.iter().map(|c| self.inner.get_uri(&schema.id, c.0)).for_each(|uri| {
                    ids.insert(uri.to_string());
                });
            }
        }
        ids.into_iter().collect()*/
    }

    fn get_slot_ids(&self) -> Vec<String> {
        self.inner.get_slot_ids() /*
                                  let mut ids: HashSet<String> = HashSet::new();
                                  for (_, schema) in self.inner.iter_schemas() {
                                      if let Some(slots) = &schema.slot_definitions {
                                          slots.iter().map(|c| self.inner.get_uri(&schema.id, c.0)).for_each(|uri| {
                                              ids.insert(uri.to_string());
                                          });
                                      }
                                  }
                                  ids.into_iter().collect()*/
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "SchemaView(n_schemas={}, n_classes={}, n_slots={})",
            self.inner.all_schema_definitions().count(),
            self.inner.get_class_ids().len(),
            self.inner.get_slot_ids().len()
        ))
    }

    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
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

    fn schema_id(&self) -> String {
        self.inner.schema_id().to_string()
    }

    fn canonical_uri(&self) -> String {
        self.inner.canonical_uri().to_string()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "ClassView(name='{}', slots={})",
            self.inner.name(),
            self.inner.slots().len()
        ))
    }

    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
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

    fn range_enum(&self) -> Option<PyEnumView> {
        self.inner
            .get_range_enum()
            .map(|ev| PyEnumView { inner: ev })
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

    fn __repr__(&self) -> PyResult<String> {
        let range = self
            .inner
            .definition()
            .range
            .clone()
            .unwrap_or_else(|| "None".to_string());
        Ok(format!(
            "SlotView(name='{}', range='{}')",
            self.inner.name.clone(),
            range
        ))
    }

    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }
}

#[pymethods]
impl PyEnumView {
    #[getter]
    pub fn name(&self) -> String {
        self.inner.name().to_string()
    }

    #[getter]
    pub fn definition(&self) -> EnumDefinition {
        self.inner.definition().clone()
    }

    fn permissible_value_keys(&self) -> PyResult<Vec<String>> {
        self.inner
            .permissible_value_keys()
            .cloned()
            .map_err(|e| PyException::new_err(format!("{:?}", e)))
    }

    fn schema_id(&self) -> String {
        self.inner.schema_id().to_string()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "EnumView(name='{}', n_values={})",
            self.inner.name(),
            self.inner
                .permissible_value_keys()
                .map(|v| v.len())
                .unwrap_or(0)
        ))
    }

    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }
}

#[pymodule(name = "linkml_schemaview")]
pub fn schemaview_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<PySchemaView>()?;
    m.add_class::<PyClassView>()?;
    m.add_class::<PySlotView>()?;
    m.add_class::<PyEnumView>()?;
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
    m.add_function(wrap_pyfunction!(py_diff, m)?)?;
    m.add_function(wrap_pyfunction!(py_patch, m)?)?;
    m.add_function(wrap_pyfunction!(py_to_turtle, m)?)?;
    m.add_class::<PyLinkMLInstance>()?;
    Ok(())
}

#[pyclass(name = "LinkMLInstance")]
pub struct PyLinkMLInstance {
    value: LinkMLInstance,
    sv: Py<PySchemaView>,
}

impl PyLinkMLInstance {
    fn new(value: LinkMLInstance, sv: Py<PySchemaView>) -> Self {
        Self { value, sv }
    }
}

fn json_value_to_py(py: Python<'_>, v: &JsonValue) -> PyObject {
    let s = serde_json::to_string(v).unwrap();
    let json_mod = PyModule::import(py, "json").unwrap();
    json_mod.call_method1("loads", (s,)).unwrap().unbind()
}

impl Clone for PyLinkMLInstance {
    fn clone(&self) -> Self {
        Python::with_gil(|py| Self {
            value: self.value.clone(),
            sv: self.sv.clone_ref(py),
        })
    }
}

#[pymethods]
impl PyLinkMLInstance {
    /// Semantic equality per LinkML Instances spec.
    /// Compares this value with another `LinkMLInstance`.
    #[pyo3(signature = (other, treat_missing_as_null = false))]
    fn equals(&self, other: &PyLinkMLInstance, treat_missing_as_null: bool) -> bool {
        self.value.equals(&other.value, treat_missing_as_null)
    }
    #[getter]
    fn slot_name(&self) -> Option<String> {
        match &self.value {
            LinkMLInstance::Scalar { slot, .. } => Some(slot.name.clone()),
            LinkMLInstance::List { slot, .. } => Some(slot.name.clone()),
            LinkMLInstance::Null { slot, .. } => Some(slot.name.clone()),
            _ => None,
        }
    }

    #[getter]
    fn kind(&self) -> String {
        match &self.value {
            LinkMLInstance::Scalar { .. } => "scalar".to_string(),
            LinkMLInstance::Null { .. } => "null".to_string(),
            LinkMLInstance::List { .. } => "list".to_string(),
            LinkMLInstance::Mapping { .. } => "mapping".to_string(),
            LinkMLInstance::Object { .. } => "object".to_string(),
        }
    }

    #[getter]
    fn node_id(&self) -> u64 {
        self.value.node_id()
    }

    #[getter]
    fn slot_definition(&self) -> Option<SlotDefinition> {
        match &self.value {
            LinkMLInstance::Scalar { slot, .. } => Some(slot.definition().clone()),
            LinkMLInstance::List { slot, .. } => Some(slot.definition().clone()),
            LinkMLInstance::Null { slot, .. } => Some(slot.definition().clone()),
            _ => None,
        }
    }

    #[getter]
    fn class_definition(&self) -> Option<ClassDefinition> {
        match &self.value {
            LinkMLInstance::Object { class, .. } => Some(class.def().clone()),
            LinkMLInstance::Scalar { class: Some(c), .. } => Some(c.def().clone()),
            LinkMLInstance::List { class: Some(c), .. } => Some(c.def().clone()),
            LinkMLInstance::Null { class: Some(c), .. } => Some(c.def().clone()),
            _ => None,
        }
    }

    #[getter]
    fn class_name(&self) -> Option<String> {
        match &self.value {
            LinkMLInstance::Object { class, .. } => Some(class.def().name.clone()),
            LinkMLInstance::Scalar { class: Some(c), .. } => Some(c.def().name.clone()),
            LinkMLInstance::List { class: Some(c), .. } => Some(c.def().name.clone()),
            LinkMLInstance::Null { class: Some(c), .. } => Some(c.def().name.clone()),
            _ => None,
        }
    }

    fn __len__(&self) -> PyResult<usize> {
        Ok(match &self.value {
            LinkMLInstance::Scalar { .. } => 0,
            LinkMLInstance::Null { .. } => 0,
            LinkMLInstance::List { values, .. } => values.len(),
            LinkMLInstance::Mapping { values, .. } => values.len(),
            LinkMLInstance::Object { values, .. } => values.len(),
        })
    }

    fn __getitem__<'py>(
        &self,
        py: Python<'py>,
        key: &Bound<'py, PyAny>,
    ) -> PyResult<PyLinkMLInstance> {
        match &self.value {
            LinkMLInstance::List { values, .. } => {
                let idx: usize = key.extract()?;
                values
                    .get(idx)
                    .map(|v| PyLinkMLInstance::new(v.clone(), self.sv.clone_ref(py)))
                    .ok_or_else(|| PyException::new_err("index out of range"))
            }
            LinkMLInstance::Object { values, .. } => {
                let k: String = key.extract()?;
                values
                    .get(&k)
                    .map(|v| PyLinkMLInstance::new(v.clone(), self.sv.clone_ref(py)))
                    .ok_or_else(|| PyException::new_err("key not found"))
            }
            LinkMLInstance::Mapping { values, .. } => {
                let k: String = key.extract()?;
                values
                    .get(&k)
                    .map(|v| PyLinkMLInstance::new(v.clone(), self.sv.clone_ref(py)))
                    .ok_or_else(|| PyException::new_err("key not found"))
            }
            _ => Err(PyException::new_err("not indexable")),
        }
    }

    /// Navigate by a path of strings (map keys or list indices).
    /// Returns a new LinkMLInstance if found, otherwise None.
    #[pyo3(name = "navigate")]
    fn py_navigate<'py>(
        &self,
        py: Python<'py>,
        path: &Bound<'py, PyAny>,
    ) -> PyResult<Option<PyLinkMLInstance>> {
        // Expect any iterable of strings
        let path_vec: Vec<String> = path
            .extract()
            .map_err(|_| PyException::new_err("path must be a sequence of strings"))?;
        if let Some(found) = self.value.navigate_path(&path_vec) {
            Ok(Some(PyLinkMLInstance::new(
                found.clone(),
                self.sv.clone_ref(py),
            )))
        } else {
            Ok(None)
        }
    }

    fn keys(&self) -> PyResult<Vec<String>> {
        match &self.value {
            LinkMLInstance::Object { values, .. } => Ok(values.keys().cloned().collect()),
            LinkMLInstance::Mapping { values, .. } => Ok(values.keys().cloned().collect()),
            _ => Ok(Vec::new()),
        }
    }

    fn values<'py>(&self, py: Python<'py>) -> PyResult<Vec<PyLinkMLInstance>> {
        match &self.value {
            LinkMLInstance::Object { values, .. } => Ok(values
                .values()
                .cloned()
                .map(|v| PyLinkMLInstance::new(v, self.sv.clone_ref(py)))
                .collect()),
            LinkMLInstance::Mapping { values, .. } => Ok(values
                .values()
                .cloned()
                .map(|v| PyLinkMLInstance::new(v, self.sv.clone_ref(py)))
                .collect()),
            LinkMLInstance::List { values, .. } => Ok(values
                .iter()
                .cloned()
                .map(|v| PyLinkMLInstance::new(v, self.sv.clone_ref(py)))
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

    fn __repr__(&self) -> PyResult<String> {
        Ok(match &self.value {
            LinkMLInstance::Scalar { value, slot, .. } => {
                format!(
                    "LinkMLInstance.Scalar(slot='{}', value={})",
                    slot.name, value
                )
            }
            LinkMLInstance::Null { slot, .. } => {
                format!("LinkMLInstance.Null(slot='{}')", slot.name)
            }
            LinkMLInstance::List { values, slot, .. } => {
                format!(
                    "LinkMLInstance.List(slot='{}', len={})",
                    slot.name,
                    values.len()
                )
            }
            LinkMLInstance::Mapping { values, slot, .. } => {
                format!(
                    "LinkMLInstance.Mapping(slot='{}', keys={:?})",
                    slot.name,
                    values.keys().collect::<Vec<&String>>()
                )
            }
            LinkMLInstance::Object { values, class, .. } => {
                let keys: Vec<&String> = values.keys().collect();
                format!(
                    "LinkMLInstance.Object(class='{}', keys={:?})",
                    class.def().name.clone(),
                    keys
                )
            }
        })
    }

    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }
}

#[pyfunction]
fn load_yaml(
    py: Python<'_>,
    source: &Bound<'_, PyAny>,
    sv: Py<PySchemaView>,
    class: Option<Py<PyClassView>>,
) -> PyResult<PyLinkMLInstance> {
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
    Ok(PyLinkMLInstance::new(v, sv))
}

#[pyfunction]
fn load_json(
    py: Python<'_>,
    source: &Bound<'_, PyAny>,
    sv: Py<PySchemaView>,
    class: Option<Py<PyClassView>>,
) -> PyResult<PyLinkMLInstance> {
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
    Ok(PyLinkMLInstance::new(v, sv))
}

#[pyfunction(name = "diff", signature = (source, target, treat_missing_as_null=None))]
fn py_diff(
    py: Python<'_>,
    source: &PyLinkMLInstance,
    target: &PyLinkMLInstance,
    treat_missing_as_null: Option<bool>,
) -> PyResult<PyObject> {
    let deltas = diff_internal(
        &source.value,
        &target.value,
        treat_missing_as_null.unwrap_or(false),
    );
    let vals: Vec<JsonValue> = deltas
        .iter()
        .map(|d| serde_json::to_value(d).unwrap())
        .collect();
    Ok(json_value_to_py(py, &JsonValue::Array(vals)))
}

#[pyfunction(name = "patch", signature = (source, deltas, treat_missing_as_null = true, ignore_no_ops = true))]
fn py_patch(
    py: Python<'_>,
    source: &PyLinkMLInstance,
    deltas: &Bound<'_, PyAny>,
    treat_missing_as_null: bool,
    ignore_no_ops: bool,
) -> PyResult<PyObject> {
    let json_mod = PyModule::import(py, "json")?;
    let deltas_str: String = json_mod.call_method1("dumps", (deltas,))?.extract()?;
    let deltas_vec: Vec<Delta> =
        serde_json::from_str(&deltas_str).map_err(|e| PyException::new_err(e.to_string()))?;
    let sv_ref = source.sv.bind(py).borrow();
    let rust_sv = sv_ref.as_rust();
    let (new_value, trace) = patch_internal(
        &source.value,
        &deltas_vec,
        rust_sv,
        linkml_runtime::diff::PatchOptions {
            ignore_no_ops,
            treat_missing_as_null,
        },
    )
    .map_err(|e| PyException::new_err(e.to_string()))?;
    let trace_json = serde_json::json!({
        "added": trace.added,
        "deleted": trace.deleted,
        "updated": trace.updated,
    });
    let py_val = PyLinkMLInstance::new(new_value, source.sv.clone_ref(py));
    let dict = pyo3::types::PyDict::new(py);
    dict.set_item("value", Py::new(py, py_val)?)?;
    dict.set_item("trace", json_value_to_py(py, &trace_json))?;
    Ok(dict.into_any().unbind())
}

#[pyfunction(name = "to_turtle", signature = (value, skolem=None))]
fn py_to_turtle(
    py: Python<'_>,
    value: &PyLinkMLInstance,
    skolem: Option<bool>,
) -> PyResult<String> {
    value.as_turtle(py, skolem)
}
