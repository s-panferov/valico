#![plugin(regex_macros)]
#![feature(plugin)]
#![feature(core)]
#![feature(path_ext)]

extern crate valico;
extern crate "rustc-serialize" as serialize;

extern crate regex;

mod dsl;
mod schema;
