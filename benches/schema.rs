#![feature(io)]
#![feature(path)]
#![feature(test)]

extern crate valico;
extern crate "rustc-serialize" as serialize;
extern crate test;

use std::old_io::fs;
use serialize::json;
use valico::json_schema;

fn read_schema() -> json::Json {
    fs::File::open(&Path::new("tests/schema/schema.json")).ok().unwrap()
        .read_to_string().ok().unwrap().parse().unwrap()
}

#[bench]
fn bench_compilation(b: &mut test::Bencher) {
    let schema = read_schema();

    b.iter(|| {
        let mut scope = json_schema::Scope::new();
        scope.compile(schema.clone(), false).ok().unwrap();
    });
}

#[bench]
fn bench_compilation_ban(b: &mut test::Bencher) {
    let schema = read_schema();

    b.iter(|| {
        let mut scope = json_schema::Scope::new();
        scope.compile(schema.clone(), true).ok().unwrap();
    });
}

#[bench]
fn bench_validation(b: &mut test::Bencher) {
    let schema = read_schema();
    let mut scope = json_schema::Scope::new();
    let compiled_schema = scope.compile_and_return(schema.clone(), true).ok().unwrap();

    b.iter(|| assert!(compiled_schema.validate(&schema).is_valid()));
}