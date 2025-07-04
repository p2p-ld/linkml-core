use std::collections::HashMap;

use crate::identifier::{
    converter_from_schema, converter_from_schemas, Identifier, IdentifierError,
};
use curies::Converter;
use linkml_meta::SchemaDefinition;

use crate::curie::curie2uri;
// re-export views from submodules
pub use crate::classview::ClassView;
pub use crate::slotview::{SlotContainerMode, SlotInlineMode, SlotView};

#[derive(Debug)]
pub enum SchemaViewError {
    IdentifierError(IdentifierError),
    NoPrimarySchema(String),
    NotFound,
}

impl From<IdentifierError> for SchemaViewError {
    fn from(err: IdentifierError) -> Self {
        SchemaViewError::IdentifierError(err)
    }
}


pub struct SchemaView {
    pub(crate) schema_definitions: HashMap<String, SchemaDefinition>,
    pub(crate) primary_schema: Option<String>,
    pub(crate) class_uri_index: HashMap<String, (String, String)>,
    pub(crate) class_name_index: HashMap<String, (String, String)>,
    pub(crate) slot_uri_index: HashMap<String, (String, String)>,
    pub(crate) slot_name_index: HashMap<String, (String, String)>,
}

impl SchemaView {
    pub fn new() -> Self {
        SchemaView {
            schema_definitions: HashMap::new(),
            primary_schema: None,
            class_uri_index: HashMap::new(),
            slot_uri_index: HashMap::new(),
            class_name_index: HashMap::new(),
            slot_name_index: HashMap::new(),
        }
    }

    pub fn get_tree_root_or(&self, class_name: Option<&str>) -> Option<ClassView<'_>> {
        let converter = self.converter_for_primary_schema()?;
        // if there is a class_name then its simple!
        if let Some(name) = class_name {
            if let Ok(Some(cv)) = self.get_class(&Identifier::new(name), &converter) {
                return Some(cv);
            }
        } else {
            // find a class with tree_root set to true in the primary schema
            if let Some(primary) = &self.primary_schema {
                if let Some(schema) = self.schema_definitions.get(primary) {
                    if let Some(classes) = &schema.classes {
                        for (name, class_def) in classes {
                            if class_def.tree_root.is_some_and(|x| x == true) {
                                if let Ok(Some(cv)) =
                                    self.get_class(&Identifier::new(name), &converter)
                                {
                                    return Some(cv);
                                }
                            }
                        }
                    }
                }
            }
        }
        return None;
        // only search in the primary schema for a class having an attribute `tree_root`
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

