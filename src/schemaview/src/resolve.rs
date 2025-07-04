use crate::{
    io::{from_uri, from_yaml},
    schemaview::SchemaView,
};
use std::path::Path;

fn get_uri_for_id(id: &str) -> Option<&'static str> {
    match id {
        "https://w3id.org/linkml/mappings" => Some("https://raw.githubusercontent.com/linkml/linkml-model/refs/heads/main/linkml_model/model/schema/mappings.yaml"),
        "https://w3id.org/linkml/types" => Some("https://raw.githubusercontent.com/linkml/linkml-model/refs/heads/main/linkml_model/model/schema/types.yaml"),
        "https://w3id.org/linkml/extensions" => Some("https://raw.githubusercontent.com/linkml/linkml-model/refs/heads/main/linkml_model/model/schema/extensions.yaml"),
        "https://w3id.org/linkml/annotations" => Some("https://raw.githubusercontent.com/linkml/linkml-model/refs/heads/main/linkml_model/model/schema/annotations.yaml"),
        "https://w3id.org/linkml/units" => Some("https://raw.githubusercontent.com/linkml/linkml-model/refs/heads/main/linkml_model/model/schema/units.yaml"),
        _ => None,
    }
}

pub fn resolve_schemas(sv: &mut SchemaView) -> Result<(), String> {
    let unresolved = sv.get_unresolved_schemas();
    if unresolved.is_empty() {
        return Ok(());
    }

    for uri in unresolved {
        if let Some(resolved_uri) = get_uri_for_id(&uri) {
            // Load the schema from the resolved URI
            let schema = match from_uri(resolved_uri) {
                Ok(s) => s,
                Err(e) => {
                    return Err(format!(
                        "Failed to load schema from {}: {}",
                        resolved_uri, e
                    ))
                }
            };
            sv.add_schema(schema)?;
        } else {
            // Attempt to treat the unresolved entry as a local file path
            let path = Path::new(&uri);
            if path.exists() {
                let mut schema = match from_yaml(path) {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(format!(
                            "Failed to load schema from {}: {}",
                            path.display(),
                            e
                        ))
                    }
                };
                sv.add_schema(schema.clone())?;
                if schema.id != uri.to_string() {
                    panic!("Schema ID '{}' does not match URI '{}'.", schema.id, uri);
                }

            } else {
                return Err(format!("No resolution found for URI: {}", uri));
            }
        }
    }
    Ok(())
}
