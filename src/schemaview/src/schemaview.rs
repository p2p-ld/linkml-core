use std::collections::HashMap;

use linkml_meta::SchemaDefinition;

use crate::curie::curie2uri;

pub struct SchemaView {
    schema_definitions: HashMap<String, SchemaDefinition>,
    primary_schema: Option<String>,
}

impl SchemaView {
    pub fn new() -> Self {
        SchemaView {
            schema_definitions: HashMap::new(),
            primary_schema: None,
        }
    }

    pub fn add_schema(&mut self, schema: SchemaDefinition) -> Result<(), String> {
        let schema_uri = schema.id.clone();
        self.schema_definitions.insert(schema_uri.to_string(), schema);
        if self.primary_schema.is_none() {
            self.primary_schema = Some(schema_uri.to_string());
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
                    },
                    None => {
                    }
                }
            }
        }
        unresolved
    }

}
