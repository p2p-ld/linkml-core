#![cfg(feature = "python")]

use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::path::PathBuf;
use schemaview::schemaview_module;

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
        let module = PyModule::new(py, "schemaview").unwrap();
        schemaview_module(&module).unwrap();
        let sys = py.import("sys").unwrap();
        let modules = sys.getattr("modules").unwrap();
        let sys_modules = modules.downcast::<PyDict>().unwrap();
        sys_modules.set_item("schemaview", module).unwrap();

        let locals = PyDict::new(py);
        locals.set_item("meta_path", meta_path().to_str().unwrap()).unwrap();
        pyo3::py_run!(py, *locals, r#"
import schemaview
sv = schemaview.SchemaView(meta_path)
unresolved = sv.get_unresolved_schemas()
assert "https://w3id.org/linkml/mappings" in unresolved
"#);
    });
}
