// Filename: cargo_toml_parser.rs

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use serde::Deserialize;
use toml::de::Error as TomlError;

#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Package,
    dependencies: HashMap<String, Dependency>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Dependency {
    Simple(String),
    Detailed(DetailedDependency),
}

#[derive(Debug, Deserialize)]
struct DetailedDependency {
    version: String,
    #[serde(default)]
    features: Vec<String>,
    #[serde(rename = "optional")]
    is_optional: Option<bool>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cargo_toml_path = "Cargo.toml";
    let cargo_toml = read_cargo_toml(cargo_toml_path)?;

    println!("Package Information:\n{:#?}", cargo_toml.package);
    println!("Dependencies:\n{:#?}", cargo_toml.dependencies);

    Ok(())
}

fn read_cargo_toml(path: &str) -> Result<CargoToml, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let cargo_toml: CargoToml = toml::from_str(&contents)?;
    Ok(cargo_toml)
}