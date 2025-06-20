use schemaview::identifier::{converter_from_schemas, Identifier};
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
fn class_get_uri() {
    let person_schema = from_yaml(Path::new(&data_path("person.yaml"))).unwrap();
    let mut sv = SchemaView::new();
    sv.add_schema(person_schema.clone()).unwrap();
    let conv = converter_from_schemas([&person_schema]);
    let cv = sv
        .get_class(&Identifier::new("Person"), &conv)
        .unwrap()
        .unwrap();
    assert_eq!(
        cv.get_uri(&conv, false, false).unwrap().to_string(),
        "linkml:Person"
    );
    assert_eq!(
        cv.get_uri(&conv, true, false).unwrap().to_string(),
        "personinfo:Person"
    );
}
