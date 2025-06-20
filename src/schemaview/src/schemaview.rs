use std::collections::{HashMap, HashSet};

use crate::identifier::{
    converter_from_schema, converter_from_schemas, Identifier, IdentifierError,
};
use curies::Converter;
use linkml_meta::{ClassDefinition, SchemaDefinition, SlotDefinition};

use crate::curie::curie2uri;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotContainerMode {
    SingleValue,
    Mapping,
    List,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotInlineMode {
    Inline,
    Primitive,
    Reference,
}

pub struct SchemaView {
    schema_definitions: HashMap<String, SchemaDefinition>,
    primary_schema: Option<String>,
    class_uri_index: HashMap<String, (String, String)>,
    slot_uri_index: HashMap<String, (String, String)>,
}

#[derive(Clone)]
pub struct ClassView<'a> {
    pub class: &'a ClassDefinition,
    slots: Vec<SlotView<'a>>,
    schema_uri: &'a str,
    sv: &'a SchemaView,
}

impl<'a> ClassView<'a> {
    pub fn new(
        class: &'a ClassDefinition,
        sv: &'a SchemaView,
        schema_uri: &'a str,
        conv: &Converter,
    ) -> Result<Self, IdentifierError> {
        fn gather<'b>(
            class_def: &'b ClassDefinition,
            sv: &'b SchemaView,
            conv: &Converter,
            visited: &mut HashSet<String>,
            acc: &mut HashMap<String, SlotView<'b>>,
        ) -> Result<(), IdentifierError> {
            if !visited.insert(class_def.name.clone()) {
                return Ok(());
            }

            if let Some(parent) = &class_def.is_a {
                if let Some(cv) = sv.get_class(&Identifier::new(parent), conv)? {
                    gather(cv.class, sv, conv, visited, acc)?;
                }
            }
            for mixin in &class_def.mixins {
                if let Some(cv) = sv.get_class(&Identifier::new(mixin), conv)? {
                    gather(cv.class, sv, conv, visited, acc)?;
                }
            }

            for slot_ref in &class_def.slots {
                let mut defs: Vec<&'b SlotDefinition> = Vec::new();
                if let Some(base) = sv.get_slot(&Identifier::new(slot_ref), conv)? {
                    defs.extend(base.definitions);
                }
                if let Some(usage) = class_def.slot_usage.get(slot_ref) {
                    defs.push(usage);
                }
                acc.insert(
                    slot_ref.clone(),
                    SlotView {
                        name: slot_ref.clone(),
                        definitions: defs,
                    },
                );
            }

            for (attr_name, attr_def) in &class_def.attributes {
                let mut defs = vec![attr_def.as_ref()];
                if let Some(usage) = class_def.slot_usage.get(attr_name) {
                    defs.push(usage);
                }
                acc.insert(
                    attr_name.clone(),
                    SlotView {
                        name: attr_name.clone(),
                        definitions: defs,
                    },
                );
            }

            for (usage_name, usage_def) in &class_def.slot_usage {
                if !class_def.slots.contains(usage_name)
                    && !class_def.attributes.contains_key(usage_name)
                {
                    // overrides slot from ancestor
                    if let Some(existing) = acc.get_mut(usage_name) {
                        existing.definitions.push(usage_def);
                    } else if let Some(base) = sv.get_slot(&Identifier::new(usage_name), conv)? {
                        let mut defs: Vec<&'b SlotDefinition> = base.definitions.to_vec();
                        defs.push(usage_def);
                        acc.insert(
                            usage_name.clone(),
                            SlotView {
                                name: usage_name.clone(),
                                definitions: defs,
                            },
                        );
                    } else {
                        acc.insert(
                            usage_name.clone(),
                            SlotView {
                                name: usage_name.clone(),
                                definitions: vec![usage_def],
                            },
                        );
                    }
                }
            }
            Ok(())
        }

