use crate::{LResult, LinkMLError, LinkMLValue, NodeId};
use linkml_schemaview::schemaview::{SchemaView, SlotView};
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
/// - X -> null: update to null (old = X, new = null).
/// - null -> X: update from null (old = null, new = X).
/// - missing -> X: add (old = None, new = X).
/// - X -> missing: ignored by default; if `treat_missing_as_null` is true, update to null (old = X, new = null).
pub fn diff(source: &LinkMLValue, target: &LinkMLValue, treat_missing_as_null: bool) -> Vec<Delta> {
    fn inner(
        path: &mut Vec<String>,
        slot: Option<&SlotView>,
        s: &LinkMLValue,
        t: &LinkMLValue,
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
                // If objects have an identifier or key slot and it changed, treat as whole-object replacement
                // This applies for single-valued and list-valued inlined objects.
                let key_slot_name = sc
                    .key_or_identifier_slot()
                    .or_else(|| tc.key_or_identifier_slot())
                    .map(|s| s.name.clone());
                if let Some(ks) = key_slot_name {
                    let sid = sm.get(&ks);
                    let tid = tm.get(&ks);
                    if let (
                        Some(LinkMLValue::Scalar { value: s_id, .. }),
                        Some(LinkMLValue::Scalar { value: t_id, .. }),
                    ) = (sid, tid)
                    {
                        if s_id != t_id {
                            out.push(Delta {
                                path: path.clone(),
                                old: Some(s.to_json()),
                                new: Some(t.to_json()),
                            });
                            return;
                        }
                    }
                }
                for (k, sv) in sm {
                    let slot_view = sc
                        .slots()
                        .iter()
                        .find(|s| s.name == *k)
                        .or_else(|| tc.slots().iter().find(|s| s.name == *k));
                    path.push(k.clone());
                    match tm.get(k) {
                        Some(tv) => inner(path, slot_view, sv, tv, treat_missing_as_null, out),
                        None => {
                            if !slot_view.is_some_and(slot_is_ignored) {
                                // Missing target slot: either ignore (default) or treat as update to null
                                if treat_missing_as_null {
                                    out.push(Delta {
                                        path: path.clone(),
                                        old: Some(sv.to_json()),
                                        new: Some(JsonValue::Null),
                                    });
                                }
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
                        (Some(sv), Some(tv)) => {
                            inner(path, None, sv, tv, treat_missing_as_null, out)
                        }
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
                        (Some(sv), Some(tv)) => {
                            inner(path, None, sv, tv, treat_missing_as_null, out)
                        }
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
                    old: Some(JsonValue::Null),
                    new: Some(tv.to_json()),
                });
            }
            (sv, LinkMLValue::Null { .. }) => {
                out.push(Delta {
                    path: path.clone(),
                    old: Some(sv.to_json()),
                    new: Some(JsonValue::Null),
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
        treat_missing_as_null,
        &mut out,
    );
    out
}

#[derive(Debug, Clone, Default)]
pub struct PatchTrace {
    /// Node IDs of subtrees that were newly created by the patch.
    ///
    /// See [`crate::NodeId`] for semantics: these are internal, ephemeral IDs
    /// that are useful for tooling and provenance, not object identifiers.
    pub added: Vec<NodeId>,
    /// Node IDs of subtrees that were removed by the patch.
    pub deleted: Vec<NodeId>,
    /// Node IDs of nodes that were directly updated (e.g., parent containers, scalars).
    pub updated: Vec<NodeId>,
}

pub fn patch(
    source: &LinkMLValue,
    deltas: &[Delta],
    sv: &SchemaView,
) -> LResult<(LinkMLValue, PatchTrace)> {
    let mut out = source.clone();
    let mut trace = PatchTrace::default();
    for d in deltas {
        apply_delta_linkml(&mut out, &d.path, &d.new, sv, &mut trace)?;
    }
    Ok((out, trace))
}

fn collect_all_ids(value: &LinkMLValue, ids: &mut Vec<NodeId>) {
    ids.push(value.node_id());
    match value {
        LinkMLValue::Scalar { .. } => {}
        LinkMLValue::Null { .. } => {}
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

// Removed thin wrappers; call LinkMLValue builders directly at call sites.

fn apply_delta_linkml(
    current: &mut LinkMLValue,
    path: &[String],
    newv: &Option<serde_json::Value>,
    sv: &SchemaView,
    trace: &mut PatchTrace,
) -> LResult<()> {
    if path.is_empty() {
        if let Some(v) = newv {
            let (class_opt, slot_opt) = match current {
                LinkMLValue::Object { class, .. } => (Some(class.clone()), None),
                LinkMLValue::List { class, slot, .. } => (class.clone(), Some(slot.clone())),
                LinkMLValue::Mapping { class, slot, .. } => (class.clone(), Some(slot.clone())),
                LinkMLValue::Scalar { class, slot, .. } => (class.clone(), Some(slot.clone())),
                LinkMLValue::Null { class, slot, .. } => (class.clone(), Some(slot.clone())),
            };
            let conv = sv.converter();
            if let Some(cls) = class_opt {
                let new_node = LinkMLValue::from_json(v.clone(), cls, slot_opt, sv, &conv, false)?;
                mark_deleted_subtree(current, trace);
                mark_added_subtree(&new_node, trace);
                *current = new_node;
            }
        }
        return Ok(());
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
                                    return Ok(());
                                }
                            }
                            let conv = sv.converter();
                            let slot = class.slots().iter().find(|s| s.name == *key).cloned();
                            let new_child = LinkMLValue::from_json(
                                v.clone(),
                                class.clone(),
                                slot,
                                sv,
                                &conv,
                                false,
                            )?;
                            let old_snapshot = std::mem::replace(old_child, new_child);
                            mark_deleted_subtree(&old_snapshot, trace);
                            mark_added_subtree(old_child, trace);
                            trace.updated.push(current.node_id());
                        } else {
                            let conv = sv.converter();
                            let slot = class.slots().iter().find(|s| s.name == *key).cloned();
                            let new_child = LinkMLValue::from_json(
                                v.clone(),
                                class.clone(),
                                slot,
                                sv,
                                &conv,
                                false,
                            )?;
                            // mark before insert
                            mark_added_subtree(&new_child, trace);
                            values.insert(key.clone(), new_child);
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
                apply_delta_linkml(child, &path[1..], newv, sv, trace)?;
            }
        }
        LinkMLValue::Mapping { values, slot, .. } => {
            let key = &path[0];
            if path.len() == 1 {
                match newv {
                    Some(v) => {
                        let conv = sv.converter();
                        let new_child = LinkMLValue::build_mapping_entry_for_slot(
                            slot,
                            v.clone(),
                            sv,
                            &conv,
                            Vec::new(),
                        )?;
                        if let Some(old_child) = values.get(key) {
                            mark_deleted_subtree(old_child, trace);
                        }
                        // mark before insert
                        mark_added_subtree(&new_child, trace);
                        values.insert(key.clone(), new_child);
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
                apply_delta_linkml(child, &path[1..], newv, sv, trace)?;
            }
        }
        LinkMLValue::List {
            values,
            slot,
            class,
            ..
        } => {
            let idx: usize = path[0].parse().map_err(|_| {
                LinkMLError(format!("invalid list index '{}' in delta path", path[0]))
            })?;
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
                                let conv = sv.converter();
                                let new_child = LinkMLValue::build_list_item_for_slot(
                                    slot,
                                    class.as_ref(),
                                    v.clone(),
                                    sv,
                                    &conv,
                                    Vec::new(),
                                )?;
                                let old = std::mem::replace(&mut values[idx], new_child);
                                mark_deleted_subtree(&old, trace);
                                mark_added_subtree(&values[idx], trace);
                                trace.updated.push(current.node_id());
                            }
                        } else if idx == values.len() {
                            let conv = sv.converter();
                            let new_child = LinkMLValue::build_list_item_for_slot(
                                slot,
                                class.as_ref(),
                                v.clone(),
                                sv,
                                &conv,
                                Vec::new(),
                            )?;
                            // mark before push
                            mark_added_subtree(&new_child, trace);
                            values.push(new_child);
                            trace.updated.push(current.node_id());
                        } else {
                            return Err(LinkMLError(format!(
                                "list index out of bounds in add: {} > {}",
                                idx,
                                values.len()
                            )));
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
                apply_delta_linkml(&mut values[idx], &path[1..], newv, sv, trace)?;
            }
        }
        LinkMLValue::Scalar { .. } => {}
        LinkMLValue::Null { .. } => {}
    }
    Ok(())
}
