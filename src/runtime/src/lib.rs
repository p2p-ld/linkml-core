use curies::Converter;
use linkml_schemaview::identifier::Identifier;
use linkml_schemaview::schemaview::{ClassView, SchemaView, SlotView};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub mod turtle;
use crate::turtle::{turtle_to_string, TurtleOptions};
#[cfg(feature = "python")]
pub use linkml_schemaview::{PyClassView, PySchemaView, PySlotView};
pub enum LinkMLValue<'a> {
    Scalar {
        value: JsonValue,
        slot: Option<SlotView<'a>>,
        sv: &'a SchemaView,
    },
    List {
        values: Vec<LinkMLValue<'a>>,
        slot: Option<SlotView<'a>>,
        sv: &'a SchemaView,
    },
    Map {
        values: HashMap<String, LinkMLValue<'a>>,
        class: Option<ClassView<'a>>,
        sv: &'a SchemaView,
    },
}

impl<'a> LinkMLValue<'a> {
    fn select_class(
        map: &serde_json::Map<String, JsonValue>,
        base: &ClassView<'a>,
        sv: &'a SchemaView,
        conv: &Converter,
    ) -> ClassView<'a> {
        let descendants = match base.get_descendants(conv, true) {
            Ok(d) => d,
            Err(_) => Vec::new(),
        };
        let mut cand_refs: Vec<&ClassView<'a>> = vec![base];
        for d in &descendants {
            cand_refs.push(d);
        }

        let mut preferred: Option<&ClassView<'a>> = None;
        if let Some(ts) = base.slots().iter().find(|s| {
            s.definitions
                .iter()
                .rev()
                .any(|d| d.designates_type.unwrap_or(false))
        }) {
            if let Some(JsonValue::String(tv)) = map.get(&ts.name) {
                if let Some(def) = ts
                    .definitions
                    .iter()
                    .rev()
                    .find(|d| d.designates_type.unwrap_or(false))
                {
                    for c in &cand_refs {
                        if let Ok(vals) = c.get_accepted_type_designator_values(def, conv) {
                            if vals.iter().any(|v| v.to_string() == *tv) {
                                preferred = Some(*c);
                                break;
                            }
                        }
                    }
                }
            }
        }
        if let Some(p) = preferred {
            return p.clone();
        }

        for c in &cand_refs {
            let tmp = LinkMLValue::from_json(
                JsonValue::Object(map.clone()),
                Some(*c),
                None,
                sv,
                conv,
                false,
            );
            if validate(&tmp).is_ok() {
                return (*c).clone();
            }
        }

        base.clone()
    }
    fn from_json(
        value: JsonValue,
        class: Option<&'a ClassView<'a>>,
        slot: Option<SlotView<'a>>,
        sv: &'a SchemaView,
        conv: &Converter,
        polymorphic: bool,
    ) -> Self {
        match value {
            JsonValue::Array(arr) => {
                let values = arr
                    .into_iter()
                    .map(|v| LinkMLValue::from_json(v, None, None, sv, conv, true))
                    .collect();
                LinkMLValue::List { values, slot, sv }
            }
            JsonValue::Object(map) => {
                if !polymorphic && slot.is_none() {
                    if let Some(cls) = class {
                        let mut values = HashMap::new();
                        for (k, v) in map.into_iter() {
                            let slot_ref = cls.slots().iter().find(|s| s.name == k).cloned();
                            values.insert(
                                k,
                                LinkMLValue::from_json(v, None, slot_ref, sv, conv, true),
                            );
                        }
                        return LinkMLValue::Map {
                            values,
                            class: Some(cls.clone()),
                            sv,
                        };
                    }
                }

                // determine base class
                let base_class: Option<ClassView<'a>> = match class {
                    Some(c) => Some(c.clone()),
                    None => slot.and_then(|s| {
                        s.definitions
                            .iter()
                            .rev()
                            .find_map(|d| d.range.as_deref())
                            .and_then(|r| sv.get_class(&Identifier::new(r), conv).ok().flatten())
                    }),
                };

                let chosen: Option<ClassView<'a>> = base_class
                    .as_ref()
                    .map(|b| Self::select_class(&map, b, sv, conv));

                let mut values = HashMap::new();
                for (k, v) in map.into_iter() {
                    let slot_tmp: Option<SlotView<'a>> = chosen
                        .as_ref()
                        .and_then(|cv| cv.slots().iter().find(|s| s.name == k))
                        .cloned();
                    values.insert(k, LinkMLValue::from_json(v, None, slot_tmp, sv, conv, true));
                }
                LinkMLValue::Map {
                    values,
                    class: chosen,
                    sv,
                }
            }
            other => LinkMLValue::Scalar {
                value: other,
                slot,
                sv,
            },
        }
    }
}

pub fn load_yaml_file<'a>(
    path: &Path,
    sv: &'a SchemaView,
    class: Option<&'a ClassView<'a>>,
    conv: &Converter,
) -> Result<LinkMLValue<'a>, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    load_yaml_str(&text, sv, class, conv)
}

