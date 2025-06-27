use linkml_schemaview::identifier::{converter_from_schemas, Identifier};
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
fn lookup_prefixed_names() {
    let schema = from_yaml(Path::new(&data_path("prefixed_names.yaml"))).unwrap();
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).unwrap();
    let conv = converter_from_schemas([&schema]);

    let class = sv
        .get_class(&Identifier::new("geo:Geometry"), &conv)
        .unwrap();
    assert!(class.is_some());

    let slot = sv.get_slot(&Identifier::new("geo:point"), &conv).unwrap();
    assert!(slot.is_some());
}
