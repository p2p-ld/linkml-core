use curies::Converter;
use linkml_schemaview::identifier::Identifier;
use linkml_schemaview::schemaview::{ClassView, SchemaView, SlotContainerMode, SlotView};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub mod diff;
#[cfg(feature = "python")]
pub mod python;
pub mod turtle;
pub use diff::{diff, patch, Delta};
#[cfg(feature = "python")]
pub use python::*;
#[derive(Debug)]
pub struct LinkMLError(pub String);

impl std::fmt::Display for LinkMLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for LinkMLError {}

pub type LResult<T> = std::result::Result<T, LinkMLError>;

fn path_to_string(path: &[String]) -> String {
    if path.is_empty() {
        "<root>".to_string()
    } else {
        path.join(".")
    }
}

pub enum LinkMLValue<'a> {
    Scalar {
        value: JsonValue,
        slot: SlotView<'a>,
        class: Option<ClassView<'a>>,
        sv: &'a SchemaView,
    },
    List {
        values: Vec<LinkMLValue<'a>>,
        slot: SlotView<'a>,
        class: Option<ClassView<'a>>,
        sv: &'a SchemaView,
    },
    Map {
        values: HashMap<String, LinkMLValue<'a>>,
        class: ClassView<'a>,
        sv: &'a SchemaView,
    },
}

