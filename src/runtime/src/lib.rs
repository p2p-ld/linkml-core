use curies::Converter;
use linkml_schemaview::identifier::Identifier;
use linkml_schemaview::schemaview::{ClassView, SchemaView, SlotContainerMode, SlotView};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

pub mod diff;
pub mod turtle;
pub use diff::{diff, patch, Delta, PatchTrace};
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
    if slot.name == key || alt_names(key).contains(&slot.name) {
        return true;
    }
    if let Some(alias) = &slot.definition().alias {
        if alias == key || alt_names(key).iter().any(|a| alias == a) {
            return true;
        }
    }
    false
}

#[derive(Clone)]
pub enum LinkMLInstance {
    Scalar {
        node_id: NodeId,
        value: JsonValue,
        slot: SlotView,
        class: Option<ClassView>,
        sv: SchemaView,
    },
    Null {
        node_id: NodeId,
        slot: SlotView,
        class: Option<ClassView>,
        sv: SchemaView,
    },
    List {
        node_id: NodeId,
        values: Vec<LinkMLInstance>,
        slot: SlotView,
        class: Option<ClassView>,
        sv: SchemaView,
    },
    Mapping {
        node_id: NodeId,
        values: HashMap<String, LinkMLInstance>,
        slot: SlotView,
        class: Option<ClassView>,
        sv: SchemaView,
    },
    Object {
        node_id: NodeId,
        values: HashMap<String, LinkMLInstance>,
        class: ClassView,
        sv: SchemaView,
    },
}

/// Internal node identifier used for provenance and update tracking.
///
/// Node IDs are assigned to every `LinkMLInstance` node when values are constructed or
/// transformed. They exist solely as technical identifiers to help with patching and
/// provenance (for example, `PatchTrace.added`/`deleted` collect `NodeId`s of affected
/// subtrees). They are not intended to identify domain objects — for that, use LinkML
/// identifier or key slots as defined in the schema.
///
/// Important properties:
/// - Local and ephemeral: loading the same data twice will yield different `NodeId`s.
/// - Non-persistent: never serialize or expose as a model identifier.
/// - Useful for tracking modifications within a single in-memory value.
pub type NodeId = u64;

static NEXT_NODE_ID: AtomicU64 = AtomicU64::new(1);

fn new_node_id() -> NodeId {
    NEXT_NODE_ID.fetch_add(1, Ordering::Relaxed)
}

