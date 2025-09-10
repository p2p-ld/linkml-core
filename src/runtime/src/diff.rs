use crate::{load_json_str, LinkMLValue, NodeId};
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

pub fn diff(source: &LinkMLValue, target: &LinkMLValue, ignore_missing_target: bool) -> Vec<Delta> {
    fn inner(
        path: &mut Vec<String>,
        slot: Option<&SlotView>,
        s: &LinkMLValue,
        t: &LinkMLValue,
        ignore_missing: bool,
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
                        Some(tv) => inner(path, slot_view, sv, tv, ignore_missing, out),
                        None => {
                            if !ignore_missing && !slot_view.is_some_and(slot_is_ignored) {
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
                        (Some(sv), Some(tv)) => inner(path, None, sv, tv, ignore_missing, out),
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
                        (Some(sv), Some(tv)) => inner(path, None, sv, tv, ignore_missing, out),
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
            _ => {
                let sv = s.to_json();
                let tv = t.to_json();
                if sv != tv {
                    out.push(Delta {
                        path: path.clone(),
                        old: Some(sv),
                        new: Some(tv),
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
        &mut out,
    );
    out
}

#[derive(Debug, Clone, Default)]
pub struct PatchTrace {
    pub added: Vec<NodeId>,
    pub deleted: Vec<NodeId>,
    pub updated: Vec<NodeId>,
}

fn collect_ids(value: &LinkMLValue, out: &mut Vec<NodeId>) {
    out.push(value.node_id());
    match value {
        LinkMLValue::Scalar { .. } => {}
        LinkMLValue::List { values, .. } => {
            for v in values {
                collect_ids(v, out);
            }
        }
        LinkMLValue::Mapping { values, .. } | LinkMLValue::Object { values, .. } => {
            for v in values.values() {
                collect_ids(v, out);
            }
        }
    }
}

fn reconcile_ids(old: &LinkMLValue, new: &mut LinkMLValue) {
    match (old, new) {
        (LinkMLValue::Scalar { node_id: oid, .. }, LinkMLValue::Scalar { node_id: nid, .. }) => {
            *nid = *oid;
        }
        (
            LinkMLValue::List {
                node_id: oid,
                values: ov,
                ..
            },
            LinkMLValue::List {
                node_id: nid,
                values: nv,
                ..
            },
        ) => {
            *nid = *oid;
            let n = std::cmp::min(ov.len(), nv.len());
            for i in 0..n {
                reconcile_ids(&ov[i], &mut nv[i]);
            }
        }
        (
            LinkMLValue::Mapping {
                node_id: oid,
                values: om,
                ..
            },
            LinkMLValue::Mapping {
                node_id: nid,
                values: nm,
                ..
            },
        ) => {
            *nid = *oid;
            for (k, ov) in om {
                if let Some(nv) = nm.get_mut(k) {
                    reconcile_ids(ov, nv);
                }
            }
        }
        (
            LinkMLValue::Object {
                node_id: oid,
                values: om,
                ..
            },
            LinkMLValue::Object {
                node_id: nid,
                values: nm,
                ..
            },
        ) => {
            *nid = *oid;
            for (k, ov) in om {
                if let Some(nv) = nm.get_mut(k) {
                    reconcile_ids(ov, nv);
                }
            }
        }
        _ => {}
    }
}

fn collect_updated(old: &LinkMLValue, new: &LinkMLValue, updated: &mut Vec<NodeId>) {
    if old.node_id() == new.node_id() && old.to_json() != new.to_json() {
        updated.push(new.node_id());
    }
    match (old, new) {
        (LinkMLValue::Scalar { .. }, LinkMLValue::Scalar { .. }) => {}
        (LinkMLValue::List { values: ov, .. }, LinkMLValue::List { values: nv, .. }) => {
            let n = std::cmp::min(ov.len(), nv.len());
            for i in 0..n {
                collect_updated(&ov[i], &nv[i], updated);
            }
        }
        (
            LinkMLValue::Mapping { values: om, .. },
            LinkMLValue::Mapping { values: nm, .. },
        )
        | (
            LinkMLValue::Object { values: om, .. },
            LinkMLValue::Object { values: nm, .. },
        ) => {
            for (k, ov) in om {
                if let Some(nv) = nm.get(k) {
                    collect_updated(ov, nv, updated);
                }
            }
        }
        _ => {}
    }
}

pub fn patch(source: &LinkMLValue, deltas: &[Delta], sv: &SchemaView) -> (LinkMLValue, PatchTrace) {
    // Pre-snapshot
    let mut pre_ids = Vec::new();
    collect_ids(source, &mut pre_ids);

    let mut json = source.to_json();
    for d in deltas {
        apply_delta(&mut json, d);
    }
    let json_str = serde_json::to_string(&json).unwrap();
    let conv = sv.converter();
    let mut new_value = match source {
        LinkMLValue::Object { class: ref c, .. } => load_json_str(&json_str, sv, c, &conv).unwrap(),
        _ => panic!("patching non-map values is not supported here"),
    };

    reconcile_ids(source, &mut new_value);

    let mut post_ids = Vec::new();
    collect_ids(&new_value, &mut post_ids);

    use std::collections::HashSet;
    let pre: HashSet<NodeId> = pre_ids.into_iter().collect();
    let post: HashSet<NodeId> = post_ids.into_iter().collect();
    let added: Vec<NodeId> = post.difference(&pre).copied().collect();
    let deleted: Vec<NodeId> = pre.difference(&post).copied().collect();

    let mut updated = Vec::new();
    collect_updated(source, &new_value, &mut updated);

    (
        new_value,
        PatchTrace {
            added,
            deleted,
            updated,
        },
    )
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
