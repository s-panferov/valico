#![feature(plugin)]
#![allow(unstable)]

extern crate valico;
extern crate "rustc-serialize" as serialize;

#[plugin]
extern crate regex_macros;
extern crate regex;

mod builder;
mod helpers;