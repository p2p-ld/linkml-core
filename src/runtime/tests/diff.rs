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

    let deltas = diff(&src, &tgt, false);
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
fn diff_null_and_missing_semantics() {
    use linkml_runtime::LinkMLValue;
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

    // X -> null => update to null
    if let LinkMLValue::Object { values, .. } = src.clone() {
        let mut tgt_values = values.clone();
        let age_slot = class
            .slots()
            .iter()
            .find(|s| s.name == "age")
            .expect("age slot");
        tgt_values.insert(
            "age".to_string(),
            LinkMLValue::Null {
                slot: age_slot.clone(),
                class: Some(class.clone()),
                sv: sv.clone(),
            },
        );
        let deltas = diff(
            &LinkMLValue::Object {
                values: values.clone(),
                class: class.clone(),
                sv: sv.clone(),
            },
            &LinkMLValue::Object {
                values: tgt_values.clone(),
                class: class.clone(),
                sv: sv.clone(),
            },
            false,
        );
        assert!(deltas
            .iter()
            .any(|d| d.path == vec!["age".to_string()] && d.new == Some(serde_json::Value::Null)));
    }

    // null -> X => update from null
    if let LinkMLValue::Object { values, .. } = src.clone() {
        let mut src_values = values.clone();
        let age_slot = class
            .slots()
            .iter()
            .find(|s| s.name == "age")
            .expect("age slot");
        src_values.insert(
            "age".to_string(),
            LinkMLValue::Null {
                slot: age_slot.clone(),
                class: Some(class.clone()),
                sv: sv.clone(),
            },
        );
        let deltas = diff(
            &LinkMLValue::Object {
                values: src_values.clone(),
                class: class.clone(),
                sv: sv.clone(),
            },
            &LinkMLValue::Object {
                values: values.clone(),
                class: class.clone(),
                sv: sv.clone(),
            },
            false,
        );
        assert!(deltas.iter().any(|d| d.path == vec!["age".to_string()]
            && d.old == Some(serde_json::Value::Null)
            && d.new.is_some()));
    }

    // missing -> X => add
    if let LinkMLValue::Object { values, .. } = src.clone() {
        let mut src_values = values.clone();
        src_values.remove("age");
        let deltas = diff(
            &LinkMLValue::Object {
                values: src_values.clone(),
                class: class.clone(),
                sv: sv.clone(),
            },
            &LinkMLValue::Object {
                values: values.clone(),
                class: class.clone(),
                sv: sv.clone(),
            },
            false,
        );
        assert!(deltas
            .iter()
            .any(|d| d.path == vec!["age".to_string()] && d.old.is_none() && d.new.is_some()));
    }

    // X -> missing: ignored by default; produce update-to-null when treat_missing_as_null=true
    if let LinkMLValue::Object { values, .. } = src.clone() {
        let mut tgt_values = values.clone();
        tgt_values.remove("age");
        let deltas = diff(
            &LinkMLValue::Object {
                values: values.clone(),
                class: class.clone(),
                sv: sv.clone(),
            },
            &LinkMLValue::Object {
                values: tgt_values.clone(),
                class: class.clone(),
                sv: sv.clone(),
            },
            false,
        );
        assert!(deltas.iter().all(|d| d.path != vec!["age".to_string()]));
        let deltas2 = diff(
            &LinkMLValue::Object {
                values: values.clone(),
                class: class.clone(),
                sv: sv.clone(),
            },
            &LinkMLValue::Object {
                values: tgt_values.clone(),
                class: class.clone(),
                sv: sv.clone(),
            },
            true,
        );
        assert!(deltas2
            .iter()
            .any(|d| d.path == vec!["age".to_string()] && d.new == Some(serde_json::Value::Null)))
    }
}

