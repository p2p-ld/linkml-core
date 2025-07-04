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

fn alt_names(name: &str) -> Vec<String> {
    let mut v = Vec::new();
    if name.contains('_') {
        v.push(name.replace('_', " "));
    }
    if name.contains(' ') {
        v.push(name.replace(' ', "_"));
    }
    v
}

fn slot_matches_key(slot: &SlotView, key: &str) -> bool {
    if slot.name == key || alt_names(key).iter().any(|a| slot.name == *a) {
        return true;
    }
    if let Some(alias) = &slot.definition().alias {
        if alias == key || alt_names(key).iter().any(|a| alias == a) {
            return true;
        }
    }
    false
}

pub enum LinkMLValue {
    Scalar {
        value: JsonValue,
        slot: SlotView,
        class: Option<ClassView>,
        sv: SchemaView,
    },
    List {
        values: Vec<LinkMLValue>,
        slot: SlotView,
        class: Option<ClassView>,
        sv: SchemaView,
    },
    Map {
        values: HashMap<String, LinkMLValue>,
        class: ClassView,
        sv: SchemaView,
    },
}

impl LinkMLValue {
    fn find_scalar_slot_for_inlined_map(
        class: &ClassView,
        key_slot_name: &str,
    ) -> Option<SlotView> {
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
        base: &ClassView,
        sv: &SchemaView,
        conv: &Converter,
    ) -> ClassView {
        let descendants = match base.get_descendants( true, false) {
            Ok(d) => d,
            Err(_) => Vec::new(),
        };
        let mut cand_refs: Vec<&ClassView> = vec![base];
        for d in &descendants {
            cand_refs.push(d);
        }

        let mut preferred: Option<&ClassView> = None;
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
            if let Ok(tmp) = Self::parse_object_fixed_class(map.clone(), *c, sv, conv, Vec::new()) {
                if validate(&tmp).is_ok() {
                    return (*c).clone();
                }
            }
        }

