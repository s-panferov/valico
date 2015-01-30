use serialize::json;
use url;

use super::super::schema;
use super::super::validators;

#[allow(missing_copy_implementations)]
pub struct Items;
impl super::Keyword for Items {
    fn compile(&self, def: &json::Json, ctx: &schema::WalkContext) -> super::KeywordResult {
        let maybe_items = def.find("items");
        let maybe_additional = def.find("additionalItems");

        if maybe_items.is_some() || maybe_additional.is_some() {
            let items = if maybe_items.is_some() {
                let items_val = maybe_items.unwrap();
                
                Some(if items_val.is_object() {

                    validators::items::ItemsKind::Schema(
                        url_parser!().parse(
                            (ctx.url.to_string() + "#" + ctx.fragment.connect("/").as_slice() + "/items").as_slice()
                        ).unwrap()
                    )

                } else if items_val.is_array() {

                    let schemas = range(0, items_val.as_array().unwrap().len()).map(|idx| {
                        url_parser!().parse(
                            (ctx.url.to_string() + "#" + ctx.fragment.connect("/").as_slice() + "/items/" + idx.to_string().as_slice()).as_slice()
                        ).unwrap()
                    }).collect::<Vec<url::Url>>();
                    validators::items::ItemsKind::Array(schemas)

                } else {

                    return Err(schema::SchemaError::Malformed {
                        path: ctx.fragment.connect("/"),
                        detail: "`items` must be an object or an array".to_string()
                    }) 

                })
            } else { None };

            let additional_items = if maybe_additional.is_some() {
                let additional_val = maybe_additional.unwrap();
                Some(if additional_val.is_boolean() {

                    validators::items::AdditionalKind::Boolean(additional_val.as_boolean().unwrap())

                } else if additional_val.is_object() {

                    validators::items::AdditionalKind::Schema(
                        url_parser!().parse(
                            (ctx.url.to_string() + "#" + ctx.fragment.connect("/").as_slice() + "/additionalItems").as_slice()
                        ).unwrap()
                    )

                } else {

                    return Err(schema::SchemaError::Malformed {
                        path: ctx.fragment.connect("/"),
                        detail: "`additionalItems` must be a boolean or an object".to_string()
                    }) 

                })
            } else { None };

            Ok(Some(Box::new(validators::Items {
                items: items,
                additional: additional_items
            })))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)] use super::super::scope;
#[cfg(test)] use jsonway;
#[cfg(test)] use serialize::json::{ToJson};

#[test]
fn validate_items() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(jsonway::object(|schema| {
        schema.object("items", |items| {
            items.set("maximum", 10);
            items.set("minimum", 5);
        });
    }).unwrap()).ok().unwrap();

    assert_eq!(schema.validate(&[5,6,7,8].to_json()).valid, true);
    assert_eq!(schema.validate(&[4,5,6,7,8].to_json()).valid, false);
}