#[test]
fn diff_add_remove_in_collections() {
    use linkml_runtime::diff::DeltaOp;
    // Use personinfo Container with multivalued 'objects' (list) and
    // inlined-as-dict 'has_familial_relationships' on Person.
    let schema = from_yaml(Path::new(&info_path("personinfo.yaml"))).unwrap();
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).unwrap();
    let conv = converter_from_schema(&schema);
    let container = sv
        .get_class(&Identifier::new("Container"), &conv)
        .unwrap()
        .expect("class not found");
    let base = load_yaml_file(
        Path::new(&info_path("example_personinfo_data.yaml")),
        &sv,
        &container,
        &conv,
    )
    .unwrap();

    // Prepare JSON for easier mutation
    let mut base_json = base.to_json();
    // Add an object to 'objects' list
    if let serde_json::Value::Object(ref mut root) = base_json {
        if let Some(serde_json::Value::Array(ref mut arr)) = root.get_mut("objects") {
            let new_obj = serde_json::json!({
                "id": "P:999",
                "name": "Added Person",
                "objecttype": "https://w3id.org/linkml/examples/personinfo/Person"
            });
            arr.push(new_obj);
        }
    }
    let base_plus_one = linkml_runtime::load_json_str(
        &serde_json::to_string(&base_json).unwrap(),
        &sv,
        &container,
        &conv,
    )
    .unwrap();
    let deltas = diff(&base, &base_plus_one, false);
    assert!(deltas
        .iter()
        .any(|d| d.op == DeltaOp::Add && d.path.first().map(|s| s.as_str()) == Some("objects")));

    // Remove last element from 'objects'
    let mut minus_json = base_plus_one.to_json();
    if let serde_json::Value::Object(ref mut root) = minus_json {
        if let Some(serde_json::Value::Array(ref mut arr)) = root.get_mut("objects") {
            arr.pop();
        }
    }
    let base_minus_one = linkml_runtime::load_json_str(
        &serde_json::to_string(&minus_json).unwrap(),
        &sv,
        &container,
        &conv,
    )
    .unwrap();
    let deltas = diff(&base_plus_one, &base_minus_one, false);
    assert!(deltas
        .iter()
        .any(|d| d.op == DeltaOp::Remove && d.path.first().map(|s| s.as_str()) == Some("objects")));

    // Mapping add/remove: add a new familial relationship key on the first Person
    let mut map_add = base.to_json();
    if let serde_json::Value::Object(ref mut root) = map_add {
        if let Some(serde_json::Value::Array(ref mut arr)) = root.get_mut("objects") {
            if let Some(serde_json::Value::Object(ref mut first)) = arr
                .iter_mut()
                .find(|v| v.get("has_familial_relationships").is_some())
            {
                let rels = first
                    .entry("has_familial_relationships")
                    .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()));
                if let serde_json::Value::Object(ref mut map) = rels {
                    map.insert(
                        "test_sibling".to_string(),
                        serde_json::json!({
                            "related_to": "P:001",
                            "type": "SIBLING_OF"
                        }),
                    );
                }
            }
        }
    }
    let v_add = linkml_runtime::load_json_str(
        &serde_json::to_string(&map_add).unwrap(),
        &sv,
        &container,
        &conv,
    )
    .unwrap();
    let deltas = diff(&base, &v_add, false);
    assert!(deltas.iter().any(
        |d| d.op == DeltaOp::Add && d.path.contains(&"has_familial_relationships".to_string())
    ));

    // Remove that key again
    let mut map_remove = v_add.to_json();
    if let serde_json::Value::Object(ref mut root) = map_remove {
        if let Some(serde_json::Value::Array(ref mut arr)) = root.get_mut("objects") {
            if let Some(serde_json::Value::Object(ref mut first)) = arr
                .iter_mut()
                .find(|v| v.get("has_familial_relationships").is_some())
            {
                if let Some(serde_json::Value::Object(ref mut map)) =
                    first.get_mut("has_familial_relationships")
                {
                    map.remove("test_sibling");
                }
            }
        }
    }
    let v_remove = linkml_runtime::load_json_str(
        &serde_json::to_string(&map_remove).unwrap(),
        &sv,
        &container,
        &conv,
    )
    .unwrap();
    let deltas = diff(&v_add, &v_remove, false);
    assert!(deltas
        .iter()
        .any(|d| d.op == DeltaOp::Remove
            && d.path.contains(&"has_familial_relationships".to_string())));
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
