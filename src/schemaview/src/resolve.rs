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

    for (schema_id, uri) in unresolved {
        let schema_source_uri = sv.get_resolution_uri_of_schema(&schema_id);
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
            sv.add_schema_with_import_ref(schema, Some((schema_id, uri)))?;
        } else {
            // Attempt to treat the unresolved entry as a local file path
            let mut path = Path::new(&uri).to_owned();
            if !path.is_absolute() {
                // imported_from_dir = parent from schema_source_uri
                let imported_from_dir = Path::new(schema_source_uri.as_deref().unwrap_or("unknown")).parent().map(|p| p.to_path_buf());
                if let Some(dir) = imported_from_dir {
                    path = dir.join(path);                    
                }
                if (!path.exists() && path.with_extension("yaml").exists()) {
                    path.set_extension("yaml");
                }
                if (!path.exists() && path.with_extension("yml").exists()) {
                    path.set_extension("yml");
                }
            }
            if path.exists() {
                let schema = match from_yaml(&path) {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(format!(
                            "Failed to load schema from {}: {}",
                            path.display(),
                            e
                        ))
                    }
                };
                sv.add_schema_with_import_ref(schema.clone(), Some((schema_id.clone(), uri.clone())))?;
    
            } else {
                return Err(format!("No resolution found for URI: {} imported from {}", uri, schema_source_uri.unwrap_or("unknown".to_owned())));
            }
        }
    }
    Ok(())
}
