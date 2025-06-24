use std::{collections::HashMap, env, error::Error, fs::File, io::Write, path::Path};
use toml::{map::Map, Table, Value};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo::rerun-if-changed=crates/codegen/data/deprecated.toml");
    println!("cargo::rerun-if-changed=build.rs");
    let mut metadata: Table =
        toml::from_str(include_str!("./crates/codegen/data/deprecated.toml"))?;
    // Add version number
    let pkg_table = metadata.get_mut("package").unwrap().as_table_mut().unwrap();
    let version = env!("CARGO_PKG_VERSION");
    pkg_table.insert("version".into(), Value::String(version.into()));
    // Add deprecated objects
    let mut unique_ids: Vec<Value> = include_str!("./crates/codegen/data/uniques.txt")
        .split('\n')
        .filter(|id| !id.is_empty())
        .map(|id| Value::String(id.into()))
        .collect();
    let broken: HashMap<String, String> =
        serde_json::from_str(include_str!("./crates/codegen/data/broken.json"))?;
    for (key, _) in broken {
        if !key.starts_with("T_") {
            unique_ids.push(Value::String(key));
        }
    }
    let mut tools = Map::new();
    let mut csse = Map::new();
    csse.insert("deprecated".into(), Value::Array(unique_ids));
    tools.insert("csse".into(), Value::Table(csse));
    metadata.insert("tools".into(), Value::Table(tools));
    // Write metadata
    let mut file = File::create(Path::new("./Morrowind-metadata.toml"))?;
    file.write_all(metadata.to_string().as_bytes())?;
    Ok(())
}