impl LinkMLInstance {
    /// Returns the internal [`NodeId`] of this node.
    ///
    /// This ID is only for internal provenance/update tracking and is not a
    /// semantic identifier of the represented object.
    pub fn node_id(&self) -> NodeId {
        match self {
            LinkMLInstance::Scalar { node_id, .. }
            | LinkMLInstance::List { node_id, .. }
            | LinkMLInstance::Mapping { node_id, .. }
            | LinkMLInstance::Object { node_id, .. }
            | LinkMLInstance::Null { node_id, .. } => *node_id,
        }
    }
    /// Navigate the value by a path of strings, where each element is either
    /// a dictionary key (for maps) or a list index (for lists).
    /// Returns `Some(&LinkMLInstance)` if the full path can be resolved, otherwise `None`.
    pub fn navigate_path<I, S>(&self, path: I) -> Option<&LinkMLInstance>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut current: &LinkMLInstance = self;
        for seg in path {
            let key = seg.as_ref();
            match current {
                LinkMLInstance::Object { values, .. } => {
                    current = values.get(key)?;
                }
                LinkMLInstance::List { values, .. } => {
                    let idx: usize = key.parse().ok()?;
                    current = values.get(idx)?;
                }
                LinkMLInstance::Mapping { values, .. } => {
                    current = values.get(key)?;
                }
                LinkMLInstance::Scalar { .. } => return None,
                LinkMLInstance::Null { .. } => return None,
            }
        }
        Some(current)
    }

    /// Compare two LinkMLInstance instances for semantic equality per the
    /// LinkML Instances specification (Identity conditions).
    ///
    /// Key points implemented:
    /// - Null equals Null.
    /// - Scalars: equal iff same underlying atomic value and compatible typed context
    ///   (same Enum range when present; otherwise same TypeDefinition range name when present).
    /// - Lists: equal iff same length and pairwise equal in order.
    /// - Mappings: equal iff same keys and values equal for each key (order-insensitive).
    /// - Objects: equal iff same instantiated class (by identity) and slot assignments match; when
    ///   `treat_missing_as_null` is true, Null is treated as omitted (normalized), otherwise Null is
    ///   distinct from missing.
    pub fn equals(&self, other: &LinkMLInstance, treat_missing_as_null: bool) -> bool {
        use LinkMLInstance::*;
        match (self, other) {
            (Null { .. }, Null { .. }) => true,
            (
                Scalar {
                    value: v1,
                    slot: s1,
                    ..
                },
                Scalar {
                    value: v2,
                    slot: s2,
                    ..
                },
            ) => {
                // If either slot has an enum range, both must and enum names must match
                let e1 = s1.get_range_enum();
                let e2 = s2.get_range_enum();
                if e1.is_some() || e2.is_some() {
                    match (e1, e2) {
                        (Some(ev1), Some(ev2)) => {
                            if ev1.schema_id() != ev2.schema_id() || ev1.name() != ev2.name() {
                                return false;
                            }
                        }
                        _ => return false,
                    }
                } else {
                    // Compare type ranges if explicitly set on both
                    let t1 = s1.definition().range.as_ref();
                    let t2 = s2.definition().range.as_ref();
                    if let (Some(r1), Some(r2)) = (t1, t2) {
                        if r1 != r2 {
                            return false;
                        }
                    }
                }
                v1 == v2
            }
            (List { values: a, .. }, List { values: b, .. }) => {
                if a.len() != b.len() {
                    return false;
                }
                for (x, y) in a.iter().zip(b.iter()) {
                    if !x.equals(y, treat_missing_as_null) {
                        return false;
                    }
                }
                true
            }
            (Mapping { values: a, .. }, Mapping { values: b, .. }) => {
                if a.len() != b.len() {
                    return false;
                }
                for (k, va) in a.iter() {
                    match b.get(k) {
                        Some(vb) => {
                            if !va.equals(vb, treat_missing_as_null) {
                                return false;
                            }
                        }
                        None => return false,
                    }
                }
                true
            }
            (
                Object {
                    values: a,
                    class: ca,
                    sv: sva,
                    ..
                },
                Object {
                    values: b,
                    class: cb,
                    sv: svb,
                    ..
                },
            ) => {
                // Compare class identity via canonical URIs if possible
                let ida = ca.canonical_uri();
                let idb = cb.canonical_uri();
                let class_equal = if let Some(conv) = sva.converter_for_schema(ca.schema_id()) {
                    // Use 'sva' for comparison; identifiers are global across schemas
                    sva.identifier_equals(&ida, &idb, conv).unwrap_or(false)
                } else if let Some(conv) = svb.converter_for_schema(cb.schema_id()) {
                    svb.identifier_equals(&ida, &idb, conv).unwrap_or(false)
                } else {
                    ca.name() == cb.name()
                };
                if !class_equal {
                    return false;
                }

                if treat_missing_as_null {
                    // Normalize conceptually by ignoring entries whose value is Null
                    let count_a = a.iter().filter(|(_, v)| !matches!(v, Null { .. })).count();
                    let count_b = b.iter().filter(|(_, v)| !matches!(v, Null { .. })).count();
                    if count_a != count_b {
                        return false;
                    }
                    for (k, va) in a.iter().filter(|(_, v)| !matches!(v, Null { .. })) {
                        match b.get(k) {
                            Some(vb) => {
                                if matches!(vb, Null { .. }) {
                                    return false;
                                }
                                if !va.equals(vb, treat_missing_as_null) {
                                    return false;
                                }
                            }
                            None => return false,
                        }
                    }
                    // Ensure b has no extra non-null keys not in a
                    for (k, _vb) in b.iter().filter(|(_, v)| !matches!(v, Null { .. })) {
                        match a.get(k) {
                            Some(va) => {
                                if matches!(va, Null { .. }) {
                                    return false;
                                }
                            }
                            None => return false,
                        }
                    }
                    true
                } else {
                    if a.len() != b.len() {
                        return false;
                    }
                    for (k, va) in a.iter() {
                        match b.get(k) {
                            Some(vb) => {
                                if !va.equals(vb, treat_missing_as_null) {
                                    return false;
                                }
                            }
                            None => return false,
                        }
                    }
                    true
                }
            }
            _ => false,
        }
    }
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
        let descendants = base.get_descendants(true, false).unwrap_or_default();
        let mut cand_refs: Vec<&ClassView> = vec![base];
        for d in &descendants {
            cand_refs.push(d);
        }

        let mut preferred: Option<&ClassView> = None;
        if let Some(td_slot) = base
            .slots()
            .iter()
            .find(|s| s.definition().designates_type.unwrap_or(false))
            .map(|s| s.definition())
        {
            if let Some(JsonValue::String(tv)) = map.get(&td_slot.name) {
                for c in &cand_refs {
                    if let Ok(vals) = c.get_accepted_type_designator_values(td_slot, conv) {
                        if vals.iter().any(|v| v.to_string() == *tv) {
                            preferred = Some(*c);
                            break;
                        }
                    }
                }
            }
        }
        if let Some(p) = preferred {
            return p.clone();
        }

        for c in &cand_refs {
            if let Ok(tmp) = Self::parse_object_fixed_class(map.clone(), c, sv, conv, Vec::new()) {
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
        Ok(LinkMLInstance::Object {
            node_id: new_node_id(),
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
                for (i, v) in arr.into_iter().enumerate() {
                    let mut p = path.clone();
                    p.push(format!("{}[{}]", sl.name, i));
                    values.push(Self::build_list_item_for_slot(
                        &sl,
                        Some(&class),
                        v,
                        sv,
                        conv,
                        p,
                    )?);
                }
                Ok(LinkMLInstance::List {
                    node_id: new_node_id(),
                    values,
                    slot: sl.clone(),
                    class: Some(class.clone()),
                    sv: sv.clone(),
                })
            }
            // Preserve explicit null as a Null value for list-valued slot
            (false, JsonValue::Null) => Ok(LinkMLInstance::Null {
                node_id: new_node_id(),
                slot: sl.clone(),
                class: Some(class.clone()),
                sv: sv.clone(),
            }),
            (false, other) => Err(LinkMLError(format!(
                "expected list for slot `{}`, found {:?} at {}",
                sl.name,
                other,
                path_to_string(&path)
            ))),
            (true, other) => Ok(LinkMLInstance::Scalar {
                node_id: new_node_id(),
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
                let mut values = HashMap::new();
                for (k, v) in map.into_iter() {
                    let child = Self::build_mapping_entry_for_slot(sl, v, sv, conv, {
                        let mut p = path.clone();
                        p.push(k.clone());
                        p
                    })?;
                    values.insert(k, child);
                }
                Ok(LinkMLInstance::Mapping {
                    node_id: new_node_id(),
                    values,
                    slot: sl.clone(),
                    class: class.clone(),
                    sv: sv.clone(),
                })
            }
            // Preserve explicit null as a Null value for mapping-valued slot
            JsonValue::Null => Ok(LinkMLInstance::Null {
                node_id: new_node_id(),
                slot: sl.clone(),
                class: class.clone(),
                sv: sv.clone(),
            }),
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
            LinkMLError(format!(
                "list requires slot at {} for class={}",
                path_to_string(&path),
                class.name()
            ))
        })?;
        let mut values = Vec::new();
        for (i, v) in arr.into_iter().enumerate() {
            let mut p = path.clone();
            p.push(format!("[{}]", i));
            values.push(Self::build_list_item_for_slot(
                &sl,
                Some(&class),
                v,
                sv,
                conv,
                p,
            )?);
        }
        Ok(LinkMLInstance::List {
            node_id: new_node_id(),
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
        }
        .ok_or_else(|| {
            LinkMLError(format!(
                "object requires class or slot at {}",
                path_to_string(&path)
            ))
        })?;
        let chosen = Self::select_class(&map, &base_class, sv, conv);

        let mut values = HashMap::new();
        for (k, v) in map.into_iter() {
            let slot_tmp: Option<SlotView> = chosen
                .slots()
                .iter()
                .find(|s| slot_matches_key(s, &k))
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
        Ok(LinkMLInstance::Object {
            node_id: new_node_id(),
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
                path_to_string(&path),
                classview_name
            ))
        })?;
        if value.is_null() {
            Ok(LinkMLInstance::Null {
                node_id: new_node_id(),
                slot: sl,
                class: Some(class.clone()),
                sv: sv.clone(),
            })
        } else {
            Ok(LinkMLInstance::Scalar {
                node_id: new_node_id(),
                value,
                slot: sl,
                class: Some(class.clone()),
                sv: sv.clone(),
            })
        }
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
            other => Self::parse_scalar_value(other, classview, slot, sv, path),
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

    // Shared builders (used by loaders and patch logic)
    pub(crate) fn build_list_item_for_slot(
        list_slot: &SlotView,
        list_class: Option<&ClassView>,
        value: JsonValue,
        sv: &SchemaView,
        conv: &Converter,
        path: Vec<String>,
    ) -> LResult<Self> {
        let class_range: Option<ClassView> = list_slot.get_range_class();
        let slot_for_item = if class_range.is_some() {
            None
        } else {
            Some(list_slot.clone())
        };
        let v_transformed = if let (Some(cr), JsonValue::String(s)) = (class_range.as_ref(), &value)
        {
            if let Some(id_slot) = cr.identifier_slot() {
                let mut m = serde_json::Map::new();
                m.insert(id_slot.name.clone(), JsonValue::String(s.clone()));
                JsonValue::Object(m)
            } else {
                value
            }
        } else {
            value
        };
        Self::from_json_internal(
            v_transformed,
            class_range
                .as_ref()
                .or(list_class)
                .cloned()
                .ok_or_else(|| LinkMLError("list item class context".to_string()))?,
            slot_for_item,
            sv,
            conv,
            true,
            path,
        )
    }

    pub(crate) fn build_mapping_entry_for_slot(
        map_slot: &SlotView,
        value: JsonValue,
        sv: &SchemaView,
        conv: &Converter,
        path: Vec<String>,
    ) -> LResult<Self> {
        let range_cv = map_slot
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
        match value {
            JsonValue::Object(m) => {
                let selected = Self::select_class(&m, &range_cv, sv, conv);
                let mut child_values = HashMap::new();
                for (ck, cv) in m.into_iter() {
                    let slot_tmp = selected
                        .slots()
                        .iter()
                        .find(|s| slot_matches_key(s, &ck))
                        .cloned();
                    let mut p = path.clone();
                    p.push(ck.clone());
                    let key_name = slot_tmp
                        .as_ref()
                        .map(|s| s.name.clone())
                        .unwrap_or_else(|| ck.clone());
                    child_values.insert(
                        key_name,
                        Self::from_json_internal(
                            cv,
                            selected.clone(),
                            slot_tmp,
                            sv,
                            conv,
                            false,
                            p,
                        )?,
                    );
                }
                Ok(LinkMLInstance::Object {
                    node_id: new_node_id(),
                    values: child_values,
                    class: selected,
                    sv: sv.clone(),
                })
            }
            other => {
                let key_slot_name = range_cv
                    .key_or_identifier_slot()
                    .map(|s| s.name.as_str())
                    .unwrap_or("");
                let scalar_slot = Self::find_scalar_slot_for_inlined_map(&range_cv, key_slot_name)
                    .ok_or_else(|| {
                        LinkMLError(format!(
                            "no scalar slot available for inlined mapping at {}",
                            path_to_string(&path)
                        ))
                    })?;
                let mut child_values = HashMap::new();
                child_values.insert(
                    scalar_slot.name.clone(),
                    LinkMLInstance::Scalar {
                        node_id: new_node_id(),
                        value: other,
                        slot: scalar_slot.clone(),
                        class: Some(range_cv.clone()),
                        sv: sv.clone(),
                    },
                );
                Ok(LinkMLInstance::Object {
                    node_id: new_node_id(),
                    values: child_values,
                    class: range_cv,
                    sv: sv.clone(),
                })
            }
        }
    }
}

pub fn load_yaml_file(
    path: &Path,
    sv: &SchemaView,
    class: &ClassView,
    conv: &Converter,
) -> std::result::Result<LinkMLInstance, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    load_yaml_str(&text, sv, class, conv)
}

