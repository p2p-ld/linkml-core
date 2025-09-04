use linkml_schemaview::identifier::{converter_from_schemas, Identifier};
use linkml_schemaview::io::from_yaml;
use linkml_schemaview::schemaview::SchemaView;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

fn data_path(name: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests");
    p.push("data");
    p.push(name);
    p
}

#[test]
fn slot_lookup_and_class_slots() {
    let units_schema = from_yaml(Path::new(&data_path("units.yaml"))).unwrap();
    let mappings_schema = from_yaml(Path::new(&data_path("mappings.yaml"))).unwrap();

    let mut sv = SchemaView::new();
    sv.add_schema(units_schema.clone()).unwrap();
    sv.add_schema(mappings_schema.clone()).unwrap();

    let conv = converter_from_schemas([&units_schema, &mappings_schema]);

    assert!(sv
        .get_slot(&Identifier::new("abbreviation"), &conv)
        .unwrap()
        .is_some());

    // class slots with slot_usage
    let class = sv
        .get_class(&Identifier::new("UnitOfMeasure"), &conv)
        .unwrap()
        .unwrap();
    let slots = class.slots();
    let mut map: HashMap<String, usize> = HashMap::new();
    for s in slots {
        map.insert(s.name.clone(), s.definitions().len());
    }
    assert_eq!(map.get("symbol"), Some(&1usize));
    assert_eq!(map.get("exact mappings"), Some(&2usize));
}
