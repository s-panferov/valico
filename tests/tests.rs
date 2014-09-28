#![feature(phase)]

extern crate valico;
extern crate serialize;

#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

mod builder;
mod helpers;