    pub fn iter_schemas(&self) -> std::collections::hash_map::Iter<'_, String, SchemaDefinition> {
        self.schema_definitions.iter()
    }

    pub fn converter(&self) -> Converter {
        converter_from_schemas(self.schema_definitions.values())
    }

    pub fn converter_for_schema(&self, schema_uri: &str) -> Option<Converter> {
        self.schema_definitions
            .get(schema_uri)
            .map(|s| converter_from_schema(s))
    }

    pub fn converter_for_primary_schema(&self) -> Option<Converter> {
        self.primary_schema
            .as_ref()
            .and_then(|uri| self.converter_for_schema(uri))
    }

    pub fn get_enum_definition(
        &self,
        _identifier: &Identifier,
    ) -> Option<linkml_meta::EnumDefinition> {
        // TODO implement this
        None
    }

    fn index_schema_classes(
        &mut self,
        schema_uri: &str,
        schema: &SchemaDefinition,
        conv: &Converter,
    ) -> Result<(), IdentifierError> {
        let default_prefix = schema.default_prefix.as_deref().unwrap_or(&schema.name);
        if let Some(classes) = &schema.classes {
            for (class_name, class_def) in classes {
                let default_id = if class_name.contains(':') && class_def.class_uri.is_none() {
                    Identifier::new(class_name)
                } else {
                    Identifier::new(&format!("{}:{}", default_prefix, class_name))
                };
                let default_uri = default_id.to_uri(conv).map(|u| u.0).unwrap_or_else(|_| {
                    format!("{}/{}", schema.id.trim_end_matches('/'), class_name)
                });
                self.class_name_index
                    .entry(class_name.clone())
                    .or_insert_with(|| (schema_uri.to_string(), class_name.clone()));

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
        if let Some(definitions) = &schema.slot_definitions {
            for (slot_name, slot_def) in definitions {
                let default_id = if slot_name.contains(':') && slot_def.slot_uri.is_none() {
                    Identifier::new(slot_name)
                } else {
                    Identifier::new(&format!("{}:{}", default_prefix, slot_name))
                };
                let default_uri = default_id.to_uri(conv).map(|u| u.0).unwrap_or_else(|_| {
                    format!("{}/{}", schema.id.trim_end_matches('/'), slot_name)
                });
                self.slot_name_index
                    .entry(slot_name.clone())
                    .or_insert_with(|| (schema_uri.to_string(), slot_name.clone()));
                /*if let Some(s) = &slot_def.alias {
                    self.slot_name_index
                        .entry(s.clone())
                        .or_insert_with(|| (schema_uri.to_string(), slot_name.clone()));
                }*/

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
        }
        Ok(())
    }

    pub fn get_unresolved_schemas(&self) -> Vec<String> {
        // every schemadefinition has imports. check if an import is not in our list
        let mut unresolved = Vec::new();
        for (_name, schema) in &self.schema_definitions {
            if let Some(imports) = &schema.imports {
                for import in imports {
                    let import_uri = curie2uri(import, schema.prefixes.as_ref());
                    match import_uri {
                        Some(uri) => {
                            if !self.schema_definitions.contains_key(&uri) {
                                unresolved.push(uri);
                            }
                        }
                        None => {
                            // if the import cannot be expanded to a URI, treat it as a
                            // potential local file path and attempt to resolve later
                            let path = import.to_string();
                            if !self.schema_definitions.contains_key(&path) {
                                unresolved.push(path);
                            }
                        }
                    }
                }
            }
        }
        unresolved
    }

    pub fn get_class_by_schema<'a>(
        &'a self,
        schema_uri: &str,
        class_name: &str,
    ) -> Result<Option<ClassView<'a>>, SchemaViewError> {
        if let Some(schema_def) = self.schema_definitions.get(schema_uri) {
            if let Some(classes) = &schema_def.classes {
                if let Some(class_def) = classes.get(class_name) {
                    return Ok(Some(ClassView::new(
                        class_def, self, schema_uri, schema_def,
                        &self.converter_for_schema(schema_uri).unwrap(),
                    )?));
                }
            }
        }
        Ok(None)
    }
    

    pub fn get_class<'a>(
        &'a self,
        id: &Identifier,
        conv: &Converter,
    ) -> Result<Option<ClassView<'a>>, SchemaViewError> {
        let index = &self.class_uri_index;
        match id {
            Identifier::Name(name) => {
                if let Some((schema, c)) = self.class_name_index.get(name) {
                    if let Some(schema_def) = self.schema_definitions.get(schema) {
                        if let Some(classes) = &schema_def.classes {
                            if let Some(class_def) = classes.get(c) {
                                return Ok(Some(ClassView::new(
                                    class_def, self, schema, schema_def, conv,
                                )?));
                            }
                        }
                    }
                }
                Ok(None)
            }
            Identifier::Curie(_) | Identifier::Uri(_) => {
                let target_uri = id
                    .to_uri(conv)
                    .map_err(|e| SchemaViewError::IdentifierError(e))?;
                if let Some((schema_uri, class_name)) = index.get(&target_uri.0) {
                    if let Some(schema) = self.schema_definitions.get(schema_uri) {
                        if let Some(classes) = &schema.classes {
                            if let Some(class) = classes.get(class_name) {
                                return Ok(Some(ClassView::new(
                                    class, self, schema_uri, schema, conv,
                                )?));
                            }
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
        fn alt_names(name: &str) -> Vec<String> {
            let mut v = Vec::new();
            v.push(name.to_string());
            if name.contains('_') {
                v.push(name.replace('_', " "));
            }
            if name.contains(' ') {
                v.push(name.replace(' ', "_"));
            }
            v
        }
        match id {
            Identifier::Name(name) => {
                let names = alt_names(name);
                for name in names {
                    if let Some((schema, slot_name)) = self.slot_name_index.get(&name) {
                        if let Some(schema_def) = self.schema_definitions.get(schema) {
                            if let Some(defs) = &schema_def.slot_definitions {
                                if let Some(slot) = defs.get(slot_name) {
                                    return Ok(Some(SlotView::new(
                                        slot_name.clone(),
                                        vec![slot],
                                        &schema_def.id,
                                        self,
                                        schema_def,
                                    )));
                                }
                            }
                        }
                    }
                }
                Ok(None)
            }
            Identifier::Curie(_) | Identifier::Uri(_) => {
                let target_uri = id.to_uri(conv)?;
                if let Some((schema_uri, slot_name)) = index.get(&target_uri.0) {
                    if let Some(schema) = self.schema_definitions.get(schema_uri) {
                        if let Some(defs) = &schema.slot_definitions {
                            if let Some(slot) = defs.get(slot_name) {
                                return Ok(Some(SlotView::new(
                                    slot_name.clone(),
                                    vec![slot],
                                    &schema.id,
                                    self,
                                    schema,
                                )));
                            }
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
                        if let Some(types) = &schema.types {
                            if let Some(t) = types.get(n) {
                                return Ok(Some(t));
                            }
                        }
                    }
                    Ok(None)
                }
                Identifier::Curie(_) | Identifier::Uri(_) => {
                    let target_uri = id.to_uri(conv)?;
                    for schema in sv.schema_definitions.values() {
                        if let Some(types) = &schema.types {
                            for t in types.values() {
                                if let Some(turi) = &t.type_uri {
                                    if Identifier::new(turi).to_uri(conv)?.0 == target_uri.0 {
                                        return Ok(Some(t));
                                    }
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