pub fn load_yaml_str<'a>(
    data: &str,
    sv: &'a SchemaView,
    class: Option<&'a ClassView<'a>>,
    conv: &Converter,
) -> Result<LinkMLValue<'a>, Box<dyn std::error::Error>> {
    let value: serde_yaml::Value = serde_yaml::from_str(data)?;
    let json = serde_json::to_value(value)?;
    Ok(LinkMLValue::from_json(json, class, None, sv, conv, true))
}

pub fn load_json_file<'a>(
    path: &Path,
    sv: &'a SchemaView,
    class: Option<&'a ClassView<'a>>,
    conv: &Converter,
) -> Result<LinkMLValue<'a>, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    load_json_str(&text, sv, class, conv)
}

pub fn load_json_str<'a>(
    data: &str,
    sv: &'a SchemaView,
    class: Option<&'a ClassView<'a>>,
    conv: &Converter,
) -> Result<LinkMLValue<'a>, Box<dyn std::error::Error>> {
    let value: JsonValue = serde_json::from_str(data)?;
    Ok(LinkMLValue::from_json(value, class, None, sv, conv, true))
}

fn validate_inner<'a>(value: &LinkMLValue<'a>) -> Result<(), String> {
    match value {
        LinkMLValue::Scalar { .. } => Ok(()),
        LinkMLValue::List { values, .. } => {
            for v in values {
                validate_inner(v)?;
            }
            Ok(())
        }
        LinkMLValue::Map { values, class, .. } => {
            if let Some(cv) = class.as_ref() {
                for (k, v) in values {
                    if cv.slots().iter().all(|s| s.name != *k) {
                        return Err(format!("unknown slot `{}`", k));
                    }
                    validate_inner(v)?;
                }
            } else {
                for v in values.values() {
                    validate_inner(v)?;
                }
            }
            Ok(())
        }
    }
}

pub fn validate<'a>(value: &LinkMLValue<'a>) -> Result<(), String> {
    validate_inner(value)
}

#[cfg(feature = "python")]
use pyo3::exceptions::PyException;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use pyo3::types::PyAnyMethods;
#[cfg(feature = "python")]
use pyo3::types::{PyAny, PyModule};
use pyo3::Bound;
#[cfg(feature = "python")]
use pyo3::{wrap_pyfunction, wrap_pymodule};
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
#[cfg(feature = "python")]
use linkml_schemaview::schemaview_module;

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (source=None))]
fn make_schema_view(source: Option<&Bound<'_, PyAny>>) -> PyResult<PySchemaView> {
    PySchemaView::new(source)
}

/// Python bindings for `linkml_runtime`.
#[cfg(feature = "python")]
#[pymodule(name = "linkml_runtime")]
pub fn runtime_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(schemaview_module))?;
    m.add_function(wrap_pyfunction!(make_schema_view, m)?)?;
    m.add_function(wrap_pyfunction!(load_yaml, m)?)?;
    m.add_function(wrap_pyfunction!(load_json, m)?)?;
    m.add_class::<PyLinkMLValue>()?;
    Ok(())
}

#[cfg(feature = "python")]
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

#[cfg(feature = "python")]
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
        fn inner<'a>(
            v: &LinkMLValueOwned,
            sv: &'a SchemaView,
            conv: &Converter,
        ) -> LinkMLValue<'a> {
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

#[cfg(feature = "python")]
#[pyclass(name = "LinkMLValue")]
pub struct PyLinkMLValue {
    value: LinkMLValueOwned,
    sv: Py<PySchemaView>,
}

#[cfg(feature = "python")]
impl PyLinkMLValue {
    fn new(value: LinkMLValueOwned, sv: Py<PySchemaView>) -> Self {
        Self { value, sv }
    }
}

#[cfg(feature = "python")]
fn json_value_to_py(py: Python<'_>, v: &JsonValue) -> PyObject {
    let s = serde_json::to_string(v).unwrap();
    let json_mod = PyModule::import(py, "json").unwrap();
    json_mod.call_method1("loads", (s,)).unwrap().unbind()
}

#[cfg(feature = "python")]
impl Clone for PyLinkMLValue {
    fn clone(&self) -> Self {
        Python::with_gil(|py| Self {
            value: self.value.clone(),
            sv: self.sv.clone_ref(py),
        })
    }
}

#[cfg(feature = "python")]
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

#[cfg(feature = "python")]
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
    let text = py_any_to_string(source)?;
    let v = load_yaml_str(&text, rust_sv, class_view.as_ref(), &conv)
        .map_err(|e| PyException::new_err(e.to_string()))?;
    Ok(PyLinkMLValue::new(LinkMLValueOwned::from_linkml(&v), sv))
}

#[cfg(feature = "python")]
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
    let text = py_any_to_string(source)?;
    let v = load_json_str(&text, rust_sv, class_view.as_ref(), &conv)
        .map_err(|e| PyException::new_err(e.to_string()))?;
    Ok(PyLinkMLValue::new(LinkMLValueOwned::from_linkml(&v), sv))
}
