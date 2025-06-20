use clap::Parser;
use linkml_runtime::{load_json_file, load_yaml_file, validate};
use schemaview::identifier::{converter_from_schema, Identifier};
use schemaview::io::from_yaml;
use schemaview::schemaview::SchemaView;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// LinkML schema YAML file
    schema: PathBuf,
    /// Name of the class to validate against
    class: String,
    /// Data file (YAML or JSON)
    data: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let schema = from_yaml(&args.schema)?;
    let mut sv = SchemaView::new();
    sv.add_schema(schema.clone()).map_err(|e| format!("{e}"))?;
    let conv = converter_from_schema(&schema);
    let class_view = sv
        .get_class(&Identifier::new(&args.class), &conv)
        .map_err(|e| format!("{e:?}"))?
        .ok_or("class not found")?;
    let data_path = &args.data;
    let value = if let Some(ext) = data_path.extension() {
        if ext == "json" {
            load_json_file(data_path, &sv, Some(&class_view), &conv)?
        } else {
            load_yaml_file(data_path, &sv, Some(&class_view), &conv)?
        }
    } else {
        load_yaml_file(data_path, &sv, Some(&class_view), &conv)?
    };
    match validate(&value) {
        Ok(_) => {
            println!("valid");
            Ok(())
        }
        Err(e) => {
            println!("invalid: {}", e);
            std::process::exit(1);
        }
    }
}
