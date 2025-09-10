use crate::{LinkMLValue, NodeId};
use linkml_schemaview::schemaview::{ClassView, SchemaView, SlotView};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

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

pub fn patch(source: &LinkMLValue, deltas: &[Delta], sv: &SchemaView) -> (LinkMLValue, PatchTrace) {
    let mut out = source.clone();
    let mut trace = PatchTrace::default();
    for d in deltas {
        apply_delta_linkml(&mut out, &d.path, &d.new, sv, &mut trace);
    }
    (out, trace)
}

fn collect_all_ids(value: &LinkMLValue, ids: &mut Vec<NodeId>) {
    ids.push(value.node_id());
    match value {
        LinkMLValue::Scalar { .. } => {}
        LinkMLValue::List { values, .. } => {
            for v in values {
                collect_all_ids(v, ids);
            }
        }
        LinkMLValue::Mapping { values, .. } | LinkMLValue::Object { values, .. } => {
            for v in values.values() {
                collect_all_ids(v, ids);
            }
        }
    }
}

fn mark_added_subtree(v: &LinkMLValue, trace: &mut PatchTrace) {
    collect_all_ids(v, &mut trace.added);
}

fn mark_deleted_subtree(v: &LinkMLValue, trace: &mut PatchTrace) {
    collect_all_ids(v, &mut trace.deleted);
}

fn build_from_json_with_context(
    json: serde_json::Value,
    class_ctx: ClassView,
    slot_ctx: Option<SlotView>,
    sv: &SchemaView,
    allow_inlined: bool,
) -> LinkMLValue {
    let conv = sv.converter();
    LinkMLValue::from_json(json, class_ctx, slot_ctx, sv, &conv, allow_inlined)
        .expect("failed to construct LinkMLValue from JSON")
}

fn build_for_object_slot(
    parent_class: &ClassView,
    key: &str,
    json: serde_json::Value,
    sv: &SchemaView,
) -> LinkMLValue {
    let slot = parent_class.slots().iter().find(|s| s.name == key).cloned();
    build_from_json_with_context(json, parent_class.clone(), slot, sv, false)
}

fn build_for_list_item(
    list_slot: &SlotView,
    list_class: Option<&ClassView>,
    json: serde_json::Value,
    sv: &SchemaView,
) -> LinkMLValue {
    let conv = sv.converter();
    LinkMLValue::build_list_item_for_slot(list_slot, list_class, json, sv, &conv, Vec::new())
        .expect("failed to parse list item")
}


fn build_for_mapping_value(
    map_slot: &SlotView,
    json: serde_json::Value,
    sv: &SchemaView,
) -> LinkMLValue {
    let conv = sv.converter();
    LinkMLValue::build_mapping_entry_for_slot(map_slot, json, sv, &conv, Vec::new())
        .expect("failed to parse mapping value")
}

