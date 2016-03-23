use ardite::{Error, Service};
use ardite::value::{Value, Object};
use iron::Url;

use resource::Resource;

pub struct Root;

impl Resource for Root {
  /// Gets an object containing all of the top level URLs exposed by this API.
  /// Such top level URLs contain:
  ///
  /// - URLs for type collections.
  fn get(&self, create_url: &Fn(Vec<String>) -> Url, service: &Service) -> Result<Value, Error> {
    let mut object = Object::new();

    for (name, _) in service.types() {
      let mut key = name.clone();
      key.push_str("_url");
      let value = value!(format!("{}", create_url(vec![name.clone()])));
      object.insert(key, value);
    }

    Ok(Value::Object(object))
  }
}