pub fn load_yaml_str(
    data: &str,
    sv: &SchemaView,
    class: &ClassView,
    conv: &Converter,
) -> std::result::Result<LinkMLInstance, Box<dyn std::error::Error>> {
    let value: serde_yaml::Value = serde_yaml::from_str(data)?;
    let json = serde_json::to_value(value)?;
    LinkMLInstance::from_json(json, class.clone(), None, sv, conv, false)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

pub fn load_json_file(
    path: &Path,
    sv: &SchemaView,
    class: &ClassView,
    conv: &Converter,
) -> std::result::Result<LinkMLInstance, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    load_json_str(&text, sv, class, conv)
}

pub fn load_json_str(
    data: &str,
    sv: &SchemaView,
    class: &ClassView,
    conv: &Converter,
) -> std::result::Result<LinkMLInstance, Box<dyn std::error::Error>> {
    let value: JsonValue = serde_json::from_str(data)?;
    LinkMLInstance::from_json(value, class.clone(), None, sv, conv, false)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

fn validate_inner(value: &LinkMLInstance) -> std::result::Result<(), String> {
    match value {
        LinkMLInstance::Scalar {
            value: jv, slot, ..
        } => {
            if let Some(ev) = slot.get_range_enum() {
                // For now, enforce that enum-backed slots take string values that
                // are among the permissible value keys.
                let s = match jv {
                    JsonValue::String(s) => s,
                    other => {
                        return Err(format!(
                            "expected string for enum '{}' in slot '{}', found {:?}",
                            ev.name(),
                            slot.name,
                            other
                        ))
                    }
                };
                let keys = ev
                    .permissible_value_keys()
                    .map_err(|e| format!("{:?}", e))?;
                if !keys.contains(s) {
                    return Err(format!(
                        "invalid enum value '{}' for slot '{}' (allowed: {:?})",
                        s, slot.name, keys
                    ));
                }
            }
            Ok(())
        }
        LinkMLInstance::Null { .. } => Ok(()),
        LinkMLInstance::List { values, .. } => {
            for v in values {
                validate_inner(v)?;
            }
            Ok(())
        }
        LinkMLInstance::Mapping { values, .. } => {
            for v in values.values() {
                validate_inner(v)?;
            }
            Ok(())
        }
        LinkMLInstance::Object { values, class, .. } => {
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

pub fn validate(value: &LinkMLInstance) -> std::result::Result<(), String> {
    validate_inner(value)
}

fn validate_collect(value: &LinkMLInstance, errors: &mut Vec<String>) {
    match value {
        LinkMLInstance::Scalar { .. } => {}
        LinkMLInstance::Null { .. } => {}
        LinkMLInstance::List { values, .. } => {
            for v in values {
                validate_collect(v, errors);
            }
        }
        LinkMLInstance::Mapping { values, .. } => {
            for v in values.values() {
                validate_collect(v, errors);
            }
        }
        LinkMLInstance::Object { values, class, .. } => {
            for (k, v) in values {
                if class.slots().iter().all(|s| s.name != *k) {
                    errors.push(format!("unknown slot `{}` for class `{}`", k, class.name()));
                }
                validate_collect(v, errors);
            }
        }
    }
}

pub fn validate_errors(value: &LinkMLInstance) -> Vec<String> {
    let mut errs = Vec::new();
    validate_collect(value, &mut errs);
    errs
}
