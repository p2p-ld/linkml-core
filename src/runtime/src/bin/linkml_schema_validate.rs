use clap::Parser;
use linkml_schemaview::{
    io::from_yaml,
    schemaview::SchemaView,
    identifier::{Identifier},
    resolve::resolve_schemas,
};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "linkml-schema-validate")]
struct Args {
    /// LinkML schema YAML file
    schema: PathBuf,
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
                if !class_exists && !ty_exists {
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
        println!("schema valid");
        Ok(())
    } else {
        for e in &errors {
            println!("{e}");
        }
        std::process::exit(1);
    }
}
