use linkml_runtime::{diff, load_json_str, load_yaml_file, patch};
use linkml_schemaview::identifier::{converter_from_schema, Identifier};
use linkml_schemaview::io::from_yaml;
use linkml_schemaview::schemaview::SchemaView;
use serde_json::Value as JsonValue;
use std::path::{Path, PathBuf};

fn data_path(name: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests");
    p.push("data");
    p.push(name);
    p
}

#[test]
fn single_inlined_object_identifier_change_is_replacement() {
    // Use personinfo schema; diagnosis is an inlined object with identifier (via NamedThing)
    let schema = from_yaml(Path::new(&data_path("personinfo.yaml"))).unwrap();
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).unwrap();
    let conv = converter_from_schema(&schema);
    let container = sv
        .get_class(&Identifier::new("Container"), &conv)
        .unwrap()
        .expect("class not found");

    let src = load_yaml_file(
        Path::new(&data_path("example_personinfo_data.yaml")),
        &sv,
        &container,
        &conv,
    )
    .unwrap();

    // Modify diagnosis.id of the first medical history event for P:002
    let mut tgt_json = src.to_json();
    if let JsonValue::Object(ref mut root) = tgt_json {
        if let Some(JsonValue::Array(objects)) = root.get_mut("objects") {
            if let Some(JsonValue::Object(p2)) = objects.get_mut(2) {
                if let Some(JsonValue::Array(mh)) = p2.get_mut("has_medical_history") {
                    if let Some(JsonValue::Object(ev0)) = mh.get_mut(0) {
                        if let Some(JsonValue::Object(diag)) = ev0.get_mut("diagnosis") {
                            diag.insert(
                                "id".to_string(),
                                JsonValue::String("CODE:D9999".to_string()),
                            );
                        }
                    }
                }
            }
        }
    }
    let tgt = load_json_str(
        &serde_json::to_string(&tgt_json).unwrap(),
        &sv,
        &container,
        &conv,
    )
    .unwrap();

    let deltas = diff(&src, &tgt, false);
    // Expect a single replacement at the diagnosis object path
    assert_eq!(deltas.len(), 1);
    let d = &deltas[0];
    assert_eq!(
        d.path,
        vec![
            "objects".to_string(),
            "2".to_string(),
            "has_medical_history".to_string(),
            "0".to_string(),
            "diagnosis".to_string()
        ]
    );
    assert!(d.old.is_some() && d.new.is_some());

    // Patch should yield target
    let (patched, _trace) = patch(&src, &deltas, &sv).unwrap();
    assert_eq!(patched.to_json(), tgt.to_json());
}

#[test]
fn single_inlined_object_non_identifier_change_is_field_delta() {
    let schema = from_yaml(Path::new(&data_path("personinfo.yaml"))).unwrap();
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).unwrap();
    let conv = converter_from_schema(&schema);
    let container = sv
        .get_class(&Identifier::new("Container"), &conv)
        .unwrap()
        .expect("class not found");

    let src = load_yaml_file(
        Path::new(&data_path("example_personinfo_data.yaml")),
        &sv,
        &container,
        &conv,
    )
    .unwrap();

    // Modify diagnosis.name only
    let mut tgt_json = src.to_json();
    if let JsonValue::Object(ref mut root) = tgt_json {
        if let Some(JsonValue::Array(objects)) = root.get_mut("objects") {
            if let Some(JsonValue::Object(p2)) = objects.get_mut(2) {
                if let Some(JsonValue::Array(mh)) = p2.get_mut("has_medical_history") {
                    if let Some(JsonValue::Object(ev0)) = mh.get_mut(0) {
                        if let Some(JsonValue::Object(diag)) = ev0.get_mut("diagnosis") {
                            diag.insert(
                                "name".to_string(),
                                JsonValue::String("new name".to_string()),
                            );
                        }
                    }
                }
            }
        }
    }
    let tgt = load_json_str(
        &serde_json::to_string(&tgt_json).unwrap(),
        &sv,
        &container,
        &conv,
    )
    .unwrap();

    let deltas = diff(&src, &tgt, false);
    assert!(deltas.iter().any(|d| d.path
        == vec![
            "objects".to_string(),
            "2".to_string(),
            "has_medical_history".to_string(),
            "0".to_string(),
            "diagnosis".to_string(),
            "name".to_string()
        ]));
    // Must not collapse to whole-object replacement here
    assert!(!deltas.iter().any(|d| d.path
        == vec![
            "objects".to_string(),
            "2".to_string(),
            "has_medical_history".to_string(),
            "0".to_string(),
            "diagnosis".to_string()
        ]));

    let (patched, _trace) = patch(&src, &deltas, &sv).unwrap();
    assert_eq!(patched.to_json(), tgt.to_json());
}