impl<'a> LinkMLValue<'a> {
    fn find_scalar_slot_for_inlined_map(
        class: &ClassView<'a>,
        key_slot_name: &str,
    ) -> Option<SlotView<'a>> {
        for s in class.slots() {
            if s.name == key_slot_name {
                continue;
            }
            let def = s.definition();
            if def.multivalued.unwrap_or(false) {
                continue;
            }
            if !s.is_range_scalar() {
                continue;
            }
            return Some(s.clone());
        }
        None
    }
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
        if let Some(ts) = base
            .slots()
            .iter()
            .find(|s| s.definition().designates_type.unwrap_or(false))
        {
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
            if let Ok(tmp) = LinkMLValue::from_json_internal(
                JsonValue::Object(map.clone()),
                Some((*c).clone()),
                None,
                sv,
                conv,
                false,
                false,
                Vec::new(),
            ) {
                if validate(&tmp).is_ok() {
                    return (*c).clone();
                }
            }
        }

        base.clone()
    }

    fn parse_list_slot(
        value: JsonValue,
        class: Option<ClassView<'a>>,
        sl: &SlotView<'a>,
        sv: &'a SchemaView,
        conv: &Converter,
        inside_list: bool,
        path: Vec<String>,
    ) -> LResult<Self> {
        match (inside_list, value) {
            (false, JsonValue::Array(arr)) => {
                let mut values = Vec::new();
                let class_range = sl.get_range_class();
                for (i, v) in arr.into_iter().enumerate() {
                    let mut p = path.clone();
                    p.push(format!("{}[{}]", sl.name, i));
                    values.push(Self::from_json_internal(
                        v,
                        class_range.clone(),
                        Some(sl.clone()),
                        sv,
                        conv,
                        true,
                        true,
                        p,
                    )?);
                }
                Ok(LinkMLValue::List {
                    values,
                    slot: sl.clone(),
                    class: class.clone(),
                    sv,
                })
            }
            (false, other) => Err(LinkMLError(format!(
                "expected list for slot `{}`, found {:?} at {}",
                sl.name,
                other,
                path_to_string(&path)
            ))),
            (true, other) => Ok(LinkMLValue::Scalar {
                value: other,
                slot: sl.clone(),
                class: class.clone(),
                sv,
            }),
        }
    }

    fn parse_mapping_slot(
        value: JsonValue,
        class: Option<ClassView<'a>>,
        sl: &SlotView<'a>,
        sv: &'a SchemaView,
        conv: &Converter,
        path: Vec<String>,
    ) -> LResult<Self> {
        match value {
            JsonValue::Object(map) => {
                let range_cv = sl
                    .definition()
                    .range
                    .as_ref()
                    .and_then(|r| sv.get_class(&Identifier::new(r), conv).ok().flatten())
                    .ok_or_else(|| {
                        LinkMLError(format!(
                            "mapping slot must have class range at {}",
                            path_to_string(&path)
                        ))
                    })?;
                let key_slot_name = range_cv
                    .key_or_identifier_slot()
                    .ok_or_else(|| {
                        LinkMLError(format!(
                            "mapping slot must have key or identifier at {}",
                            path_to_string(&path)
                        ))
                    })?
                    .name
                    .clone();
                let key_slot = sv
                    .get_slot(&Identifier::new(&key_slot_name), conv)
                    .ok()
                    .flatten()
                    .ok_or_else(|| LinkMLError(format!(
                        "key slot not found at {}",
                        path_to_string(&path)
                    )))?
                    .clone();
                let mut values = Vec::new();
                for (k, v) in map.into_iter() {
                    let mut m = match v {
                        JsonValue::Object(m) => m,
                        other => {
                            let chosen = sv
                                .get_class(&Identifier::new(range_cv.name()), conv)
                                .ok()
                                .flatten()
                                .unwrap_or_else(|| range_cv.clone());
                            let scalar_slot = Self::find_scalar_slot_for_inlined_map(
                                &chosen,
                                &key_slot.name,
                            )
                            .ok_or_else(|| {
                                LinkMLError(format!(
                                    "no scalar slot available for inlined mapping at {}",
                                    path_to_string(&path)
                                ))
                            })?;
                            let mut tmp = serde_json::Map::new();
                            tmp.insert(scalar_slot.name.clone(), other);
                            tmp
                        }
                    };
                    m.insert(key_slot.name.clone(), JsonValue::String(k.clone()));
                    let chosen = sv
                        .get_class(&Identifier::new(range_cv.name()), conv)
                        .ok()
                        .flatten()
                        .unwrap_or_else(|| range_cv.clone());
                    let mut child_values = HashMap::new();
                    for (ck, cv) in m.into_iter() {
                        let slot_tmp = chosen.slots().iter().find(|s| s.name == ck).cloned();
                        let mut p = path.clone();
                        p.push(format!("{}:{}", k, ck));
                        child_values.insert(
                            ck,
                            Self::from_json_internal(cv, None, slot_tmp, sv, conv, true, false, p)?,
                        );
                    }
                    values.push(LinkMLValue::Map {
                        values: child_values,
                        class: chosen.clone(),
                        sv,
                    });
                }
                Ok(LinkMLValue::List {
                    values,
                    slot: sl.clone(),
                    class: class.clone(),
                    sv,
                })
            }
            other => Err(LinkMLError(format!(
                "expected mapping for slot `{}`, found {:?} at {}",
                sl.name,
                other,
                path_to_string(&path)
            ))),
        }
    }

    fn parse_array_value(
        arr: Vec<JsonValue>,
        class: Option<ClassView<'a>>,
        slot: Option<SlotView<'a>>,
        sv: &'a SchemaView,
        conv: &Converter,
        path: Vec<String>,
    ) -> LResult<Self> {
        let sl = slot.ok_or_else(|| LinkMLError(format!(
            "list requires slot at {}",
            path_to_string(&path)
        )))?;
        let mut values = Vec::new();
        for (i, v) in arr.into_iter().enumerate() {
            let mut p = path.clone();
            p.push(format!("[{}]", i));
            values.push(Self::from_json_internal(v, None, None, sv, conv, true, false, p)?);
        }
        Ok(LinkMLValue::List {
            values,
            slot: sl,
            class: class.clone(),
            sv,
        })
    }

    fn parse_object_value(
        map: serde_json::Map<String, JsonValue>,
        class: Option<ClassView<'a>>,
        slot: Option<SlotView<'a>>,
        sv: &'a SchemaView,
        conv: &Converter,
        polymorphic: bool,
        path: Vec<String>,
    ) -> LResult<Self> {
        if !polymorphic && slot.is_none() {
            if let Some(cls) = &class {
                let mut values = HashMap::new();
                for (k, v) in map.clone().into_iter() {
                    let slot_ref = cls.slots().iter().find(|s| s.name == k).cloned();
                    let mut p = path.clone();
                    p.push(k.clone());
                    values.insert(
                        k,
                        Self::from_json_internal(v, None, slot_ref, sv, conv, true, false, p)?,
                    );
                }
                return Ok(LinkMLValue::Map {
                    values,
                    class: cls.clone(),
                    sv,
                });
            }
        }

        let base_class: Option<ClassView<'a>> = match class {
            Some(c) => Some(c.clone()),
            None => slot.as_ref().and_then(|s| {
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
            let mut p = path.clone();
            p.push(k.clone());
            values.insert(
                k,
                Self::from_json_internal(v, None, slot_tmp, sv, conv, true, false, p)?,
            );
        }
        Ok(LinkMLValue::Map {
            values,
            class: chosen.ok_or_else(|| LinkMLError("class not determined".to_string()))?,
            sv,
        })
    }

    fn parse_scalar_value(
        value: JsonValue,
        class: Option<ClassView<'a>>,
        slot: Option<SlotView<'a>>,
        sv: &'a SchemaView,
    ) -> LResult<Self> {
        let sl = slot.ok_or_else(|| LinkMLError(format!(
            "scalar requires slot {}",
            class.clone().map(|c| c.name().to_owned()).unwrap_or("null".to_owned())
        )))?;
        Ok(LinkMLValue::Scalar {
            value,
            slot: sl,
            class: class.clone(),
            sv,
        })
    }
    fn from_json_internal(
        value: JsonValue,
        class: Option<ClassView<'a>>,
        slot: Option<SlotView<'a>>,
        sv: &'a SchemaView,
        conv: &Converter,
        polymorphic: bool,
        inside_list: bool,
        path: Vec<String>,
    ) -> LResult<Self> {
        if let Some(ref sl) = slot {
            let container_mode = sl.determine_slot_container_mode();
            match container_mode {
                SlotContainerMode::List => {
                    return Self::parse_list_slot(value, class.clone(), sl, sv, conv, inside_list, path);
                }
                SlotContainerMode::Mapping => {
                    return Self::parse_mapping_slot(value, class.clone(), sl, sv, conv, path);
                }
                SlotContainerMode::SingleValue => {}
            }
        }

        match value {
            JsonValue::Array(arr) => Self::parse_array_value(arr, class, slot, sv, conv, path),
            JsonValue::Object(map) => {
                Self::parse_object_value(map, class, slot, sv, conv, polymorphic, path)
            }
            other => Self::parse_scalar_value(other, class, slot, sv),
        }
    }

    fn from_json(
        value: JsonValue,
        class: Option<ClassView<'a>>,
        slot: Option<SlotView<'a>>,
        sv: &'a SchemaView,
        conv: &Converter,
        polymorphic: bool,
        inside_list: bool,
    ) -> LResult<Self> {
        Self::from_json_internal(
            value,
            class,
            slot,
            sv,
            conv,
            polymorphic,
            inside_list,
            Vec::new(),
        )
    }
}

