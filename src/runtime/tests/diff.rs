use linkml_runtime::{load_yaml_file, diff, patch};
use linkml_schemaview::identifier::{converter_from_schema, Identifier};
use linkml_schemaview::io::from_yaml;
use linkml_schemaview::schemaview::SchemaView;
use std::path::{Path, PathBuf};

fn data_path(name: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests");
    p.push("data");
    p.push(name);
    p
}

#[test]
fn diff_and_patch_person() {
    let schema = from_yaml(Path::new(&data_path("schema.yaml"))).unwrap();
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).unwrap();
    let conv = converter_from_schema(&schema);
    let class = sv
        .get_class(&Identifier::new("Person"), &conv)
        .unwrap()
        .expect("class not found");
    let src = load_yaml_file(Path::new(&data_path("person_valid.yaml")), &sv, Some(&class), &conv).unwrap();
    let tgt = load_yaml_file(Path::new(&data_path("person_older.yaml")), &sv, Some(&class), &conv).unwrap();

    let deltas = diff(&src, &tgt, false);
    assert_eq!(deltas.len(), 1);

    let patched = patch(&src, &deltas, &sv);
    let patched_json = patched.to_json();
    let target_json = tgt.to_json();
    assert_eq!(patched_json, target_json);
}

#[test]
fn diff_ignore_missing_target() {
    let schema = from_yaml(Path::new(&data_path("schema.yaml"))).unwrap();
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).unwrap();
    let conv = converter_from_schema(&schema);
    let class = sv
        .get_class(&Identifier::new("Person"), &conv)
        .unwrap()
        .expect("class not found");
    let src = load_yaml_file(Path::new(&data_path("person_valid.yaml")), &sv, Some(&class), &conv).unwrap();
    let tgt = load_yaml_file(Path::new(&data_path("person_partial.yaml")), &sv, Some(&class), &conv).unwrap();

    let deltas = diff(&src, &tgt, true);
    assert!(deltas.is_empty());
    let patched = patch(&src, &deltas, &sv);
    let patched_json = patched.to_json();
    let src_json = src.to_json();
    assert_eq!(patched_json, src_json);
}