#[test]
fn list_inlined_object_identifier_change_is_replacement() {
    let schema = from_yaml(Path::new(&data_path("personinfo.yaml"))).unwrap();
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).unwrap();
    let conv = converter_from_schema(&schema);
    let container = sv
        .get_class(&Identifier::new("Container"), &conv)
        .unwrap()
        .expect("class not found");

    let src = load_yaml_file(
        Path::new(&data_path("example_personinfo_data.yaml")),
        &sv,
        &container,
        &conv,
    )
    .unwrap();

    // Change the id of the third object (P:002)
    let mut tgt_json = src.to_json();
    if let JsonValue::Object(ref mut root) = tgt_json {
        if let Some(JsonValue::Array(objects)) = root.get_mut("objects") {
            if let Some(JsonValue::Object(p2)) = objects.get_mut(2) {
                p2.insert("id".to_string(), JsonValue::String("P:099".to_string()));
            }
        }
    }
    let tgt = load_json_str(
        &serde_json::to_string(&tgt_json).unwrap(),
        &sv,
        &container,
        &conv,
    )
    .unwrap();

    let deltas = diff(&src, &tgt, false);
    // Expect a single replacement at the list item path
    assert!(deltas
        .iter()
        .any(|d| d.path == vec!["objects".to_string(), "2".to_string()]));
    assert!(!deltas
        .iter()
        .any(|d| d.path == vec!["objects".to_string(), "2".to_string(), "id".to_string()]));

    let (patched, _trace) = patch(&src, &deltas, &sv).unwrap();
    assert_eq!(patched.to_json(), tgt.to_json());
}

#[test]
fn mapping_inlined_identifier_change_is_add_delete() {
    // Use mapping schema with inlined_as_dict keyed by 'key'
    let schema = from_yaml(Path::new(&data_path("mapping_schema.yaml"))).unwrap();
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).unwrap();
    let conv = converter_from_schema(&schema);
    let bag = sv
        .get_class(&Identifier::new("Bag"), &conv)
        .unwrap()
        .expect("class not found");

    let src = linkml_runtime::load_json_file(
        Path::new(&data_path("mapping_data.json")),
        &sv,
        &bag,
        &conv,
    )
    .unwrap();

    // Rename mapping key 'alpha' to 'alpha2'
    let mut tgt_json = src.to_json();
    if let JsonValue::Object(ref mut root) = tgt_json {
        if let Some(JsonValue::Object(things)) = root.get_mut("things") {
            if let Some(alpha) = things.remove("alpha") {
                things.insert("alpha2".to_string(), alpha);
            }
        }
    }
    let tgt = load_json_str(&serde_json::to_string(&tgt_json).unwrap(), &sv, &bag, &conv).unwrap();

    let deltas = diff(&src, &tgt, false);
    // Expect one delete and one add at mapping keys; no inner key-slot deltas
    assert!(deltas
        .iter()
        .any(|d| d.path == vec!["things".to_string(), "alpha".to_string()] && d.new.is_none()));
    assert!(deltas
        .iter()
        .any(|d| d.path == vec!["things".to_string(), "alpha2".to_string()] && d.old.is_none()));
    assert!(!deltas
        .iter()
        .any(|d| d.path == vec!["things".to_string(), "alpha".to_string(), "key".to_string()]));

    let (patched, _trace) = patch(&src, &deltas, &sv).unwrap();
    assert_eq!(patched.to_json(), tgt.to_json());
}
