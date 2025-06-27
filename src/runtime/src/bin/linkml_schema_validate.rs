use clap::{Parser, ValueEnum};
use linkml_schemaview::{
    identifier::Identifier, io::from_yaml, resolve::resolve_schemas, schemaview::SchemaView,
};
use serde_json;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "linkml-schema-validate")]
struct Args {
    /// LinkML schema YAML file
    schema: PathBuf,
    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    output: OutputFormat,
}

#[derive(ValueEnum, Clone)]
enum OutputFormat {
    Text,
    Json,
}

fn type_exists(
    sv: &SchemaView,
    id: &Identifier,
    conv: &curies::Converter,
) -> Result<bool, linkml_schemaview::identifier::IdentifierError> {
    use linkml_schemaview::identifier::Identifier as Id;
    match id {
        Id::Name(n) => {
            for (_, schema) in sv.iter_schemas() {
                if schema.types.contains_key(n) {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        Id::Curie(_) | Id::Uri(_) => {
            let target_uri = id.to_uri(conv)?;
            for (_, schema) in sv.iter_schemas() {
                for t in schema.types.values() {
                    if let Some(turi) = &t.type_uri {
                        if Identifier::new(turi).to_uri(conv)?.0 == target_uri.0 {
                            return Ok(true);
                        }
                    }
                }
            }
            Ok(false)
        }
    }
}

fn enum_exists(
    sv: &SchemaView,
    id: &Identifier,
    conv: &curies::Converter,
) -> Result<bool, linkml_schemaview::identifier::IdentifierError> {
    use linkml_schemaview::identifier::Identifier as Id;
    match id {
        Id::Name(n) => {
            for (_, schema) in sv.iter_schemas() {
                if schema.enums.contains_key(n) {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        Id::Curie(_) | Id::Uri(_) => {
            let target_uri = id.to_uri(conv)?;
            for (_, schema) in sv.iter_schemas() {
                for (name, e) in &schema.enums {
                    if let Some(euri) = &e.enum_uri {
                        if Identifier::new(euri).to_uri(conv)?.0 == target_uri.0 {
                            return Ok(true);
                        }
                    } else {
                        let default_prefix =
                            schema.default_prefix.as_deref().unwrap_or(&schema.name);
                        let default_uri = Identifier::new(&format!("{}:{}", default_prefix, name))
                            .to_uri(conv)?
                            .0;
                        if default_uri == target_uri.0 {
                            return Ok(true);
                        }
                    }
                }
            }
            Ok(false)
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let schema = from_yaml(&args.schema)?;
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).map_err(|e| format!("{e}"))?;
    if let Err(e) = resolve_schemas(&mut sv) {
        eprintln!("{e}");
    }
    let conv = sv.converter();

    let mut errors = Vec::new();
    for uri in sv.get_unresolved_schemas() {
        errors.push(format!("Unresolved import: {}", uri));
    }

    for (schema_uri, schema_def) in sv.iter_schemas() {
        for (slot_name, slot_def) in &schema_def.slot_definitions {
            if let Some(range) = &slot_def.range {
                let id = Identifier::new(range);
                let class_exists = sv
                    .get_class(&id, &conv)
                    .map_err(|e| format!("{e:?}"))?
                    .is_some();
                let ty_exists = type_exists(&sv, &id, &conv).map_err(|e| format!("{e:?}"))?;
                let en_exists = enum_exists(&sv, &id, &conv).map_err(|e| format!("{e:?}"))?;
                if !class_exists && !ty_exists && !en_exists {
                    errors.push(format!(
                        "Unknown range `{}` for slot `{}` in schema `{}`",
                        range, slot_name, schema_uri
                    ));
                }
            }
        }

        for (class_name, class_def) in &schema_def.classes {
            if let Some(parent) = &class_def.is_a {
                let id = Identifier::new(parent);
                if sv
                    .get_class(&id, &conv)
                    .map_err(|e| format!("{e:?}"))?
                    .is_none()
                {
                    errors.push(format!(
                        "Unknown parent class `{}` referenced by class `{}` in schema `{}`",
                        parent, class_name, schema_uri
                    ));
                }
            }
            for slot in &class_def.slots {
                let id = Identifier::new(slot);
                if sv
                    .get_slot(&id, &conv)
                    .map_err(|e| format!("{e:?}"))?
                    .is_none()
                {
                    errors.push(format!(
                        "Unknown slot `{}` used in class `{}` in schema `{}`",
                        slot, class_name, schema_uri
                    ));
                }
            }
        }
    }

    if errors.is_empty() {
        match args.output {
            OutputFormat::Text => println!("schema valid"),
            OutputFormat::Json => println!("{}", serde_json::json!({"status":"valid"})),
        }
        Ok(())
    } else {
        match args.output {
            OutputFormat::Text => {
                for e in &errors {
                    println!("{e}");
                }
            }
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&errors)?);
            }
        }
        std::process::exit(1);
    }
}
