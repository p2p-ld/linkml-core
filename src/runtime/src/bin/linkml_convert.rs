use clap::Parser;
use linkml_runtime::{
    load_json_file, load_yaml_file,
    turtle::{write_turtle, TurtleOptions},
};
use linkml_schemaview::identifier::converter_from_schema;
use linkml_schemaview::io::from_yaml;
use linkml_schemaview::schemaview::SchemaView;
use std::fs::File;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// LinkML schema YAML file
    schema: PathBuf,
    /// Data file (YAML or JSON)
    data: PathBuf,
    /// Output TTL file
    #[arg(short, long)]
    output: PathBuf,
    /// Use skolem IRIs instead of blank nodes
    #[arg(long)]
    skolem: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let schema = from_yaml(&args.schema)?;
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).map_err(|e| format!("{e}"))?;
    let conv = converter_from_schema(&schema);
    let data_path = &args.data;
    let value = if let Some(ext) = data_path.extension() {
        if ext == "json" {
            load_json_file(data_path, &sv, None, &conv)?
        } else {
            load_yaml_file(data_path, &sv, None, &conv)?
        }
    } else {
        load_yaml_file(data_path, &sv, None, &conv)?
    };
    let mut f = File::create(&args.output)?;
    write_turtle(
        &value,
        &sv,
        &schema,
        &conv,
        &mut f,
        TurtleOptions {
            skolem: args.skolem,
        },
    )?;
    Ok(())
}
