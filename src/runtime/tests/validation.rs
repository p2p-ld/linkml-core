use linkml_runtime::{load_yaml_file, validate};
use linkml_schemaview::identifier::converter_from_schema;
use linkml_schemaview::io::from_yaml;
use linkml_schemaview::schemaview::SchemaView;
use std::path::{Path, PathBuf};

fn info_path(name: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests");
    p.push(name);
    p
}

#[test]
fn validate_personinfo_example1() {
    let schema = from_yaml(Path::new(&info_path("personinfo.yaml"))).unwrap();
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).unwrap();
    let conv = converter_from_schema(&schema);
    let v = load_yaml_file(
        Path::new(&info_path("example_personinfo_data.yaml")),
        &sv,
        None,
        &conv,
    )
    .unwrap();
    assert!(validate(&v).is_ok());
}

#[test]
fn validate_personinfo_example2() {
    let schema = from_yaml(Path::new(&info_path("personinfo.yaml"))).unwrap();
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).unwrap();
    let conv = converter_from_schema(&schema);
    let v = load_yaml_file(
        Path::new(&info_path("example_personinfo_data_2.yaml")),
        &sv,
        None,
        &conv,
    )
    .unwrap();
    assert!(validate(&v).is_ok());
}
