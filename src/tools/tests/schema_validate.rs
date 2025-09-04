use assert_cmd::Command;
use predicates::prelude::*;
use std::path::PathBuf;

fn data_path(name: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("../runtime/tests/data");
    p.push(name);
    p
}

#[test]
fn detect_invalid_schema() {
    let schema = data_path("invalid_schema.yaml");
    let mut cmd = Command::cargo_bin("linkml-schema-validate").unwrap();
    cmd.arg(&schema);
    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("Unknown parent class"))
        .stdout(predicate::str::contains("Unknown slot"));
}

#[test]
fn person_schema_missing_slot() {
    let schema = data_path("personinfo.yaml");
    let mut cmd = Command::cargo_bin("linkml-schema-validate").unwrap();
    cmd.arg(&schema);
    cmd.assert().success();
}
