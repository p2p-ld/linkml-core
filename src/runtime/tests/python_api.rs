#![cfg(feature = "python")]

use linkml_runtime::runtime_module;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::path::PathBuf;

fn meta_path() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests");
    p.push("data");
    p.push("meta.yaml");
    p
}

#[test]
fn construct_via_python() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let module = PyModule::new(py, "linkml_runtime").unwrap();
        runtime_module(&module).unwrap();
        let sys = py.import("sys").unwrap();
        let modules = sys.getattr("modules").unwrap();
        let sys_modules = modules.downcast::<PyDict>().unwrap();
        sys_modules.set_item("linkml_runtime", module).unwrap();

        let locals = PyDict::new(py);
        locals
            .set_item("meta_path", meta_path().to_str().unwrap())
            .unwrap();
        pyo3::py_run!(
            py,
            *locals,
            r#"
import linkml_runtime as lr
sv = lr.make_schema_view(meta_path)
unresolved = sv.get_unresolved_schemas()
assert "https://w3id.org/linkml/mappings" in unresolved
"#
        );
    });
}

#[test]
fn definitions_via_python() {
    pyo3::prepare_freethreaded_python();
    let yaml = std::fs::read_to_string(meta_path()).unwrap();
    Python::with_gil(|py| {
        let module = PyModule::new(py, "linkml_runtime").unwrap();
        runtime_module(&module).unwrap();
        let sys = py.import("sys").unwrap();
        let modules = sys.getattr("modules").unwrap();
        let sys_modules = modules.downcast::<PyDict>().unwrap();
        sys_modules.set_item("linkml_runtime", module).unwrap();

        let locals = PyDict::new(py);
        locals.set_item("meta_yaml", &yaml).unwrap();
        pyo3::py_run!(
            py,
            *locals,
            r#"
import linkml_runtime as lr
sv = lr.make_schema_view()
sv.add_schema_str(meta_yaml)
print('schemas', sv.get_unresolved_schemas())
s = sv.get_schema('https://w3id.org/linkml/meta')
print('schema', s)
assert s is not None and s.name == 'meta'
c = sv.get_class_view('linkml:class_definition')
print('class', c)
assert c is not None and c.name == 'class_definition'
"#
        );
    });
}

#[test]
fn iterate_definitions_via_python() {
    pyo3::prepare_freethreaded_python();
    let yaml = std::fs::read_to_string(meta_path()).unwrap();
    Python::with_gil(|py| {
        let module = PyModule::new(py, "linkml_runtime").unwrap();
        runtime_module(&module).unwrap();
        let sys = py.import("sys").unwrap();
        let modules = sys.getattr("modules").unwrap();
        let sys_modules = modules.downcast::<PyDict>().unwrap();
        sys_modules.set_item("linkml_runtime", module).unwrap();

        let locals = PyDict::new(py);
        locals.set_item("meta_yaml", &yaml).unwrap();
        pyo3::py_run!(
            py,
            *locals,
            r#"
import linkml_runtime as lr
sv = lr.make_schema_view()
sv.add_schema_str(meta_yaml)
cls = sv.class_definitions()
assert any(c.name == 'class_definition' for c in cls)
slots = sv.slot_definitions()
assert any(s.name == 'name' for s in slots)
schemas = sv.schema_definitions()
assert any(s.name == 'meta' for s in schemas)
"#
        );
    });
}
