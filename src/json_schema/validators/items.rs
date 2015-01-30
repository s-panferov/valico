use serialize::json;
use url;

use super::super::errors;
use super::super::scope;

#[derive(Debug)]
pub enum ItemsKind {
    Schema(url::Url),
    Array(Vec<url::Url>)
}

#[derive(Debug)]
pub enum AdditionalKind {
    Boolean(bool),
    Schema(url::Url)
}

#[allow(missing_copy_implementations)]
pub struct Items {
    pub items: Option<ItemsKind>,
    pub additional: Option<AdditionalKind>
}

impl super::Validator for Items {
    fn validate(&self, val: &json::Json, path: &str, strict: bool, scope: &scope::Scope) -> super::ValidatorResult {
        let array = strict_process!(val.as_array(), path, strict, "The value must be an array");

        match self.items {
            Some(ItemsKind::Schema(ref url)) => {
                let schema = scope.resolve(url);
                println!("{:?}", url);
                panic!("{:?}", schema)
            },
            _ => ()
        }

        Ok(())
    }
}