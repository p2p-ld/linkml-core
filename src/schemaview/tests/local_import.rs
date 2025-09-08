#![cfg(feature = "resolve")]

use linkml_schemaview::io::from_yaml;
use linkml_schemaview::resolve::resolve_schemas;
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
fn resolve_local_import() {
    let schema = from_yaml(Path::new(&data_path("local_main.yaml"))).unwrap();
    let mut sv = SchemaView::new();
    sv.add_schema(schema).unwrap();
    let unresolved = sv
        .get_unresolved_schemas()
        .iter()
        .map(|x| x.1.clone())
        .collect::<Vec<_>>();
    assert!(unresolved.contains(&"tests/data/local_target.yaml".to_string()));
    resolve_schemas(&mut sv).unwrap();
    let unresolved = sv.get_unresolved_schemas();
    println!("Unresolved imports after resolution: {:?}", unresolved);
    println!("Resolved schemas: {:?}", sv._get_resolved_schema_imports());
    assert!(unresolved.is_empty());
    assert!(sv.get_schema("http://example.com/local_target").is_some());
}
