#![plugin(regex_macros)]
#![feature(plugin)]
#![feature(core)]
#![feature(fs)]
#![feature(path)]
#![feature(io)]
#![feature(os)]

extern crate valico;
extern crate "rustc-serialize" as serialize;

extern crate regex;

mod dsl;
mod schema;
