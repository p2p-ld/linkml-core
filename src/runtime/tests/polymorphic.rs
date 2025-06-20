use linkml_runtime::{load_yaml_file, validate};
use schemaview::identifier::{converter_from_schema, Identifier};
use schemaview::io::from_yaml;
use schemaview::schemaview::SchemaView;
use std::path::{Path, PathBuf};

fn data_path(name: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests");
    p.push("data");
    p.push(name);
    p
}

#[test]
fn polymorphism_with_type() {
    let schema = from_yaml(Path::new(&data_path("poly_schema.yaml"))).unwrap();
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).unwrap();
    let conv = converter_from_schema(&schema);
    let class = sv
        .get_class(&Identifier::new("Container"), &conv)
        .unwrap()
        .expect("class not found");
    let v = load_yaml_file(
        Path::new(&data_path("poly_with_type.yaml")),
        &sv,
        Some(&class),
        &conv,
    )
    .unwrap();
    assert!(validate(&v).is_ok());
}

#[test]
fn polymorphism_without_type() {
    let schema = from_yaml(Path::new(&data_path("poly_schema.yaml"))).unwrap();
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).unwrap();
    let conv = converter_from_schema(&schema);
    let class = sv
        .get_class(&Identifier::new("Container"), &conv)
        .unwrap()
        .expect("class not found");
    let v = load_yaml_file(
        Path::new(&data_path("poly_without_type.yaml")),
        &sv,
        Some(&class),
        &conv,
    )
    .unwrap();
    assert!(validate(&v).is_ok());
}

#[test]
fn root_polymorphism_with_type() {
    let schema = from_yaml(Path::new(&data_path("poly_schema.yaml"))).unwrap();
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).unwrap();
    let conv = converter_from_schema(&schema);
    let class = sv
        .get_class(&Identifier::new("Parent"), &conv)
        .unwrap()
        .expect("class not found");
    let v = load_yaml_file(
        Path::new(&data_path("root_with_type.yaml")),
        &sv,
        Some(&class),
        &conv,
    )
    .unwrap();
    assert!(validate(&v).is_ok());
}

#[test]
fn root_polymorphism_without_type() {
    let schema = from_yaml(Path::new(&data_path("poly_schema.yaml"))).unwrap();
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).unwrap();
    let conv = converter_from_schema(&schema);
    let class = sv
        .get_class(&Identifier::new("Parent"), &conv)
        .unwrap()
        .expect("class not found");
    let v = load_yaml_file(
        Path::new(&data_path("root_without_type.yaml")),
        &sv,
        Some(&class),
        &conv,
    )
    .unwrap();
    assert!(validate(&v).is_ok());
}
