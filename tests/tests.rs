#![plugin(regex_macros)]
#![feature(plugin)]
#![feature(io)]
#![feature(path)]
#![feature(core)]

extern crate valico;
extern crate "rustc-serialize" as serialize;

extern crate regex;

mod dsl;
mod schema;
