use ardite::{Error, Service};
use ardite::schema::{Definition};
use ardite::value::{Value, Object};
use inflections::Inflect;
use iron::Url;

use resource::Resource;
use resource::type_::Type;

pub struct Root;

impl Resource for Root {
  fn route<'a>(&self, definition: &'a Definition, next: String) -> Option<Box<Resource + 'a>> {
    if let Some(ref type_) = definition.get_type(&next.to_snake_case()) {
      Some(Box::new(Type::new(type_)))
    } else {
      None
    }
  }

  /// Gets an object containing all of the top level URLs exposed by this API.
  /// Such top level URLs contain:
  ///
  /// - URLs for type collections.
  fn get(&self, create_url: &Fn(Vec<String>) -> Url, service: &Service) -> Result<Value, Error> {
    let mut object = Object::new();

    for (name, _) in service.types() {
      let mut key = name.clone();
      key.push_str("_collection_url");
      let value = value!(format!("{}", create_url(vec![name.clone()])));
      object.insert(key, value);
    }

    Ok(Value::Object(object))
  }
}