        let mut visited = HashSet::new();
        let mut acc: HashMap<String, SlotView<'a>> = HashMap::new();
        gather(class, sv, conv, &mut visited, &mut acc)?;
        Ok(Self {
            class,
            slots: acc.into_values().collect(),
            schema_uri,
            sv,
        })
    }

    pub fn slots(&self) -> &[SlotView<'a>] {
        &self.slots
    }

    pub fn get_uri(
        &self,
        conv: &Converter,
        native: bool,
        expand: bool,
    ) -> Result<Identifier, IdentifierError> {
        let schema = self
            .sv
            .schema_definitions
            .get(self.schema_uri)
            .ok_or_else(|| IdentifierError::NameNotResolvable)?;
        let default_prefix = schema.default_prefix.as_deref().unwrap_or(&schema.name);
        let base = if native || self.class.class_uri.is_none() {
            format!("{}:{}", default_prefix, self.class.name)
        } else {
            self.class.class_uri.as_ref().unwrap().clone()
        };

        if expand {
            Ok(Identifier::Uri(Identifier::new(&base).to_uri(conv)?))
        } else {
            match Identifier::new(&base) {
                Identifier::Curie(c) => Ok(Identifier::Curie(c)),
                Identifier::Uri(_) => Ok(Identifier::Curie(Identifier::new(&base).to_curie(conv)?)),
                Identifier::Name(_) => {
                    Ok(Identifier::Curie(Identifier::new(&base).to_curie(conv)?))
                }
            }
        }
    }

    pub fn get_type_designator_value(
        &self,
        type_slot: &SlotDefinition,
        conv: &Converter,
    ) -> Result<Identifier, IdentifierError> {
        if let Some(range) = &type_slot.range {
            let slot_types = self.sv.type_ancestors(&Identifier::new(range), conv)?;
            if slot_types.iter().any(|t| t.to_string() == "uri") {
                return self.get_uri(conv, false, true);
            } else if slot_types.iter().any(|t| t.to_string() == "uriorcurie") {
                return self.get_uri(conv, false, false);
            } else if slot_types.iter().any(|t| t.to_string() == "string") {
                return Ok(Identifier::Name(self.class.name.clone()));
            }
        }
        self.get_uri(conv, false, false)
    }

    pub fn get_accepted_type_designator_values(
        &self,
        type_slot: &SlotDefinition,
        conv: &Converter,
    ) -> Result<Vec<Identifier>, IdentifierError> {
        let mut vals = vec![
            self.get_uri(conv, true, true)?,
            self.get_uri(conv, false, true)?,
            self.get_uri(conv, true, false)?,
            self.get_uri(conv, false, false)?,
        ];
        let mut seen = std::collections::HashSet::new();
        vals.retain(|v| seen.insert(v.to_string()));

        if let Some(range) = &type_slot.range {
            let slot_types = self.sv.type_ancestors(&Identifier::new(range), conv)?;
            if slot_types
                .iter()
                .any(|t| t.to_string() == "uri" || t.to_string() == "uriorcurie")
            {
                return Ok(vals);
            } else if range == "string" {
                return Ok(vec![Identifier::Name(self.class.name.clone())]);
            }
        }
        Ok(vals)
    }

    pub fn get_descendants(
        &self,
        conv: &Converter,
        recurse: bool,
    ) -> Result<Vec<ClassView<'a>>, IdentifierError> {
        let mut out = Vec::new();
        for schema in self.sv.schema_definitions.values() {
            for (cls_name, cls_def) in &schema.classes {
                if let Some(parent) = &cls_def.is_a {
                    if let Some(parent_cv) = self.sv.get_class(&Identifier::new(parent), conv)? {
                        if parent_cv.class.name == self.class.name
                            && parent_cv.schema_uri == self.schema_uri
                        {
                            if let Some(child_cv) =
                                self.sv.get_class(&Identifier::new(cls_name), conv)?
                            {
                                out.push(child_cv.clone());
                                if recurse {
                                    out.extend(child_cv.get_descendants(conv, true)?);
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(out)
    }

    pub fn key_or_identifier_slot(&'a self) -> Option<&'a SlotView<'a>> {
        self.slots.iter().find(|s| {
            let d = s.merged_definition();
            d.identifier.unwrap_or(false) || d.key.unwrap_or(false)
        })
    }

    pub fn identifier_slot(&'a self) -> Option<&'a SlotView<'a>> {
        self.slots
            .iter()
            .find(|s| s.merged_definition().identifier.unwrap_or(false))
    }
}

#[derive(Clone)]
pub struct SlotView<'a> {
    pub name: String,
    pub definitions: Vec<&'a SlotDefinition>,
}

impl<'a> SlotView<'a> {
    pub fn new(name: String, slot: &'a SlotDefinition) -> Self {
        Self {
            name,
            definitions: vec![slot],
        }
    }

    pub fn merged_definition(&self) -> SlotDefinition {
        let mut base = self.definitions[0].clone();
        for d in self.definitions.iter().skip(1) {
            if let Some(v) = &d.range {
                base.range = Some(v.clone());
            }
            if let Some(v) = d.multivalued {
                base.multivalued = Some(v);
            }
            if let Some(v) = d.inlined {
                base.inlined = Some(v);
            }
            if let Some(v) = d.inlined_as_list {
                base.inlined_as_list = Some(v);
            }
            if let Some(v) = d.key {
                base.key = Some(v);
            }
            if let Some(v) = d.identifier {
                base.identifier = Some(v);
            }
        }
        base
    }

    pub fn determine_slot_container_mode(
        &self,
        sv: &'a SchemaView,
        conv: &Converter,
    ) -> SlotContainerMode {
        let s = self.merged_definition();
        let multivalued = s.multivalued.unwrap_or(false);
        let class_range = match &s.range {
            Some(r) => sv
                .get_class(&Identifier::new(r), conv)
                .ok()
                .flatten()
                .is_some(),
            None => false,
        };

        if !class_range {
            return if multivalued {
                SlotContainerMode::List
            } else {
                SlotContainerMode::SingleValue
            };
        }

        if multivalued && s.inlined_as_list.unwrap_or(false) {
            return SlotContainerMode::List;
        }

        let range_cv = s
            .range
            .as_ref()
            .and_then(|r| sv.get_class(&Identifier::new(r), conv).ok().flatten());
        let key_slot = range_cv.as_ref().and_then(|cv| cv.key_or_identifier_slot());
        let identifier_slot = range_cv.as_ref().and_then(|cv| cv.identifier_slot());

        let mut inlined = s.inlined.unwrap_or(false);
        if identifier_slot.is_none() {
            inlined = true;
        }

        if !multivalued {
            return SlotContainerMode::SingleValue;
        }

        if !inlined {
            return SlotContainerMode::List;
        }

        if key_slot.is_some() {
            SlotContainerMode::Mapping
        } else {
            SlotContainerMode::List
        }
    }

    pub fn determine_slot_inline_mode(
        &self,
        sv: &'a SchemaView,
        conv: &Converter,
    ) -> SlotInlineMode {
        let s = self.merged_definition();
        let multivalued = s.multivalued.unwrap_or(false);
        let class_range = match &s.range {
            Some(r) => sv
                .get_class(&Identifier::new(r), conv)
                .ok()
                .flatten()
                .is_some(),
            None => false,
        };

        if !class_range {
            return SlotInlineMode::Primitive;
        }

        if multivalued && s.inlined_as_list.unwrap_or(false) {
            return SlotInlineMode::Inline;
        }

        let range_cv = s
            .range
            .as_ref()
            .and_then(|r| sv.get_class(&Identifier::new(r), conv).ok().flatten());
        let identifier_slot = range_cv.as_ref().and_then(|cv| cv.identifier_slot());

        let mut inlined = s.inlined.unwrap_or(false);
        if identifier_slot.is_none() {
            inlined = true;
        }

        if !multivalued {
            return if inlined {
                SlotInlineMode::Inline
            } else {
                SlotInlineMode::Reference
            };
        }

        if !inlined {
            SlotInlineMode::Reference
        } else {
            SlotInlineMode::Inline
        }
    }

    pub fn determine_slot_mode(
        &self,
        sv: &'a SchemaView,
        conv: &Converter,
    ) -> (SlotContainerMode, SlotInlineMode) {
        (
            self.determine_slot_container_mode(sv, conv),
            self.determine_slot_inline_mode(sv, conv),
        )
    }

    pub fn can_contain_reference_to_class(
        &self,
        cls: &ClassView<'a>,
        sv: &'a SchemaView,
        conv: &Converter,
    ) -> bool {
        let mut classes_to_check = match self.merged_definition().range {
            Some(r) => vec![r],
            None => return false,
        };
        let mut seen = HashSet::new();
        let target = &cls.class.name;

        while let Some(c) = classes_to_check.pop() {
            if !seen.insert(c.clone()) {
                continue;
            }
            if let Ok(Some(cv)) = sv.get_class(&Identifier::new(&c), conv) {
                if cv.class.name == *target {
                    return true;
                }
                for slot in cv.slots() {
                    if let Some(r) = &slot.merged_definition().range {
                        if !seen.contains(r) {
                            classes_to_check.push(r.clone());
                        }
                    }
                }
            }
        }
        false
    }
}

impl SchemaView {
    pub fn new() -> Self {
        SchemaView {
            schema_definitions: HashMap::new(),
            primary_schema: None,
            class_uri_index: HashMap::new(),
            slot_uri_index: HashMap::new(),
        }
    }

    pub fn add_schema(&mut self, schema: SchemaDefinition) -> Result<(), String> {
        let schema_uri = schema.id.clone();
        let conv = converter_from_schema(&schema);
        self.index_schema_classes(&schema_uri, &schema, &conv)
            .map_err(|e| format!("{:?}", e))?;
        self.index_schema_slots(&schema_uri, &schema, &conv)
            .map_err(|e| format!("{:?}", e))?;
        self.schema_definitions
            .insert(schema_uri.to_string(), schema);
        if self.primary_schema.is_none() {
            self.primary_schema = Some(schema_uri.to_string());
        }
        Ok(())
    }

    pub fn get_schema(&self, id: &str) -> Option<&SchemaDefinition> {
        self.schema_definitions.get(id)
    }

    pub fn converter(&self) -> Converter {
        converter_from_schemas(self.schema_definitions.values())
    }

    pub fn get_class_definition<'a>(
        &'a self,
        id: &Identifier,
        conv: &Converter,
    ) -> Result<Option<&'a ClassDefinition>, IdentifierError> {
        Ok(self.get_class(id, conv)?.map(|cv| cv.class))
    }

    fn index_schema_classes(
        &mut self,
        schema_uri: &str,
        schema: &SchemaDefinition,
        conv: &Converter,
    ) -> Result<(), IdentifierError> {
        let default_prefix = schema.default_prefix.as_deref().unwrap_or(&schema.name);
        for (class_name, class_def) in &schema.classes {
            let default_uri = Identifier::new(&format!("{}:{}", default_prefix, class_name))
                .to_uri(conv)
                .map(|u| u.0)
                .unwrap_or_else(|_| format!("{}/{}", schema.id.trim_end_matches('/'), class_name));

            if let Some(curi) = &class_def.class_uri {
                let explicit_uri = Identifier::new(curi).to_uri(conv)?.0;
                self.class_uri_index
                    .entry(explicit_uri.clone())
                    .or_insert_with(|| (schema_uri.to_string(), class_name.clone()));
                if explicit_uri != default_uri {
                    self.class_uri_index
                        .entry(default_uri.clone())
                        .or_insert_with(|| (schema_uri.to_string(), class_name.clone()));
                }
            } else {
                self.class_uri_index
                    .entry(default_uri)
                    .or_insert_with(|| (schema_uri.to_string(), class_name.clone()));
            }
        }
        Ok(())
    }

    fn index_schema_slots(
        &mut self,
        schema_uri: &str,
        schema: &SchemaDefinition,
        conv: &Converter,
    ) -> Result<(), IdentifierError> {
        let default_prefix = schema.default_prefix.as_deref().unwrap_or(&schema.name);
        for (slot_name, slot_def) in &schema.slot_definitions {
            let default_uri = Identifier::new(&format!("{}:{}", default_prefix, slot_name))
                .to_uri(conv)
                .map(|u| u.0)
                .unwrap_or_else(|_| format!("{}/{}", schema.id.trim_end_matches('/'), slot_name));

            if let Some(suri) = &slot_def.slot_uri {
                let explicit_uri = Identifier::new(suri).to_uri(conv)?.0;
                self.slot_uri_index
                    .entry(explicit_uri.clone())
                    .or_insert_with(|| (schema_uri.to_string(), slot_name.clone()));
                if explicit_uri != default_uri {
                    self.slot_uri_index
                        .entry(default_uri.clone())
                        .or_insert_with(|| (schema_uri.to_string(), slot_name.clone()));
                }
            } else {
                self.slot_uri_index
                    .entry(default_uri)
                    .or_insert_with(|| (schema_uri.to_string(), slot_name.clone()));
            }
        }
        Ok(())
    }

    pub fn get_unresolved_schemas(&self) -> Vec<String> {
        // every schemadefinition has imports. check if an import is not in our list
        let mut unresolved = Vec::new();
        for (_name, schema) in &self.schema_definitions {
            for import in &schema.imports {
                let import_uri = curie2uri(import, &schema.prefixes);
                match import_uri {
                    Some(uri) => {
                        if !self.schema_definitions.contains_key(&uri) {
                            unresolved.push(uri);
                        }
                    }
                    None => {}
                }
            }
        }
        unresolved
    }

    pub fn get_class<'a>(
        &'a self,
        id: &Identifier,
        conv: &Converter,
    ) -> Result<Option<ClassView<'a>>, IdentifierError> {
        let index = &self.class_uri_index;
        match id {
            Identifier::Name(name) => {
                let primary = match &self.primary_schema {
                    Some(p) => p,
                    None => return Ok(None),
                };
                let schema = match self.schema_definitions.get(primary) {
                    Some(s) => s,
                    None => return Ok(None),
                };
                if let Some(class_def) = schema.classes.get(name) {
                    return Ok(Some(ClassView::new(class_def, self, primary, conv)?));
                }
                Ok(None)
            }
            Identifier::Curie(_) | Identifier::Uri(_) => {
                let target_uri = id.to_uri(conv)?;
                if let Some((schema_uri, class_name)) = index.get(&target_uri.0) {
                    if let Some(schema) = self.schema_definitions.get(schema_uri) {
                        if let Some(class) = schema.classes.get(class_name) {
                            return Ok(Some(ClassView::new(class, self, schema_uri, conv)?));
                        }
                    }
                }
                Ok(None)
            }
        }
    }

    pub fn get_slot<'a>(
        &'a self,
        id: &Identifier,
        conv: &Converter,
    ) -> Result<Option<SlotView<'a>>, IdentifierError> {
        let index = &self.slot_uri_index;
        match id {
            Identifier::Name(name) => {
                let primary = match &self.primary_schema {
                    Some(p) => p,
                    None => return Ok(None),
                };
                let schema = match self.schema_definitions.get(primary) {
                    Some(s) => s,
                    None => return Ok(None),
                };
                Ok(schema
                    .slot_definitions
                    .get(name)
                    .map(|s| SlotView::new(name.clone(), s)))
            }
            Identifier::Curie(_) | Identifier::Uri(_) => {
                let target_uri = id.to_uri(conv)?;
                if let Some((schema_uri, slot_name)) = index.get(&target_uri.0) {
                    if let Some(schema) = self.schema_definitions.get(schema_uri) {
                        if let Some(slot) = schema.slot_definitions.get(slot_name) {
                            return Ok(Some(SlotView::new(slot_name.clone(), slot)));
                        }
                    }
                }
                Ok(None)
            }
        }
    }

    pub fn type_ancestors(
        &self,
        id: &Identifier,
        conv: &Converter,
    ) -> Result<Vec<Identifier>, IdentifierError> {
        fn get_type<'b>(
            sv: &'b SchemaView,
            id: &Identifier,
            conv: &Converter,
        ) -> Result<Option<&'b linkml_meta::TypeDefinition>, IdentifierError> {
            match id {
                Identifier::Name(n) => {
                    for schema in sv.schema_definitions.values() {
                        if let Some(t) = schema.types.get(n) {
                            return Ok(Some(t));
                        }
                    }
                    Ok(None)
                }
                Identifier::Curie(_) | Identifier::Uri(_) => {
                    let target_uri = id.to_uri(conv)?;
                    for schema in sv.schema_definitions.values() {
                        for t in schema.types.values() {
                            if let Some(turi) = &t.type_uri {
                                if Identifier::new(turi).to_uri(conv)?.0 == target_uri.0 {
                                    return Ok(Some(t));
                                }
                            }
                        }
                    }
                    Ok(None)
                }
            }
        }

        let mut out = Vec::new();
        let mut cur = get_type(self, id, conv)?;
        while let Some(t) = cur {
            out.push(Identifier::Name(t.name.clone()));
            if let Some(parent) = &t.typeof_ {
                cur = get_type(self, &Identifier::new(parent), conv)?;
            } else {
                break;
            }
        }
        if out.is_empty() {
            out.push(id.clone());
        }
        Ok(out)
    }

    pub fn primary_schema(&self) -> Option<&SchemaDefinition> {
        match &self.primary_schema {
            Some(uri) => self.schema_definitions.get(uri),
            None => None,
        }
    }
}
