#![allow(unstable)]

extern crate valico;
extern crate "rustc-serialize" as serialize;

use serialize::json::{Json};
use valico::json_dsl;

fn main() {

    let params = json_dsl::Builder::build(|params| {
        params.req_nested("user", json_dsl::array(), |params| {
            params.req_typed("name", json_dsl::string());
            params.req_typed("friend_ids", json_dsl::array_of(json_dsl::u64()))
        });
    });

    let mut obj = r#"{"user": {"name": "Frodo", "friend_ids": ["1223"]}}"#.parse::<Json>().unwrap();

    match params.process(&mut obj) {
        Ok(()) => {
            println!("Result object is {}", obj.pretty().to_string());
        },
        Err(err) => {
            panic!("Errors during process: {:?}", err);
        }
    }

}