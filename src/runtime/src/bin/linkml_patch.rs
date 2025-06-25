use clap::Parser;
use linkml_runtime::{load_json_file, load_yaml_file, patch, Delta};
use linkml_schemaview::identifier::{converter_from_schema, Identifier};
use linkml_schemaview::io::from_yaml;
use linkml_schemaview::schemaview::{ClassView, SchemaView};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "linkml-patch")]
struct Args {
    /// LinkML schema YAML file
    schema: PathBuf,
    /// Name of the root class
    #[arg(short, long)]
    class: Option<String>,
    /// Source data file (YAML or JSON)
    source: PathBuf,
    /// Delta file (YAML or JSON)
    delta: PathBuf,
    /// Output patched file; defaults to stdout
    #[arg(short, long)]
    output: Option<PathBuf>,
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
    path: Option<&Path>,
    value: &linkml_runtime::LinkMLValue,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = value.to_json();
    let mut writer: Box<dyn Write> = if let Some(p) = path {
        Box::new(File::create(p)?)
    } else {
        Box::new(std::io::stdout())
    };
    if let Some(ext) = path.and_then(|p| p.extension().and_then(|s| s.to_str())) {
        if ext == "json" {
            serde_json::to_writer_pretty(&mut writer, &json)?;
        } else {
            serde_yaml::to_writer(&mut writer, &json)?;
        }
    } else {
        serde_yaml::to_writer(&mut writer, &json)?;
    }
    writer.write_all(b"\n")?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let schema = from_yaml(&args.schema)?;
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).map_err(|e| format!("{e}"))?;
    let conv = converter_from_schema(&schema);
    let class_view = choose_class(&sv, &schema, &args.class, &conv)?;

    let src = load_value(&args.source, &sv, class_view.as_ref(), &conv)?;
    let delta_text = std::fs::read_to_string(&args.delta)?;
    let deltas: Vec<Delta> = if let Some(ext) = args.delta.extension().and_then(|s| s.to_str()) {
        if ext == "json" {
            serde_json::from_str(&delta_text)?
        } else {
            serde_yaml::from_str(&delta_text)?
        }
    } else {
        serde_yaml::from_str(&delta_text)?
    };
    let patched = patch(&src, &deltas, &sv);
    write_value(args.output.as_deref(), &patched)?;
    Ok(())
}
