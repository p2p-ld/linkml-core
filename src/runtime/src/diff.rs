use crate::{load_json_str, LinkMLValue};
use linkml_schemaview::schemaview::{SchemaView, SlotView};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value as JsonValue};

const IGNORE_ANNOTATION: &str = "diff.linkml.io/ignore";

fn slot_is_ignored(slot: &SlotView) -> bool {
    if slot.definitions().is_empty() {
        return false;
    }
    slot.definition()
        .annotations
        .as_ref()
        .map(|a| a.contains_key(IGNORE_ANNOTATION))
        .unwrap_or(false)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Delta {
    pub path: Vec<String>,
    pub old: Option<JsonValue>,
    pub new: Option<JsonValue>,
}

impl LinkMLValue {
    pub fn to_json(&self) -> JsonValue {
        match self {
            LinkMLValue::Scalar { value, .. } => value.clone(),
            LinkMLValue::Null { .. } => JsonValue::Null,
            LinkMLValue::List { values, .. } => {
                JsonValue::Array(values.iter().map(|v| v.to_json()).collect())
            }
            LinkMLValue::Mapping { values, .. } => JsonValue::Object(
                values
                    .iter()
                    .map(|(k, v)| (k.clone(), v.to_json()))
                    .collect(),
            ),
            LinkMLValue::Object { values, .. } => JsonValue::Object(
                values
                    .iter()
                    .map(|(k, v)| (k.clone(), v.to_json()))
                    .collect(),
            ),
        }
    }
}

/// Compute a semantic diff between two LinkMLValue trees.
///
/// Semantics of nulls and missing values:
/// - X -> null: produces a removal delta (new = None).
/// - null -> X: produces an add delta (old = None, new = X).
/// - missing -> X: produces an add delta (old = None, new = X).
/// - X -> missing:
///   - if `treat_missing_as_null` is true: produces a removal delta (new = None).
///   - else if `ignore_missing_target` is true: ignored (no delta).
///   - else: produces a removal delta (new = None), matching previous behavior.
pub fn diff(
    source: &LinkMLValue,
    target: &LinkMLValue,
    ignore_missing_target: bool,
    treat_missing_as_null: bool,
) -> Vec<Delta> {
    fn inner(
        path: &mut Vec<String>,
        slot: Option<&SlotView>,
        s: &LinkMLValue,
        t: &LinkMLValue,
        ignore_missing: bool,
        treat_missing_as_null: bool,
        out: &mut Vec<Delta>,
    ) {
        if let Some(sl) = slot {
            if slot_is_ignored(sl) {
                return;
            }
        }
        match (s, t) {
            (
                LinkMLValue::Object {
                    values: sm,
                    class: sc,
                    ..
                },
                LinkMLValue::Object {
                    values: tm,
                    class: tc,
                    ..
                },
            ) => {
                for (k, sv) in sm {
                    let slot_view = sc
                        .slots()
                        .iter()
                        .find(|s| s.name == *k)
                        .or_else(|| tc.slots().iter().find(|s| s.name == *k));
                    path.push(k.clone());
                    match tm.get(k) {
                        Some(tv) => inner(
                            path,
                            slot_view,
                            sv,
                            tv,
                            ignore_missing,
                            treat_missing_as_null,
                            out,
                        ),
                        None => {
                            if !slot_view.is_some_and(slot_is_ignored)
                                && (treat_missing_as_null || !ignore_missing)
                            {
                                out.push(Delta {
                                    path: path.clone(),
                                    old: Some(sv.to_json()),
                                    new: None,
                                });
                            }
                        }
                    }
                    path.pop();
                }
                for (k, tv) in tm {
                    if !sm.contains_key(k) {
                        let slot_view = sc
                            .slots()
                            .iter()
                            .find(|s| s.name == *k)
                            .or_else(|| tc.slots().iter().find(|s| s.name == *k));
                        if !slot_view.is_some_and(slot_is_ignored) {
                            path.push(k.clone());
                            out.push(Delta {
                                path: path.clone(),
                                old: None,
                                new: Some(tv.to_json()),
                            });
                            path.pop();
                        }
                    }
                }
            }
            (LinkMLValue::List { values: sl, .. }, LinkMLValue::List { values: tl, .. }) => {
                let max_len = std::cmp::max(sl.len(), tl.len());
                for i in 0..max_len {
                    path.push(i.to_string());
                    match (sl.get(i), tl.get(i)) {
                        (Some(sv), Some(tv)) => inner(
                            path,
                            None,
                            sv,
                            tv,
                            ignore_missing,
                            treat_missing_as_null,
                            out,
                        ),
                        (Some(sv), None) => out.push(Delta {
                            path: path.clone(),
                            old: Some(sv.to_json()),
                            new: None,
                        }),
                        (None, Some(tv)) => out.push(Delta {
                            path: path.clone(),
                            old: None,
                            new: Some(tv.to_json()),
                        }),
                        (None, None) => {}
                    }
                    path.pop();
                }
            }
            (LinkMLValue::Mapping { values: sm, .. }, LinkMLValue::Mapping { values: tm, .. }) => {
                use std::collections::BTreeSet;
                let keys: BTreeSet<_> = sm.keys().chain(tm.keys()).cloned().collect();
                for k in keys {
                    path.push(k.clone());
                    match (sm.get(&k), tm.get(&k)) {
                        (Some(sv), Some(tv)) => inner(
                            path,
                            None,
                            sv,
                            tv,
                            ignore_missing,
                            treat_missing_as_null,
                            out,
                        ),
                        (Some(sv), None) => out.push(Delta {
                            path: path.clone(),
                            old: Some(sv.to_json()),
                            new: None,
                        }),
                        (None, Some(tv)) => out.push(Delta {
                            path: path.clone(),
                            old: None,
                            new: Some(tv.to_json()),
                        }),
                        (None, None) => {}
                    }
                    path.pop();
                }
            }
            (LinkMLValue::Null { .. }, LinkMLValue::Null { .. }) => {}
            (LinkMLValue::Null { .. }, tv) => {
                out.push(Delta {
                    path: path.clone(),
                    old: None,
                    new: Some(tv.to_json()),
                });
            }
            (sv, LinkMLValue::Null { .. }) => {
                out.push(Delta {
                    path: path.clone(),
                    old: Some(sv.to_json()),
                    new: None,
                });
            }
            (sv, tv) => {
                let sj = sv.to_json();
                let tj = tv.to_json();
                if sj != tj {
                    out.push(Delta {
                        path: path.clone(),
                        old: Some(sj),
                        new: Some(tj),
                    });
                }
            }
        }
    }
    let mut out = Vec::new();
    inner(
        &mut Vec::new(),
        None,
        source,
        target,
        ignore_missing_target,
        treat_missing_as_null,
        &mut out,
    );
    out
}

pub fn patch(source: &LinkMLValue, deltas: &[Delta], sv: &SchemaView) -> LinkMLValue {
    let mut json = source.to_json();
    for d in deltas {
        apply_delta(&mut json, d);
    }
    let json_str = serde_json::to_string(&json).unwrap();
    let conv = sv.converter();
    match source {
        LinkMLValue::Object { class: ref c, .. } => load_json_str(&json_str, sv, c, &conv).unwrap(),
        _ => panic!("patching non-map values is not supported here"),
    }
}

fn apply_delta(value: &mut JsonValue, delta: &Delta) {
    apply_delta_inner(value, &delta.path, &delta.new);
}

fn apply_delta_inner(value: &mut JsonValue, path: &[String], newv: &Option<JsonValue>) {
    if path.is_empty() {
        if let Some(v) = newv {
            *value = v.clone();
        }
        return;
    }
    match value {
        JsonValue::Object(map) => {
            let key = &path[0];
            if path.len() == 1 {
                match newv {
                    Some(v) => {
                        map.insert(key.clone(), v.clone());
                    }
                    None => {
                        map.remove(key);
                    }
                }
            } else {
                let entry = map
                    .entry(key.clone())
                    .or_insert(JsonValue::Object(Map::new()));
                apply_delta_inner(entry, &path[1..], newv);
            }
        }
        JsonValue::Array(arr) => {
            let idx: usize = path[0].parse().unwrap();
            if path.len() == 1 {
                match newv {
                    Some(v) => {
                        if idx < arr.len() {
                            arr[idx] = v.clone();
                        } else if idx == arr.len() {
                            arr.push(v.clone());
                        } else {
                            while arr.len() < idx {
                                arr.push(JsonValue::Null);
                            }
                            arr.push(v.clone());
                        }
                    }
                    None => {
                        if idx < arr.len() {
                            arr.remove(idx);
                        }
                    }
                }
            } else {
                if idx >= arr.len() {
                    arr.resize(idx + 1, JsonValue::Null);
                }
                apply_delta_inner(&mut arr[idx], &path[1..], newv);
            }
        }
        _ => {
            if path.is_empty() {
                if let Some(v) = newv {
                    *value = v.clone();
                }
            }
        }
    }
}
