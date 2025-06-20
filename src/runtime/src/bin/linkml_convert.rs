use clap::Parser;
use linkml_runtime::{
    load_json_file, load_yaml_file,
    turtle::{write_turtle, TurtleOptions},
    validate,
};
use schemaview::identifier::converter_from_schema;
use schemaview::identifier::Identifier;
use schemaview::io::from_yaml;
use schemaview::schemaview::SchemaView;
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
    let class_view = if let Some(cls_name) = &args.class {
        Some(
            sv.get_class(&Identifier::new(cls_name), &conv)
                .map_err(|e| format!("{e:?}"))?
                .ok_or("class not found")?,
        )
    } else {
        None
    };
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
