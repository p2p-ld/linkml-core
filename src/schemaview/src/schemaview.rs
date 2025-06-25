use std::collections::HashMap;

use crate::identifier::{
    converter_from_schema, converter_from_schemas, Identifier, IdentifierError,
};
use curies::Converter;
use linkml_meta::{ClassDefinition, SchemaDefinition};

use crate::curie::curie2uri;
// re-export views from submodules
pub use crate::classview::ClassView;
pub use crate::slotview::{SlotContainerMode, SlotInlineMode, SlotView};

pub struct SchemaView {
    pub(crate) schema_definitions: HashMap<String, SchemaDefinition>,
    pub(crate) primary_schema: Option<String>,
    pub(crate) class_uri_index: HashMap<String, (String, String)>,
    pub(crate) slot_uri_index: HashMap<String, (String, String)>,
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

    pub fn iter_schemas(&self) -> std::collections::hash_map::Iter<'_, String, SchemaDefinition> {
        self.schema_definitions.iter()
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
