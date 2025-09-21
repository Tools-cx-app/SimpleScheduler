use std::{fs, io::Write};

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct Package {
    pub authors: Vec<String>,
    pub name: String,
    pub version: String,
    pub description: String,
}

#[derive(Deserialize)]
struct CargoConfig {
    pub package: Package,
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=update");

    let toml = fs::read_to_string("Cargo.toml")?;
    let data: CargoConfig = toml::from_str(&toml)?;

    gen_module_prop(&data)?;

    Ok(())
}

fn gen_module_prop(data: &CargoConfig) -> Result<()> {
    let package = &data.package;
    let id = package.name.replace('-', "_");
    let version_code: usize = package.version.replace('.', "").trim().parse()?;
    let authors = &package.authors;
    let mut author = String::new();
    let mut conut = 0;
    for a in authors {
        conut += 1;
        if conut > 1 {
            author += &format!("& {a}");
        } else {
            author += &format!("{a} ");
        }
    }
    let author = author.trim();

    let mut file = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("module/module.prop")?;

    writeln!(file, "id={id}")?;
    writeln!(file, "name={}", package.name)?;
    writeln!(file, "version=v{}", package.version)?;
    writeln!(file, "versionCode={version_code}")?;
    writeln!(file, "author={author}")?;
    writeln!(file, "description={}", package.description)?;

    Ok(())
}
