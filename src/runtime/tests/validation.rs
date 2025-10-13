use linkml_runtime::{load_yaml_file, validate};
use linkml_schemaview::identifier::{converter_from_schema, Identifier};
use linkml_schemaview::io::from_yaml;
use linkml_schemaview::schemaview::SchemaView;
use std::path::{Path, PathBuf};

fn info_path(name: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests");
    p.push("data");
    p.push(name);
    p
}

#[test]
fn validate_personinfo_example1() {
    let schema = from_yaml(Path::new(&info_path("personinfo.yaml"))).unwrap();
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).unwrap();
    let conv = converter_from_schema(&schema);
    let container = sv
        .get_class(&Identifier::new("Container"), &conv)
        .unwrap()
        .expect("class not found");
    let v = load_yaml_file(
        Path::new(&info_path("example_personinfo_data.yaml")),
        &sv,
        &container,
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
    let container = sv
        .get_class(&Identifier::new("Container"), &conv)
        .unwrap()
        .expect("class not found");
    let v = load_yaml_file(
        Path::new(&info_path("example_personinfo_data_2.yaml")),
        &sv,
        &container,
        &conv,
    )
    .unwrap();
    assert!(validate(&v).is_ok());
}

#[test]
fn validate_personinfo_null_collections() {
    let schema = from_yaml(Path::new(&info_path("personinfo.yaml"))).unwrap();
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).unwrap();
    let conv = converter_from_schema(&schema);
    let container = sv
        .get_class(&Identifier::new("Container"), &conv)
        .unwrap()
        .expect("class not found");
    let v = load_yaml_file(
        Path::new(&info_path("example_personinfo_data_nulls.yaml")),
        &sv,
        &container,
        &conv,
    )
    .unwrap();
    assert!(validate(&v).is_ok());
    // Assert that nulls are preserved as LinkMLInstance::Null (not empty collections)
    if let linkml_runtime::LinkMLInstance::Object { values, .. } = &v {
        if let Some(linkml_runtime::LinkMLInstance::List { values: objs, .. }) =
            values.get("objects")
        {
            if let Some(linkml_runtime::LinkMLInstance::Object { values: person, .. }) =
                objs.first()
            {
                assert!(matches!(
                    person.get("aliases"),
                    Some(linkml_runtime::LinkMLInstance::Null { .. })
                ));
                assert!(matches!(
                    person.get("has_employment_history"),
                    Some(linkml_runtime::LinkMLInstance::Null { .. })
                ));
                assert!(matches!(
                    person.get("has_familial_relationships"),
                    Some(linkml_runtime::LinkMLInstance::Null { .. })
                ));
            } else {
                panic!("expected first object to be an Object");
            }
        } else {
            panic!("expected Container.objects to be a List");
        }
    } else {
        panic!("expected root to be an Object");
    }
}
