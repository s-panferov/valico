#![allow(unstable)]

extern crate valico;
extern crate "rustc-serialize" as serialize;

use serialize::json::{Json, ToJson, as_pretty_json};
use valico::{Builder, MutableJson};

fn main() {

    let params = Builder::build(|params| {
        params.req_nested("user", valico::array(), |params| {
            params.req_typed("name", valico::string());
            params.req_typed("friend_ids", valico::array_of(valico::u64()))
        });
    });

    let mut obj = r#"{"user": {"name": "Frodo", "friend_ids": ["1223"]}}"#.parse::<Json>().unwrap();

    match params.process(obj.as_object_mut().unwrap()) {
        Ok(()) => {
            println!("Result object is {}", as_pretty_json(&obj).to_string());
        },
        Err(err) => {
            panic!("Error during process: {}", as_pretty_json(&err.to_json()).to_string());
        }
    }

}