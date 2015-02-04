#![feature(plugin)]
#![feature(io)]
#![feature(path)]
#![feature(core)]

extern crate valico;
extern crate "rustc-serialize" as serialize;

#[plugin]
extern crate regex_macros;
extern crate regex;

mod dsl;
mod schema;