        base.clone()
    }

    fn parse_object_fixed_class(
        map: serde_json::Map<String, JsonValue>,
        class: &ClassView,
        sv: &SchemaView,
        conv: &Converter,
        path: Vec<String>,
    ) -> LResult<Self> {
        let mut values = HashMap::new();
        for (k, v) in map.into_iter() {
            let slot_ref = class
                .slots()
                .iter()
                .find(|s| slot_matches_key(s, &k))
                .cloned();
            let mut p = path.clone();
            p.push(k.clone());
            let key_name = slot_ref
                .as_ref()
                .map(|s| s.name.clone())
                .unwrap_or_else(|| k.clone());
            values.insert(
                key_name,
                Self::from_json_internal(v, class.clone(), slot_ref, sv, conv, false, p)?,
            );
        }
        Ok(LinkMLValue::Map {
            values,
            class: class.clone(),
            sv: sv.clone(),
        })
    }

    fn parse_list_slot(
        value: JsonValue,
        class: ClassView,
        sl: SlotView,
        sv: &SchemaView,
        conv: &Converter,
        inside_list: bool,
        path: Vec<String>,
    ) -> LResult<Self> {
        match (inside_list, value) {
            (false, JsonValue::Array(arr)) => {
                let mut values = Vec::new();
                let class_range: Option<ClassView> = sl.get_range_class();
                let slot_for_item = if class_range.is_some() {
                    None
                } else {
                    Some(sl.clone())
                };
                for (i, v) in arr.into_iter().enumerate() {
                    let mut p = path.clone();
                    p.push(format!("{}[{}]", sl.name, i));
                    let v_transformed = if let (Some(cr), JsonValue::String(s)) = (class_range.as_ref(), &v) {
                        if let Some(id_slot) = cr.identifier_slot() {
                            let mut m = serde_json::Map::new();
                            m.insert(id_slot.name.clone(), JsonValue::String(s.clone()));
                            JsonValue::Object(m)
                        } else {
                            v
                        }
                    } else {
                        v
                    };
                    values.push(Self::from_json_internal(
                        v_transformed,
                        class_range.as_ref().unwrap_or(&class).clone(),
                        slot_for_item.clone(),
                        sv,
                        conv,
                        true,
                        p,
                    )?);
                }
                Ok(LinkMLValue::List {
                    values,
                    slot: sl.clone(),
                    class: Some(class.clone()),
                    sv: sv.clone(),
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
                class: Some(class.clone()),
                sv: sv.clone(),
            }),
        }
    }

    fn parse_mapping_slot(
        value: JsonValue,
        class: Option<ClassView>,
        sl: &SlotView,
        sv: &SchemaView,
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
                    .ok_or_else(|| {
                        LinkMLError(format!("key slot not found at {}", path_to_string(&path)))
                    })?
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
                            let scalar_slot =
                                Self::find_scalar_slot_for_inlined_map(&chosen, &key_slot.name)
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
                        let slot_tmp = chosen
                            .slots()
                            .iter()
                            .find(|s| slot_matches_key(s, &ck))
                            .cloned();
                        let mut p = path.clone();
                        p.push(format!("{}:{}", k, ck));
                        let key_name = slot_tmp
                            .as_ref()
                            .map(|s| s.name.clone())
                            .unwrap_or_else(|| ck.clone());
                        child_values.insert(
                            key_name,
                            Self::from_json_internal(
                                cv,
                                chosen.clone(),
                                slot_tmp,
                                sv,
                                conv,
                                false,
                                p,
                            )?,
                        );
                    }
                    values.push(LinkMLValue::Map {
                        values: child_values,
                        class: chosen.clone(),
                        sv: sv.clone(),
                    });
                }
                Ok(LinkMLValue::List {
                    values,
                    slot: sl.clone(),
                    class: class.clone(),
                    sv: sv.clone(),
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
        class: ClassView,
        slot: Option<SlotView>,
        sv: &SchemaView,
        conv: &Converter,
        path: Vec<String>,
    ) -> LResult<Self> {
        let sl = slot.ok_or_else(|| {
            LinkMLError(format!("list requires slot at {}", path_to_string(&path)))
        })?;
        let mut values = Vec::new();
        for (i, v) in arr.into_iter().enumerate() {
            let mut p = path.clone();
            p.push(format!("[{}]", i));
            values.push(Self::from_json_internal(v, class.clone(), None, sv, conv, false, p)?);
        }
        Ok(LinkMLValue::List {
            values,
            slot: sl,
            class: Some(class),
            sv: sv.clone(),
        })
    }

    fn parse_object_value(
        map: serde_json::Map<String, JsonValue>,
        class: ClassView,
        slot: Option<SlotView>,
        sv: &SchemaView,
        conv: &Converter,
        path: Vec<String>,
    ) -> LResult<Self> {
        let base_class = match slot {
            Some(sl) => sl.get_range_class(),
            None => Some(class.clone()),
        }.ok_or_else(|| {
            LinkMLError(format!(
                "object requires class or slot at {}",
                path_to_string(&path)
            ))
        })?;
        let chosen = Self::select_class(&map, &base_class, sv, conv);

        let mut values = HashMap::new();
        for (k, v) in map.into_iter() {
            let slot_tmp: Option<SlotView> = chosen.slots().iter().find(|s| slot_matches_key(s, &k))
                .cloned();
            let mut p = path.clone();
            p.push(k.clone());
            let key_name = slot_tmp
                .as_ref()
                .map(|s| s.name.clone())
                .unwrap_or_else(|| k.clone());
            values.insert(
                key_name,
                Self::from_json_internal(v, chosen.clone(), slot_tmp, sv, conv, false, p)?,
            );
        }
        Ok(LinkMLValue::Map {
            values,
            class: chosen,
            sv: sv.clone(),
        })
    }

    fn parse_scalar_value(
        value: JsonValue,
        class: ClassView,
        slot: Option<SlotView>,
        sv: &SchemaView,
        path: Vec<String>,
    ) -> LResult<Self> {
        let sl = slot.ok_or_else(|| {
            let classview_name = class.name().to_string();
            LinkMLError(format!(
                "scalar requires slot for at {} {}",
                path_to_string(&path), classview_name
            ))
        })?;
        Ok(LinkMLValue::Scalar {
            value,
            slot: sl,
            class: Some(class.clone()),
            sv: sv.clone(),
        })
    }

    fn from_json_internal(
        value: JsonValue,
        classview: ClassView,
        slot: Option<SlotView>,
        sv: &SchemaView,
        conv: &Converter,
        inside_list: bool,
        path: Vec<String>,
    ) -> LResult<Self> {
        if let Some(ref sl) = slot {
            let container_mode = sl.determine_slot_container_mode();
            match container_mode {
                SlotContainerMode::List => {
                    return Self::parse_list_slot(
                        value,
                        classview,
                        sl.clone(),
                        sv,
                        conv,
                        inside_list,
                        path,
                    );
                }
                SlotContainerMode::Mapping => {
                    return Self::parse_mapping_slot(value, Some(classview), sl, sv, conv, path);
                }
                SlotContainerMode::SingleValue => {}
            }
        }

        match value {
            JsonValue::Array(arr) => Self::parse_array_value(arr, classview, slot, sv, conv, path),
            JsonValue::Object(map) => {
                Self::parse_object_value(map, classview, slot, sv, conv, path)
            }
            other => {
                Self::parse_scalar_value(other, classview, slot, sv, path)
            }
        }
    }

    fn from_json(
        value: JsonValue,
        class: ClassView,
        slot: Option<SlotView>,
        sv: &SchemaView,
        conv: &Converter,
        inside_list: bool,
    ) -> LResult<Self> {
        Self::from_json_internal(value, class, slot, sv, conv, inside_list, Vec::new())
    }
}

pub fn load_yaml_file<'a>(
    path: &Path,
    sv: &SchemaView,
    class: &ClassView,
    conv: &Converter,
) -> std::result::Result<LinkMLValue, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    load_yaml_str(&text, sv, class, conv)
}

pub fn load_yaml_str<'a>(
    data: &str,
    sv: &SchemaView,
    class: &ClassView,
    conv: &Converter,
) -> std::result::Result<LinkMLValue, Box<dyn std::error::Error>> {
    let value: serde_yaml::Value = serde_yaml::from_str(data)?;
    let json = serde_json::to_value(value)?;
    LinkMLValue::from_json(json, class.clone(), None, sv, conv, false)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

pub fn load_json_file<'a>(
    path: &Path,
    sv: &'a SchemaView,
    class: &ClassView,
    conv: &Converter,
) -> std::result::Result<LinkMLValue, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    load_json_str(&text, sv, class, conv)
}

pub fn load_json_str<'a>(
    data: &str,
    sv: &'a SchemaView,
    class: &ClassView,
    conv: &Converter,
) -> std::result::Result<LinkMLValue, Box<dyn std::error::Error>> {
    let value: JsonValue = serde_json::from_str(data)?;
    LinkMLValue::from_json(value, class.clone(), None, sv, conv, false)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

fn validate_inner(value: &LinkMLValue) -> std::result::Result<(), String> {
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

pub fn validate(value: &LinkMLValue) -> std::result::Result<(), String> {
    validate_inner(value)
}

fn validate_collect(value: &LinkMLValue, errors: &mut Vec<String>) {
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

pub fn validate_errors(value: &LinkMLValue) -> Vec<String> {
    let mut errs = Vec::new();
    validate_collect(value, &mut errs);
    errs
}