fn apply_delta_linkml(
    current: &mut LinkMLValue,
    path: &[String],
    newv: &Option<serde_json::Value>,
    sv: &SchemaView,
    trace: &mut PatchTrace,
) {
    if path.is_empty() {
        if let Some(v) = newv {
            let (class_opt, slot_opt) = match current {
                LinkMLValue::Object { class, .. } => (Some(class.clone()), None),
                LinkMLValue::List { class, slot, .. } => (class.clone(), Some(slot.clone())),
                LinkMLValue::Mapping { class, slot, .. } => (class.clone(), Some(slot.clone())),
                LinkMLValue::Scalar { class, slot, .. } => (class.clone(), Some(slot.clone())),
            };
            let conv = sv.converter();
            if let Some(cls) = class_opt {
                let new_node = LinkMLValue::from_json(v.clone(), cls, slot_opt, sv, &conv, false)
                    .expect("failed to parse replacement value");
                mark_deleted_subtree(current, trace);
                mark_added_subtree(&new_node, trace);
                *current = new_node;
            }
        }
        return;
    }

    match current {
        LinkMLValue::Object { values, class, .. } => {
            let key = &path[0];
            if path.len() == 1 {
                match newv {
                    Some(v) => {
                        if let Some(old_child) = values.get_mut(key) {
                            if let LinkMLValue::Scalar { value, .. } = old_child {
                                if !v.is_object() && !v.is_array() {
                                    *value = v.clone();
                                    trace.updated.push(old_child.node_id());
                                    return;
                                }
                            }
                            let new_child = build_for_object_slot(class, key, v.clone(), sv);
                            let old_snapshot = std::mem::replace(old_child, new_child);
                            mark_deleted_subtree(&old_snapshot, trace);
                            mark_added_subtree(values.get(key).unwrap(), trace);
                            trace.updated.push(current.node_id());
                        } else {
                            let new_child = build_for_object_slot(class, key, v.clone(), sv);
                            values.insert(key.clone(), new_child);
                            mark_added_subtree(values.get(key).unwrap(), trace);
                            trace.updated.push(current.node_id());
                        }
                    }
                    None => {
                        if let Some(old_child) = values.remove(key) {
                            mark_deleted_subtree(&old_child, trace);
                            trace.updated.push(current.node_id());
                        }
                    }
                }
            } else if let Some(child) = values.get_mut(key) {
                apply_delta_linkml(child, &path[1..], newv, sv, trace);
            }
        }
        LinkMLValue::Mapping { values, slot, .. } => {
            let key = &path[0];
            if path.len() == 1 {
                match newv {
                    Some(v) => {
                        let new_child = build_for_mapping_value(slot, v.clone(), sv);
                        if let Some(old_child) = values.get(key) {
                            mark_deleted_subtree(old_child, trace);
                        }
                        values.insert(key.clone(), new_child);
                        mark_added_subtree(values.get(key).unwrap(), trace);
                        trace.updated.push(current.node_id());
                    }
                    None => {
                        if let Some(old_child) = values.remove(key) {
                            mark_deleted_subtree(&old_child, trace);
                            trace.updated.push(current.node_id());
                        }
                    }
                }
            } else if let Some(child) = values.get_mut(key) {
                apply_delta_linkml(child, &path[1..], newv, sv, trace);
            }
        }
        LinkMLValue::List {
            values,
            slot,
            class,
            ..
        } => {
            let idx: usize = path[0].parse().expect("invalid list index in delta path");
            if path.len() == 1 {
                match newv {
                    Some(v) => {
                        if idx < values.len() {
                            let is_scalar_target =
                                matches!(values[idx], LinkMLValue::Scalar { .. });
                            if is_scalar_target && !v.is_object() && !v.is_array() {
                                if let LinkMLValue::Scalar { value, .. } = &mut values[idx] {
                                    *value = v.clone();
                                    trace.updated.push(values[idx].node_id());
                                }
                            } else {
                                let new_child =
                                    build_for_list_item(slot, class.as_ref(), v.clone(), sv);
                                let old = std::mem::replace(&mut values[idx], new_child);
                                mark_deleted_subtree(&old, trace);
                                mark_added_subtree(&values[idx], trace);
                                trace.updated.push(current.node_id());
                            }
                        } else if idx == values.len() {
                            let new_child =
                                build_for_list_item(slot, class.as_ref(), v.clone(), sv);
                            values.push(new_child);
                            mark_added_subtree(values.last().unwrap(), trace);
                            trace.updated.push(current.node_id());
                        } else {
                            panic!(
                                "list index out of bounds in add: {} > {}",
                                idx,
                                values.len()
                            );
                        }
                    }
                    None => {
                        if idx < values.len() {
                            let old = values.remove(idx);
                            mark_deleted_subtree(&old, trace);
                            trace.updated.push(current.node_id());
                        }
                    }
                }
            } else if idx < values.len() {
                apply_delta_linkml(&mut values[idx], &path[1..], newv, sv, trace);
            }
        }
        LinkMLValue::Scalar { .. } => {}
    }
}
