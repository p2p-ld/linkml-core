use clap::Parser;
use linkml_runtime::{diff, load_json_file, load_yaml_file, Delta};
use linkml_schemaview::identifier::Identifier;
use linkml_schemaview::io::from_yaml;
use linkml_schemaview::resolve::resolve_schemas;
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
    /// Source data file (YAML or JSON)
    source: PathBuf,
    /// Target data file (YAML or JSON)
    target: PathBuf,
    /// Output file for deltas; defaults to stdout
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let schema = from_yaml(&args.schema)?;
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).map_err(|e| format!("{e}"))?;
    resolve_schemas(&mut sv).map_err(|e| format!("{e}"))?;
    let conv = sv.converter();
    let class_view = sv.get_tree_root_or(args.class.as_deref());

    let src = load_value(&args.source, &sv, class_view.as_ref(), &conv)?;
    let tgt = load_value(&args.target, &sv, class_view.as_ref(), &conv)?;
    let deltas = diff(&src, &tgt, false);

    let mut writer: Box<dyn Write> = if let Some(out) = &args.output {
        Box::new(File::create(out)?)
    } else {
        Box::new(std::io::stdout())
    };
    if let Some(ext) = args
        .output
        .as_ref()
        .and_then(|p| p.extension().and_then(|s| s.to_str()))
    {
        if ext == "json" {
            serde_json::to_writer_pretty(&mut writer, &deltas)?;
        } else {
            serde_yaml::to_writer(&mut writer, &deltas)?;
        }
    } else {
        serde_yaml::to_writer(&mut writer, &deltas)?;
    }
    writer.write_all(b"\n")?;
    Ok(())
}
