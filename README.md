# What is Valico?

Valico is a validation and coersion tool for JSON objects, written in Rust and inspired by [Grape]. It designed to be a support library for the various REST-like frameworks or other tools that need to validate and coerce JSON input from outside world.

[Grape]: https://github.com/intridea/grape

It has built-in support for common coercers, validators and can return detailed error messages if something goes wrong.

# Basic Usage

All Valico stuff is making by Builder instance. Below is a simple example showing how one can create and setup Builder: 

~~~rust
let params = Builder::build(|params| {
	params.req_nested("user", Builder::list(), |params| {
		params.req_typed("name", Builder::string());
		params.req_typed("friend_ids", Builder::list_of(Builder::u64()))
	});
});
~~~

Later `params` instance can be used to process one or more JSON objects with it's `process` method with signature `fn process(&self, tree: &mut JsonObject) -> ValicoResult<()>`

Example: 

~~~rust

extern crate valico;
extern crate serialize;

use serialize::json;
use serialize::json::{ToJson};
use valico::{Builder, MutableJson};

fn main() {

    let params = Builder::build(|params| {
        params.req_nested("user", Builder::list(), |params| {
            params.req_typed("name", Builder::string());
            params.req_typed("friend_ids", Builder::list_of(Builder::u64()))
        });
    });

    let mut obj = json::from_str(
        r#"{"user": {"name": "Frodo", "friend_ids": ["1223"]}}"#
    ).unwrap();

    match params.process(obj.as_object_mut().unwrap()) {
        Ok(()) => {
            println!("Result object is {}", obj.to_pretty_str());
        },
        Err(err) => {
            fail!("Error during process: {}", err.to_json().to_pretty_str());
        }
    }

}
~~~