pub fn load_yaml_file<'a>(
    path: &Path,
    sv: &'a SchemaView,
    class: Option<&'a ClassView<'a>>,
    conv: &Converter,
) -> std::result::Result<LinkMLValue<'a>, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    load_yaml_str(&text, sv, class, conv)
}

pub fn load_yaml_str<'a>(
    data: &str,
    sv: &'a SchemaView,
    class: Option<&'a ClassView<'a>>,
    conv: &Converter,
) -> std::result::Result<LinkMLValue<'a>, Box<dyn std::error::Error>> {
    let value: serde_yaml::Value = serde_yaml::from_str(data)?;
    let json = serde_json::to_value(value)?;
    LinkMLValue::from_json(json, class.cloned(), None, sv, conv, true, false)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

pub fn load_json_file<'a>(
    path: &Path,
    sv: &'a SchemaView,
    class: Option<&'a ClassView<'a>>,
    conv: &Converter,
) -> std::result::Result<LinkMLValue<'a>, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    load_json_str(&text, sv, class, conv)
}

pub fn load_json_str<'a>(
    data: &str,
    sv: &'a SchemaView,
    class: Option<&'a ClassView<'a>>,
    conv: &Converter,
) -> std::result::Result<LinkMLValue<'a>, Box<dyn std::error::Error>> {
    let value: JsonValue = serde_json::from_str(data)?;
    LinkMLValue::from_json(value, class.cloned(), None, sv, conv, true, false)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

fn validate_inner<'a>(value: &LinkMLValue<'a>) -> std::result::Result<(), String> {
    match value {
        LinkMLValue::Scalar { .. } => Ok(()),
        LinkMLValue::List { values, .. } => {
            for v in values {
                validate_inner(v)?;
            }
            Ok(())
        }
        LinkMLValue::Map { values, class, .. } => {
            for (k, v) in values {
                if class.slots().iter().all(|s| s.name != *k) {
                    return Err(format!("unknown slot `{}` for class `{}`", k, class.name()));
                }
                validate_inner(v)?;
            }
            Ok(())
        }
    }
}

pub fn validate<'a>(value: &LinkMLValue<'a>) -> std::result::Result<(), String> {
    validate_inner(value)
}

fn validate_collect<'a>(value: &LinkMLValue<'a>, errors: &mut Vec<String>) {
    match value {
        LinkMLValue::Scalar { .. } => {}
        LinkMLValue::List { values, .. } => {
            for v in values {
                validate_collect(v, errors);
            }
        }
        LinkMLValue::Map { values, class, .. } => {
            for (k, v) in values {
                if class.slots().iter().all(|s| s.name != *k) {
                    errors.push(format!("unknown slot `{}` for class `{}`", k, class.name()));
                }
                validate_collect(v, errors);
            }
        }
    }
}

pub fn validate_errors<'a>(value: &LinkMLValue<'a>) -> Vec<String> {
    let mut errs = Vec::new();
    validate_collect(value, &mut errs);
    errs
}
