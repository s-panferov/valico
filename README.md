# What is Valico?

Valico is a validation and coercion tool for JSON objects, written in Rust and inspired by [Grape]. It designed to be a support library for the various REST-like frameworks or other tools that need to validate and coerce JSON input from outside world.

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

# Validation and coercion

You can define validations and coercion options for your parameters using a `Builder::build` block. Parameters can be **optional** and **required**. Requires parameters must be always present. Optional parameters can be omitted.

When parameter is present in JSON all validation and coercions will be applied and error fired if something goes wrong.

## Builder

This functions are available in Builder to define parameters:

~~~rust

// Parameter is required, no coercion
fn req_defined(&mut self, name: &str);

// Parameter is required, with coercion
fn req_typed(&mut self, name: &str, coercer: Box<Coercer>);

// Parameter is required, with coercion and nested checks
fn req_nested(&mut self, name: &str, coercer: Box<Coercer>, nest_def: |&mut Builder|);

// Parameter is required, setup with Param DSL
fn req(&mut self, name: &str, param_builder: |&mut Param|);

// Parameter is optional, no coercion
fn opt_defined(&mut self, name: &str);

// Parameter is optional, with coercion
fn opt_typed(&mut self, name: &str, coercer: Box<Coercer>);

// Parameter is optional, with coercion and nested checks
fn opt_nested(&mut self, name: &str, coercer: Box<Coercer>, nest_def: |&mut Builder|);

// Parameter is required, setup with Param DSL
fn opt(&mut self, name: &str, param_builder: |&mut Param|);

~~~

## Built-in Coercers

Available list of coercers:
    
* Builder::i64() 
* Builder::u64() 
* Builder::f64() 
* Builder::string() 
* Builder::boolean() 
* Builder::null() 
* Builder::list() 
* Builder::list_of() 
* Builder::object() 

Example of usage:

~~~rust
let params = Builder::build(|params| {
    params.req_typed("id", Builder::u64());
    params.req_typed("name", Builder::string());
    params.opt_typed("is_active", Builder::boolean());
    params.opt_typed("tags", Builder::list_of(Builder::strings()));
});
~~~

## Nested processing

You can specify rules to nesting processing for **lists** and **objects**:

~~~rust
let params = Builder::build(|params| {
    params.req_nested("user", Builder::object(), |params| {
        params.req_typed("name", Builder::string());
        params.opt_typed("is_active", Builder::boolean());
        params.opt_typed("tags", Builder::list_of(Builder::strings()));
    });
});

let params = Builder::build(|params| {
    params.req_nested("users", Builder::list(), |params| {
        params.req_typed("name", Builder::string());
        params.opt_typed("is_active", Builder::boolean());
        params.opt_typed("tags", Builder::list_of(Builder::strings()));
    });
});
~~~

Nesting level is not limited in Valico.

## Parameters DSL

You can use DSL block to setup parameters with more flexible way:

~~~rust
let params = Builder::build(|params| {
    params.req("user", |user| {
        user.desc("Parameter is used to create new user");
        user.coerce(Builder::object());

        // this allows null to be a valid value
        user.allow_null();
        
        user.nest(|params| {
            params.req_typed("name", Builder::string());
            params.opt("kind", |kind| {
                kind.coerce(Builder::string());

                // optional values can have default values
                kind.default("simeple_user".to_string())
            });
        });
    });
});
~~~

## Parameter validations

Parameter validations available only in DSL block.

### allow_values

Parameters can be restricted to a specific set of values with **allow_values**:

~~~rust
let params = Builder::build(|params| {
    params.req("kind", |kind| {
        kind.coerce(Builder::string());
        kind.allow_values(["circle".to_string(), "square".to_string()]);
    })
})
~~~

### reject_values

Some values can be rejected with **reject_values**:

~~~rust
let params = Builder::build(|params| {
    params.req("user_role", |kind| {
        kind.coerce(Builder::string());
        kind.reject_values(["admin".to_string(), "manager".to_string()]);
    })
})
~~~

### regex

String values can be tested with Regex:

~~~rust
let params = Builder::build(|params| {
    params.req("nickname", |a| {
        a.coerce(Builder::string());

        // force all nicknames to start with "Amazing"
        a.regex(regex!("^Amazing"));
    })
});
~~~

### validate_with

Sometimes it's usefull to use some custom function as validator:

~~~rust
let params = Builder::build(|params| {
    params.req("pushkin_birthday", |a| {
        a.coerce(Builder::u64());

        fn guess(val: &Json) -> Result<(), String> {
            if *val == 1799u.to_json() {
                Ok(())
            } else {
                Err("No!".to_string())
            }
        }

        a.validate_with(guess);
    });
});
~~~

### validate

One can use custom validator. Docs in Progress.

