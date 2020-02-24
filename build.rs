

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    write!(&mut file, "#[allow(clippy::unreadable_literal)]\n").unwrap();
    write!(&mut file, "static PROPERTY_KEYS: phf::Set<&'static str> = {}",
    phf_codegen::Set::new()
        .entry("properties")
        .entry("patternProperties")
        .build()
    ).unwrap();
    write!(&mut file, ";\n").unwrap();

    write!(&mut file, "#[allow(clippy::unreadable_literal)]\n").unwrap();
    write!(
        &mut file,
        "static NON_SCHEMA_KEYS: phf::Set<&'static str> = {};",
        phf_codegen::Set::new()
            .entry("properties")
            .entry("patternProperties")
            .entry("dependencies")
            .entry("definitions")
            .entry("anyOf")
            .entry("allOf")
            .entry("oneOf")
            .build()
    ).unwrap();

    write!(&mut file, "#[allow(clippy::unreadable_literal)]\n").unwrap();
    write!(
        &mut file,
        "static BOOLEAN_SCHEMA_ARRAY_KEYS: phf::Set<&'static str> = {};",
        phf_codegen::Set::new()
            .entry("allOf")
            .entry("anyOf")
            .entry("items")
            .entry("oneOf")
            .build()
    ).unwrap();

    write!(&mut file, "#[allow(clippy::unreadable_literal)]\n").unwrap();
    write!(&mut file, "static FINAL_KEYS: phf::Set<&'static str> = {};",
    phf_codegen::Set::new()
        .entry("enum")
        .entry("required")
        .entry("type")
        .build()
    ).unwrap();

    write!(&mut file, "#[allow(clippy::unreadable_literal)]\n").unwrap();
    write!(
        &mut file,
        "const ALLOW_NON_CONSUMED_KEYS: phf::Set<&'static str> = {};",
        phf_codegen::Set::new()
            .entry("definitions")
            .entry("$schema")
            .entry("$id")
            .entry("default")
            .entry("title")
            .entry("description")
            .entry("format")
            .entry("examples")
            .build()
    ).unwrap();
}
