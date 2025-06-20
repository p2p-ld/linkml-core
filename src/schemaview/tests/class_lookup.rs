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
fn lookup_classes() {
    let person_schema = from_yaml(Path::new(&data_path("person.yaml"))).unwrap();
    let container_schema = from_yaml(Path::new(&data_path("container.yaml"))).unwrap();

    let mut sv = SchemaView::new();
    sv.add_schema(container_schema.clone()).unwrap();
    sv.add_schema(person_schema.clone()).unwrap();

    let conv = converter_from_schemas([&person_schema, &container_schema]);
    assert!(conv.expand("personinfo:Person").is_ok());

    // Container should be found by name in primary schema
    let c = sv.get_class(&Identifier::new("Container"), &conv).unwrap();
    assert!(c.is_some());

    // Person only accessible by curie or uri
    let p1 = sv
        .get_class(&Identifier::new("personinfo:Person"), &conv)
        .unwrap();
    assert!(p1.is_some());
    let p2 = sv.get_class(&Identifier::new("alt:Person"), &conv).unwrap();
    assert!(p2.is_some());
    let p_linkml = sv.get_class(&Identifier::new("linkml:Person"), &conv).unwrap();
    assert!(p_linkml.is_some());
    let p3 = sv
        .get_class(
            &Identifier::new("https://w3id.org/linkml/examples/personinfo/Person"),
            &conv,
        )
        .unwrap();
    assert!(p3.is_some());
}
