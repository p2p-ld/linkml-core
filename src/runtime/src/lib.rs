use curies::Converter;
use linkml_schemaview::identifier::Identifier;
use linkml_schemaview::schemaview::{ClassView, SchemaView, SlotView};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[cfg(feature = "python")]
pub mod python;
pub mod turtle;
#[cfg(feature = "python")]
pub use python::*;
pub enum LinkMLValue<'a> {
    Scalar {
        value: JsonValue,
        slot: Option<SlotView<'a>>,
        sv: &'a SchemaView,
    },
    List {
        values: Vec<LinkMLValue<'a>>,
        slot: Option<SlotView<'a>>,
        sv: &'a SchemaView,
    },
    Map {
        values: HashMap<String, LinkMLValue<'a>>,
        class: Option<ClassView<'a>>,
        sv: &'a SchemaView,
    },
}

impl<'a> LinkMLValue<'a> {
    fn select_class(
        map: &serde_json::Map<String, JsonValue>,
        base: &ClassView<'a>,
        sv: &'a SchemaView,
        conv: &Converter,
    ) -> ClassView<'a> {
        let descendants = match base.get_descendants(conv, true) {
            Ok(d) => d,
            Err(_) => Vec::new(),
        };
        let mut cand_refs: Vec<&ClassView<'a>> = vec![base];
        for d in &descendants {
            cand_refs.push(d);
        }

        let mut preferred: Option<&ClassView<'a>> = None;
        if let Some(ts) = base.slots().iter().find(|s| {
            s.definitions
                .iter()
                .rev()
                .any(|d| d.designates_type.unwrap_or(false))
        }) {
            if let Some(JsonValue::String(tv)) = map.get(&ts.name) {
                if let Some(def) = ts
                    .definitions
                    .iter()
                    .rev()
                    .find(|d| d.designates_type.unwrap_or(false))
                {
                    for c in &cand_refs {
                        if let Ok(vals) = c.get_accepted_type_designator_values(def, conv) {
                            if vals.iter().any(|v| v.to_string() == *tv) {
                                preferred = Some(*c);
                                break;
                            }
                        }
                    }
                }
            }
        }
        if let Some(p) = preferred {
            return p.clone();
        }

        for c in &cand_refs {
            let tmp = LinkMLValue::from_json(
                JsonValue::Object(map.clone()),
                Some(*c),
                None,
                sv,
                conv,
                false,
            );
            if validate(&tmp).is_ok() {
                return (*c).clone();
            }
        }

        base.clone()
    }
    fn from_json(
        value: JsonValue,
        class: Option<&'a ClassView<'a>>,
        slot: Option<SlotView<'a>>,
        sv: &'a SchemaView,
        conv: &Converter,
        polymorphic: bool,
    ) -> Self {
        match value {
            JsonValue::Array(arr) => {
                let values = arr
                    .into_iter()
                    .map(|v| LinkMLValue::from_json(v, None, None, sv, conv, true))
                    .collect();
                LinkMLValue::List { values, slot, sv }
            }
            JsonValue::Object(map) => {
                if !polymorphic && slot.is_none() {
                    if let Some(cls) = class {
                        let mut values = HashMap::new();
                        for (k, v) in map.into_iter() {
                            let slot_ref = cls.slots().iter().find(|s| s.name == k).cloned();
                            values.insert(
                                k,
                                LinkMLValue::from_json(v, None, slot_ref, sv, conv, true),
                            );
                        }
                        return LinkMLValue::Map {
                            values,
                            class: Some(cls.clone()),
                            sv,
                        };
                    }
                }

                // determine base class
                let base_class: Option<ClassView<'a>> = match class {
                    Some(c) => Some(c.clone()),
                    None => slot.and_then(|s| {
                        s.definitions
                            .iter()
                            .rev()
                            .find_map(|d| d.range.as_deref())
                            .and_then(|r| sv.get_class(&Identifier::new(r), conv).ok().flatten())
                    }),
                };

                let chosen: Option<ClassView<'a>> = base_class
                    .as_ref()
                    .map(|b| Self::select_class(&map, b, sv, conv));

                let mut values = HashMap::new();
                for (k, v) in map.into_iter() {
                    let slot_tmp: Option<SlotView<'a>> = chosen
                        .as_ref()
                        .and_then(|cv| cv.slots().iter().find(|s| s.name == k))
                        .cloned();
                    values.insert(k, LinkMLValue::from_json(v, None, slot_tmp, sv, conv, true));
                }
                LinkMLValue::Map {
                    values,
                    class: chosen,
                    sv,
                }
            }
            other => LinkMLValue::Scalar {
                value: other,
                slot,
                sv,
            },
        }
    }
}

pub fn load_yaml_file<'a>(
    path: &Path,
    sv: &'a SchemaView,
    class: Option<&'a ClassView<'a>>,
    conv: &Converter,
) -> Result<LinkMLValue<'a>, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    load_yaml_str(&text, sv, class, conv)
}

pub fn load_yaml_str<'a>(
    data: &str,
    sv: &'a SchemaView,
    class: Option<&'a ClassView<'a>>,
    conv: &Converter,
) -> Result<LinkMLValue<'a>, Box<dyn std::error::Error>> {
    let value: serde_yaml::Value = serde_yaml::from_str(data)?;
    let json = serde_json::to_value(value)?;
    Ok(LinkMLValue::from_json(json, class, None, sv, conv, true))
}

pub fn load_json_file<'a>(
    path: &Path,
    sv: &'a SchemaView,
    class: Option<&'a ClassView<'a>>,
    conv: &Converter,
) -> Result<LinkMLValue<'a>, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    load_json_str(&text, sv, class, conv)
}

pub fn load_json_str<'a>(
    data: &str,
    sv: &'a SchemaView,
    class: Option<&'a ClassView<'a>>,
    conv: &Converter,
) -> Result<LinkMLValue<'a>, Box<dyn std::error::Error>> {
    let value: JsonValue = serde_json::from_str(data)?;
    Ok(LinkMLValue::from_json(value, class, None, sv, conv, true))
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
            if let Some(cv) = class.as_ref() {
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
