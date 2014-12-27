#![feature(phase)]

extern crate valico;
extern crate "rustc-serialize" as serialize;

#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

mod builder;
mod helpers;