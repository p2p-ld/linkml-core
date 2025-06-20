use schemaview::schemaview::{ClassView, SchemaView, SlotView};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub enum LinkMLValue<'a> {
    Scalar {
        value: JsonValue,
        slot: Option<&'a SlotView<'a>>,
        sv: &'a SchemaView,
    },
    List {
        values: Vec<LinkMLValue<'a>>,
        slot: Option<&'a SlotView<'a>>,
        sv: &'a SchemaView,
    },
    Map {
        values: HashMap<String, LinkMLValue<'a>>,
        class: Option<&'a ClassView<'a>>,
        sv: &'a SchemaView,
    },
}

impl<'a> LinkMLValue<'a> {
    fn from_json(
        value: JsonValue,
        class: Option<&'a ClassView<'a>>,
        slot: Option<&'a SlotView<'a>>,
        sv: &'a SchemaView,
    ) -> Self {
        match value {
            JsonValue::Array(arr) => {
                let values = arr
                    .into_iter()
                    .map(|v| LinkMLValue::from_json(v, None, None, sv))
                    .collect();
                LinkMLValue::List { values, slot, sv }
            }
            JsonValue::Object(map) => {
                let mut values = HashMap::new();
                for (k, v) in map.into_iter() {
                    let slot_ref = class
                        .and_then(|cv| cv.slots().iter().find(|s| s.name == k));
                    values.insert(k, LinkMLValue::from_json(v, None, slot_ref, sv));
                }
                LinkMLValue::Map { values, class, sv }
            }
            other => LinkMLValue::Scalar { value: other, slot, sv },
        }
    }
}

pub fn load_yaml_file<'a>(
    path: &Path,
    sv: &'a SchemaView,
    class: Option<&'a ClassView<'a>>,
) -> Result<LinkMLValue<'a>, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    let value: serde_yaml::Value = serde_yaml::from_str(&text)?;
    let json = serde_json::to_value(value)?;
    Ok(LinkMLValue::from_json(json, class, None, sv))
}

pub fn load_json_file<'a>(
    path: &Path,
    sv: &'a SchemaView,
    class: Option<&'a ClassView<'a>>,
) -> Result<LinkMLValue<'a>, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    let value: JsonValue = serde_json::from_str(&text)?;
    Ok(LinkMLValue::from_json(value, class, None, sv))
}

fn validate_inner<'a>(value: &LinkMLValue<'a>) -> Result<(), String> {
    match value {
        LinkMLValue::Scalar { .. } => Ok(()),
        LinkMLValue::List { values, .. } => {
            for v in values {
                validate_inner(v)?;
            }
            Ok(())
        }
        LinkMLValue::Map { values, class, .. } => {
            if let Some(cv) = class {
                for (k, v) in values {
                    if cv.slots().iter().all(|s| s.name != *k) {
                        return Err(format!("unknown slot `{}`", k));
                    }
                    validate_inner(v)?;
                }
            } else {
                for v in values.values() {
                    validate_inner(v)?;
                }
            }
            Ok(())
        }
    }
}

pub fn validate<'a>(value: &LinkMLValue<'a>) -> Result<(), String> {
    validate_inner(value)
}

