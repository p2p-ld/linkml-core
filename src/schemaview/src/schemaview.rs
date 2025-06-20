use std::collections::HashMap;

use crate::identifier::{converter_from_schema, Identifier, IdentifierError};
use curies::Converter;
use linkml_meta::{ClassDefinition, SchemaDefinition};

use crate::curie::curie2uri;

pub struct SchemaView {
    schema_definitions: HashMap<String, SchemaDefinition>,
    primary_schema: Option<String>,
    class_uri_index: HashMap<String, (String, String)>,
}

pub struct ClassView<'a> {
    pub class: &'a ClassDefinition,
}

impl<'a> ClassView<'a> {
    pub fn new(class: &'a ClassDefinition) -> Self {
        Self { class }
    }
}

impl SchemaView {
    pub fn new() -> Self {
        SchemaView {
            schema_definitions: HashMap::new(),
            primary_schema: None,
            class_uri_index: HashMap::new(),
        }
    }

    pub fn add_schema(&mut self, schema: SchemaDefinition) -> Result<(), String> {
        let schema_uri = schema.id.clone();
        let conv = converter_from_schema(&schema);
        self.index_schema_classes(&schema_uri, &schema, &conv)
            .map_err(|e| format!("{:?}", e))?;
        self.schema_definitions.insert(schema_uri.to_string(), schema);
        if self.primary_schema.is_none() {
            self.primary_schema = Some(schema_uri.to_string());
        }
        Ok(())
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
                .unwrap_or_else(|_| {
                    format!("{}/{}", schema.id.trim_end_matches('/'), class_name)
                });

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
                Ok(schema.classes.get(name).map(|c| ClassView::new(c)))
            }
            Identifier::Curie(_) | Identifier::Uri(_) => {
                let target_uri = id.to_uri(conv)?;
                if let Some((schema_uri, class_name)) = index.get(&target_uri.0) {
                    if let Some(schema) = self.schema_definitions.get(schema_uri) {
                        if let Some(class) = schema.classes.get(class_name) {
                            return Ok(Some(ClassView::new(class)));
                        }
                    }
                }
                Ok(None)
            }
        }
    }
}
