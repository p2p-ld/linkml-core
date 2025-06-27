use clap::Parser;
use linkml_runtime::{
    load_json_file, load_yaml_file,
    turtle::{write_turtle, TurtleOptions},
    validate,
};
use linkml_schemaview::io::from_yaml;
use linkml_schemaview::resolve::resolve_schemas;
use linkml_schemaview::schemaview::SchemaView;
use std::fs::File;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// LinkML schema YAML file
    schema: PathBuf,
    /// Data file (YAML or JSON)
    data: PathBuf,
    /// Name of the class to use as the root object
    #[arg(short, long)]
    class: Option<String>,
    /// Output TTL file; defaults to stdout
    #[arg(short, long)]
    output: Option<PathBuf>,
    /// Use skolem IRIs instead of blank nodes
    #[arg(long)]
    skolem: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let schema = from_yaml(&args.schema)?;
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).map_err(|e| format!("{e}"))?;
    resolve_schemas(&mut sv).map_err(|e| format!("{e}"))?;
    let conv = sv.converter();
    let class_view = sv.get_tree_root_or(args.class.as_deref());
    let data_path = &args.data;
    let value = if let Some(ext) = data_path.extension() {
        if ext == "json" {
            load_json_file(data_path, &sv, class_view.as_ref(), &conv)?
        } else {
            load_yaml_file(data_path, &sv, class_view.as_ref(), &conv)?
        }
    } else {
        load_yaml_file(data_path, &sv, class_view.as_ref(), &conv)?
    };
    if let Err(e) = validate(&value) {
        eprintln!("invalid: {e}");
        std::process::exit(1);
    }
    let mut writer: Box<dyn std::io::Write> = if let Some(out) = &args.output {
        Box::new(File::create(out)?)
    } else {
        Box::new(std::io::stdout())
    };
    write_turtle(
        &value,
        &sv,
        &schema,
        &conv,
        &mut writer,
        TurtleOptions {
            skolem: args.skolem,
        },
    )?;
    Ok(())
}
