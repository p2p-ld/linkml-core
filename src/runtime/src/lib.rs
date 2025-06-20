use schemaview::schemaview::{ClassView, SchemaView, SlotView};
use schemaview::identifier::Identifier;
use serde_json::Value as JsonValue;
use curies::Converter;
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
    fn select_class(
        map: &serde_json::Map<String, JsonValue>,
        base: &'a ClassView<'a>,
        sv: &'a SchemaView,
        conv: &Converter,
    ) -> &'a ClassView<'a> {
        let base_box = Box::leak(Box::new(base.clone()));
        let mut cands: Vec<&'a ClassView<'a>> = vec![base_box];
        if let Ok(desc) = base_box.get_descendants(conv, true) {
            for d in desc.into_iter() {
                cands.push(Box::leak(Box::new(d)));
            }
        }

        let mut preferred: Option<&ClassView<'a>> = None;
        if let Some(ts) = base_box
            .slots()
            .iter()
            .find(|s| s.definitions.iter().rev().any(|d| d.designates_type.unwrap_or(false)))
        {
            if let Some(JsonValue::String(tv)) = map.get(&ts.name) {
                if let Some(def) = ts.definitions.iter().rev().find(|d| d.designates_type.unwrap_or(false)) {
                    for c in &cands {
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
            return p;
        }

        for c in &cands {
            let tmp = LinkMLValue::from_json(JsonValue::Object(map.clone()), Some(*c), None, sv, conv, false);
            if validate(&tmp).is_ok() {
                return *c;
            }
        }

        base_box
    }
    fn from_json(
        value: JsonValue,
        class: Option<&'a ClassView<'a>>,
        slot: Option<&'a SlotView<'a>>,
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
                            let slot_ref = cls.slots().iter().find(|s| s.name == k);
                            values.insert(k, LinkMLValue::from_json(v, None, slot_ref, sv, conv, true));
                        }
                        return LinkMLValue::Map { values, class: Some(cls), sv };
                    }
                }

                // determine base class
                let base_class_ref: Option<&'a ClassView<'a>> = match class {
                    Some(c) => Some(Box::leak(Box::new(c.clone())) as &'a ClassView<'a>),
                    None => slot.and_then(|s| {
                        s.definitions
                            .iter()
                            .rev()
                            .find_map(|d| d.range.as_deref())
                            .and_then(|r| sv.get_class(&Identifier::new(r), conv).ok().flatten())
                            .map(|cls| Box::leak(Box::new(cls)) as &'a ClassView<'a>)
                    }),
                };

                let chosen_ref: Option<&'a ClassView<'a>> =
                    base_class_ref.map(|b| Self::select_class(&map, b, sv, conv));

                let mut values = HashMap::new();
                for (k, v) in map.into_iter() {
                    let slot_ref = chosen_ref
                        .and_then(|cv| cv.slots().iter().find(|s| s.name == k));
                    values.insert(k, LinkMLValue::from_json(v, None, slot_ref, sv, conv, true));
                }
                LinkMLValue::Map { values, class: chosen_ref, sv }
            }
            other => LinkMLValue::Scalar { value: other, slot, sv },
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
    let value: serde_yaml::Value = serde_yaml::from_str(&text)?;
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
    let value: JsonValue = serde_json::from_str(&text)?;
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

