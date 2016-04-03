use ardite::{Error, Service};
use ardite::schema::{Definition};
use ardite::value::{Value, Object};
use inflections::Inflect;
use iron::Url;

use resource::{Resource, Data};
use resource::collection::Collection;

pub struct Root;

impl Resource for Root {
  fn route<'a>(&self, definition: &'a Definition, next: String) -> Option<Box<Resource + 'a>> {
    let name = next.to_snake_case();
    if definition.get_collection(&name).is_some() {
      Some(Box::new(Collection::new(name)))
    } else {
      None
    }
  }

  /// Gets an object containing all of the top level URLs exposed by this API.
  /// Such top level URLs contain:
  ///
  /// - URLs for type collections.
  fn get(&self, create_url: &Fn(Vec<String>) -> Url, service: &Service) -> Result<Data, Error> {
    let mut object = Object::new();

    for (name, _) in service.collections() {
      let mut key = name.clone();
      key.push_str("_collection_url");
      let value = value!(format!("{}", create_url(vec![name.clone()])));
      object.insert(key, value);
    }

    Ok(Value::Object(object).into())
  }
}
