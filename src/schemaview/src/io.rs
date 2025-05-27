
use linkml_meta::SchemaDefinition;
use linkml_meta::poly::SchemaDefinition as _;

use serde_yml;
use std::fs;
use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{PathBuf, absolute};

pub fn from_yaml(path: &Path) -> Result<SchemaDefinition, Box<dyn Error>> {

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let schema = serde_yml::Deserializer::from_reader(reader);
    let result: Result<SchemaDefinition, _> = serde_path_to_error::deserialize(schema);

    Ok(result?)
}

#[cfg(test)]
mod tests {
    use linkml_meta::poly_containers::MapRef;

    use super::*;
    #[test]
    fn load_schema(){
        let mut this_file = PathBuf::from("src/io.rs");

        println!("{:?}", this_file);
        this_file = absolute(this_file.as_path()).unwrap();

        println!("{:?}", this_file);
        this_file.pop();
        this_file.pop();
        this_file.push("tests");
        this_file.push("data");
        this_file.push("meta.yaml");
        let path = this_file.as_path();

        println!("{:?}", path);
        let schema = match from_yaml(path) {
            Ok(v) => v,
            Err(why) => panic!("{why:?}"),
        };
        
        let classes = schema.classes();
        for (name, class) in classes.iter() {
            let is_a = &class.is_a;
            println!("class: {name} is a subclass of {is_a:?}")
        }

        //println!("{:?}", schema)
    }
}