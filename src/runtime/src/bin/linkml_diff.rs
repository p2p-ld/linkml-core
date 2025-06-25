use clap::{Parser, Subcommand};
use linkml_runtime::{diff, load_json_file, load_yaml_file, patch, Delta};
use linkml_schemaview::identifier::{converter_from_schema, Identifier};
use linkml_schemaview::io::from_yaml;
use linkml_schemaview::schemaview::{ClassView, SchemaView};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "linkml-diff")]
struct Args {
    /// LinkML schema YAML file
    schema: PathBuf,
    /// Name of the root class
    #[arg(short, long)]
    class: Option<String>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Compute the diff between two data files
    Diff {
        /// Source data file (YAML or JSON)
        source: PathBuf,
        /// Target data file (YAML or JSON)
        target: PathBuf,
        /// Output file for deltas
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Apply a patch (delta file) to a source file
    Patch {
        /// Source data file (YAML or JSON)
        source: PathBuf,
        /// Delta file (YAML or JSON)
        delta: PathBuf,
        /// Output patched file
        #[arg(short, long)]
        output: PathBuf,
    },
}

fn load_value<'a>(
    path: &Path,
    sv: &'a SchemaView,
    class: Option<&'a ClassView<'a>>,
    conv: &curies::Converter,
) -> Result<linkml_runtime::LinkMLValue<'a>, Box<dyn std::error::Error>> {
    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
        if ext == "json" {
            load_json_file(path, sv, class, conv)
        } else {
            load_yaml_file(path, sv, class, conv)
        }
    } else {
        load_yaml_file(path, sv, class, conv)
    }
}

fn choose_class<'a>(
    sv: &'a SchemaView,
    schema: &'a linkml_meta::SchemaDefinition,
    class_name: &Option<String>,
    conv: &curies::Converter,
) -> Result<Option<ClassView<'a>>, Box<dyn std::error::Error>> {
    if let Some(n) = class_name {
        Ok(Some(
            sv.get_class(&Identifier::new(n), conv)
                .map_err(|e| format!("{:?}", e))?
                .ok_or("class not found")?,
        ))
    } else {
        for (name, def) in &schema.classes {
            if def.tree_root.unwrap_or(false) {
                return Ok(Some(
                    sv.get_class(&Identifier::new(name), conv)
                        .map_err(|e| format!("{:?}", e))?
                        .ok_or("class not found")?,
                ));
            }
        }
        Ok(None)
    }
}

fn write_value(
    path: &Path,
    value: &linkml_runtime::LinkMLValue,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = value.to_json();
    let mut file = File::create(path)?;
    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
        if ext == "json" {
            serde_json::to_writer_pretty(&mut file, &json)?;
        } else {
            serde_yaml::to_writer(&mut file, &json)?;
        }
    } else {
        serde_yaml::to_writer(&mut file, &json)?;
    }
    file.write_all(b"\n")?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let schema = from_yaml(&args.schema)?;
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).map_err(|e| format!("{e}"))?;
    let conv = converter_from_schema(&schema);
    let class_view = choose_class(&sv, &schema, &args.class, &conv)?;

    match args.command {
        Command::Diff {
            source,
            target,
            output,
        } => {
            let src = load_value(&source, &sv, class_view.as_ref(), &conv)?;
            let tgt = load_value(&target, &sv, class_view.as_ref(), &conv)?;
            let deltas = diff(&src, &tgt, false);
            let mut file = File::create(&output)?;
            if let Some(ext) = output.extension().and_then(|s| s.to_str()) {
                if ext == "json" {
                    serde_json::to_writer_pretty(&mut file, &deltas)?;
                } else {
                    serde_yaml::to_writer(&mut file, &deltas)?;
                }
            } else {
                serde_yaml::to_writer(&mut file, &deltas)?;
            }
            file.write_all(b"\n")?;
        }
        Command::Patch {
            source,
            delta,
            output,
        } => {
            let src = load_value(&source, &sv, class_view.as_ref(), &conv)?;
            let delta_text = std::fs::read_to_string(&delta)?;
            let deltas: Vec<Delta> = if let Some(ext) = delta.extension().and_then(|s| s.to_str()) {
                if ext == "json" {
                    serde_json::from_str(&delta_text)?
                } else {
                    serde_yaml::from_str(&delta_text)?
                }
            } else {
                serde_yaml::from_str(&delta_text)?
            };
            let patched = patch(&src, &deltas, &sv);
            write_value(&output, &patched)?;
        }
    }
    Ok(())
}
