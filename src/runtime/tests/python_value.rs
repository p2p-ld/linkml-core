#![cfg(feature = "python")]

use linkml_runtime::runtime_module;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::path::PathBuf;

fn data_path(name: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests");
    p.push("data");
    p.push(name);
    p
}

#[test]
fn load_value_via_python() {
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
            .set_item("schema_path", data_path("schema.yaml").to_str().unwrap())
            .unwrap();
        locals
            .set_item(
                "person_path",
                data_path("person_valid.yaml").to_str().unwrap(),
            )
            .unwrap();

        pyo3::py_run!(
            py,
            *locals,
            r#"
import linkml_runtime as lr
sv = lr.make_schema_view(schema_path)
cls = sv.get_class_view('Person')
value = lr.load_yaml(person_path, sv, cls)
assert value.class_name == 'Person'
assert value.class_definition.name == 'Person'
assert value['name'].slot_name == 'name'
assert value['name'].slot_definition.name == 'name'
assert value['name'].class_definition.name == 'Person'
assert value['name'].as_python() == 'Alice'
"#
        );
    });
}
