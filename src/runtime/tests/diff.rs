use linkml_runtime::{diff, load_yaml_file, patch};
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

fn info_path(name: &str) -> PathBuf {
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
    let src = load_yaml_file(
        Path::new(&data_path("person_valid.yaml")),
        &sv,
        &class,
        &conv,
    )
    .unwrap();
    let tgt = load_yaml_file(
        Path::new(&data_path("person_older.yaml")),
        &sv,
        &class,
        &conv,
    )
    .unwrap();

    let deltas = diff(&src, &tgt, false);
    assert_eq!(deltas.len(), 1);
    // Ensure delta paths are navigable on respective values
    for d in &deltas {
        if d.old.is_some() {
            assert!(src.navigate_path(&d.path).is_some());
        }
        if d.new.is_some() {
            assert!(tgt.navigate_path(&d.path).is_some());
        }
    }

    let patched = patch(&src, &deltas, &sv);
    let patched_json = patched.to_json();
    let target_json = tgt.to_json();
    let src_json = src.to_json();
    assert_ne!(patched_json, target_json);
    assert_eq!(patched_json.get("age"), target_json.get("age"));
    assert_eq!(patched_json.get("internal_id"), src_json.get("internal_id"));
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
    let src = load_yaml_file(
        Path::new(&data_path("person_valid.yaml")),
        &sv,
        &class,
        &conv,
    )
    .unwrap();
    let tgt = load_yaml_file(
        Path::new(&data_path("person_partial.yaml")),
        &sv,
        &class,
        &conv,
    )
    .unwrap();

    let deltas = diff(&src, &tgt, true);
    assert!(deltas.is_empty());
    let patched = patch(&src, &deltas, &sv);
    let patched_json = patched.to_json();
    let src_json = src.to_json();
    assert_eq!(patched_json, src_json);
}

#[test]
fn diff_and_patch_personinfo() {
    let schema = from_yaml(Path::new(&info_path("personinfo.yaml"))).unwrap();
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).unwrap();
    let conv = converter_from_schema(&schema);
    let container = sv
        .get_class(&Identifier::new("Container"), &conv)
        .unwrap()
        .expect("class not found");
    let src = load_yaml_file(
        Path::new(&info_path("example_personinfo_data.yaml")),
        &sv,
        &container,
        &conv,
    )
    .unwrap();
    let tgt = load_yaml_file(
        Path::new(&info_path("example_personinfo_data_2.yaml")),
        &sv,
        &container,
        &conv,
    )
    .unwrap();

    let deltas = diff(&src, &tgt, false);
    assert!(!deltas.is_empty());
    // Ensure delta paths are navigable on respective values, including mapping-list keys
    for d in &deltas {
        if d.old.is_some() {
            assert!(src.navigate_path(&d.path).is_some());
        }
        if d.new.is_some() {
            assert!(tgt.navigate_path(&d.path).is_some());
        }
    }
    let patched = patch(&src, &deltas, &sv);
    assert_eq!(patched.to_json(), tgt.to_json());
}

#[test]
fn personinfo_invalid_fails() {
    let schema = from_yaml(Path::new(&info_path("personinfo.yaml"))).unwrap();
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).unwrap();
    let conv = converter_from_schema(&schema);
    let class = sv
        .get_class(&Identifier::new("Person"), &conv)
        .unwrap()
        .expect("class not found");
    let v = load_yaml_file(
        Path::new(&info_path("example_personinfo_data_invalid.yaml")),
        &sv,
        &class,
        &conv,
    );
    assert!(v.is_err());
}
