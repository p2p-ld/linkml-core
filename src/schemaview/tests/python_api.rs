#![cfg(feature = "python")]

use pyo3::prelude::*;
use pyo3::types::PyDict;
use linkml_schemaview::schemaview_module;
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
        let module = PyModule::new(py, "linkml_schemaview").unwrap();
        schemaview_module(&module).unwrap();
        let sys = py.import("sys").unwrap();
        let modules = sys.getattr("modules").unwrap();
        let sys_modules = modules.downcast::<PyDict>().unwrap();
        sys_modules.set_item("linkml_schemaview", module).unwrap();

        let locals = PyDict::new(py);
        locals
            .set_item("meta_path", meta_path().to_str().unwrap())
            .unwrap();
        pyo3::py_run!(
            py,
            *locals,
            r#"
import linkml_schemaview as schemaview
sv = schemaview.SchemaView(meta_path)
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
        let module = PyModule::new(py, "linkml_schemaview").unwrap();
        schemaview_module(&module).unwrap();
        let sys = py.import("sys").unwrap();
        let modules = sys.getattr("modules").unwrap();
        let sys_modules = modules.downcast::<PyDict>().unwrap();
        sys_modules.set_item("linkml_schemaview", module).unwrap();

        let locals = PyDict::new(py);
        locals.set_item("meta_yaml", &yaml).unwrap();
        pyo3::py_run!(
            py,
            *locals,
            r#"
import linkml_schemaview as schemaview
sv = schemaview.SchemaView()
sv.add_schema_str(meta_yaml)
print('schemas', sv.get_unresolved_schemas())
s = sv.get_schema('https://w3id.org/linkml/meta')
print('schema', s)
assert s is not None and s.name == 'meta'
c = sv.get_class('linkml:class_definition')
print('class', c)
assert c is not None and c.name == 'class_definition'
"#
        );
    });
}
