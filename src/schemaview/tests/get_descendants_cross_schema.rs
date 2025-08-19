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
fn cross_schema_descendants() {
    let geo_schema = from_yaml(Path::new(&data_path("geo.yaml"))).unwrap();
    let era_schema = from_yaml(Path::new(&data_path("era.yaml"))).unwrap();
    let mut sv = SchemaView::new();
    sv.add_schema(geo_schema.clone()).unwrap();
    sv.add_schema(era_schema.clone()).unwrap();
    let conv = converter_from_schemas([&geo_schema, &era_schema]);
    let base_class = sv.get_class(&Identifier::new("geo:Geometry"), &conv).unwrap().unwrap();
    let descendants = base_class.get_descendants(false, false).unwrap();
    assert!(descendants.iter().any(|cv| cv.get_uri(&conv, false, false).unwrap().to_string() == "era:Geometry"));
}

