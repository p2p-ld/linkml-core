use schemaview::identifier::{converter_from_schemas, Identifier};
use schemaview::io::from_yaml;
use schemaview::schemaview::{SchemaView, SlotContainerMode, SlotInlineMode};
use std::path::{Path, PathBuf};

fn data_path(name: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests");
    p.push("data");
    p.push(name);
    p
}

#[test]
fn test_slot_modes() {
    let schema = from_yaml(Path::new(&data_path("modes_schema.yaml"))).unwrap();
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).unwrap();
    let conv = converter_from_schemas([&schema]);

    let container = sv
        .get_class(&Identifier::new("Container"), &conv)
        .unwrap()
        .unwrap();
    let persons_slot = container
        .slots()
        .iter()
        .find(|s| s.name == "persons")
        .unwrap();
    assert_eq!(
        persons_slot.determine_slot_container_mode(&sv, &conv),
        SlotContainerMode::List
    );
    assert_eq!(
        persons_slot.determine_slot_inline_mode(&sv, &conv),
        SlotInlineMode::Inline
    );

    let best_friend = container
        .slots()
        .iter()
        .find(|s| s.name == "best_friend")
        .unwrap();
    assert_eq!(
        best_friend.determine_slot_container_mode(&sv, &conv),
        SlotContainerMode::SingleValue
    );
    assert_eq!(
        best_friend.determine_slot_inline_mode(&sv, &conv),
        SlotInlineMode::Reference
    );

    let person = sv
        .get_class(&Identifier::new("Person"), &conv)
        .unwrap()
        .unwrap();
    let name_slot = person
        .slots()
        .iter()
        .find(|s| s.name == "name")
        .unwrap();
    assert_eq!(
        name_slot.determine_slot_container_mode(&sv, &conv),
        SlotContainerMode::SingleValue
    );
    assert_eq!(
        name_slot.determine_slot_inline_mode(&sv, &conv),
        SlotInlineMode::Primitive
    );

    let holder = sv
        .get_class(&Identifier::new("Holder"), &conv)
        .unwrap()
        .unwrap();
    let exts_slot = holder
        .slots()
        .iter()
        .find(|s| s.name == "exts")
        .unwrap();
    assert_eq!(
        exts_slot.determine_slot_container_mode(&sv, &conv),
        SlotContainerMode::Mapping
    );
    assert_eq!(
        exts_slot.determine_slot_inline_mode(&sv, &conv),
        SlotInlineMode::Inline
    );
}

