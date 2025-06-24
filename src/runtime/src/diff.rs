use crate::{load_json_str, LinkMLValue};
use linkml_schemaview::schemaview::SchemaView;
use serde_json::{Value as JsonValue, Map};

#[derive(Clone, Debug, PartialEq)]
pub struct Delta {
    pub path: Vec<String>,
    pub old: Option<JsonValue>,
    pub new: Option<JsonValue>,
}

impl<'a> LinkMLValue<'a> {
    pub fn to_json(&self) -> JsonValue {
        match self {
            LinkMLValue::Scalar { value, .. } => value.clone(),
            LinkMLValue::List { values, .. } => {
                JsonValue::Array(values.iter().map(|v| v.to_json()).collect())
            }
            LinkMLValue::Map { values, .. } => JsonValue::Object(
                values
                    .iter()
                    .map(|(k, v)| (k.clone(), v.to_json()))
                    .collect(),
            ),
        }
    }
}

pub fn diff<'a>(source: &LinkMLValue<'a>, target: &LinkMLValue<'a>, ignore_missing_target: bool) -> Vec<Delta> {
    fn inner<'b>(path: &mut Vec<String>, s: &LinkMLValue<'b>, t: &LinkMLValue<'b>, ignore_missing: bool, out: &mut Vec<Delta>) {
        match (s, t) {
            (LinkMLValue::Map { values: sm, .. }, LinkMLValue::Map { values: tm, .. }) => {
                for (k, sv) in sm {
                    path.push(k.clone());
                    match tm.get(k) {
                        Some(tv) => inner(path, sv, tv, ignore_missing, out),
                        None => {
                            if !ignore_missing {
                                out.push(Delta { path: path.clone(), old: Some(sv.to_json()), new: None });
                            }
                        }
                    }
                    path.pop();
                }
                for (k, tv) in tm {
                    if !sm.contains_key(k) {
                        path.push(k.clone());
                        out.push(Delta { path: path.clone(), old: None, new: Some(tv.to_json()) });
                        path.pop();
                    }
                }
            }
            _ => {
                let sv = s.to_json();
                let tv = t.to_json();
                if sv != tv {
                    out.push(Delta { path: path.clone(), old: Some(sv), new: Some(tv) });
                }
            }
        }
    }
    let mut out = Vec::new();
    inner(&mut Vec::new(), source, target, ignore_missing_target, &mut out);
    out
}

pub fn patch<'a>(source: &'a LinkMLValue<'a>, deltas: &[Delta], sv: &'a SchemaView) -> LinkMLValue<'a> {
    let mut json = source.to_json();
    for d in deltas {
        apply_delta(&mut json, d);
    }
    let json_str = serde_json::to_string(&json).unwrap();
    let conv = sv.converter();
    match source {
        LinkMLValue::Map { class: Some(ref c), .. } => {
            load_json_str(&json_str, sv, Some(c), &conv).unwrap()
        }
        _ => load_json_str(&json_str, sv, None, &conv).unwrap(),
    }
}

fn apply_delta(value: &mut JsonValue, delta: &Delta) {
    apply_delta_inner(value, &delta.path, &delta.new);
}

fn apply_delta_inner(value: &mut JsonValue, path: &[String], newv: &Option<JsonValue>) {
    if path.is_empty() {
        if let Some(v) = newv { *value = v.clone(); }
        return;
    }
    match value {
        JsonValue::Object(map) => {
            let key = &path[0];
            if path.len() == 1 {
                match newv {
                    Some(v) => { map.insert(key.clone(), v.clone()); },
                    None => { map.remove(key); },
                }
            } else {
                let entry = map.entry(key.clone()).or_insert(JsonValue::Object(Map::new()));
                apply_delta_inner(entry, &path[1..], newv);
            }
        }
        _ => {
            if path.is_empty() {
                if let Some(v) = newv { *value = v.clone(); }
            }
        }
    }
}
