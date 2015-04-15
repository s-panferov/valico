extern crate valico;
extern crate rustc_serialize as serialize;

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

    let state = params.process(&mut obj, &None);
    if state.is_valid() {
        println!("Result object is {}", obj.pretty().to_string());
    } else {
        panic!("Errors during process: {:?}", state);
    }

}