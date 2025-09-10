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
