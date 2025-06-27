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
fn cross_schema_is_a() {
    let geo_schema = from_yaml(Path::new(&data_path("geo.yaml"))).unwrap();
    let era_schema = from_yaml(Path::new(&data_path("era.yaml"))).unwrap();

    let mut sv = SchemaView::new();
    sv.add_schema(geo_schema.clone()).unwrap();
    sv.add_schema(era_schema.clone()).unwrap();

    let conv = converter_from_schemas([&geo_schema, &era_schema]);

    let era_geom_curie = sv
        .get_class(&Identifier::new("era:Geometry"), &conv)
        .unwrap()
        .unwrap();
    assert_eq!(
        era_geom_curie
            .get_uri(&conv, false, false)
            .unwrap()
            .to_string(),
        "era:Geometry"
    );

    let parent = era_geom_curie.parent_class().unwrap().unwrap();
    assert_eq!(
        parent.get_uri(&conv, false, false).unwrap().to_string(),
        "geo:Geometry"
    );

    let era_geom_uri = sv
        .get_class(&Identifier::new("http://example.com/era/Geometry"), &conv)
        .unwrap()
        .unwrap();
    let parent_uri = era_geom_uri.parent_class().unwrap().unwrap();
    assert_eq!(
        parent_uri.get_uri(&conv, false, false).unwrap().to_string(),
        "geo:Geometry"
    );
}
