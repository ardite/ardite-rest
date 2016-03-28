extern crate hyper;
extern crate serde_json;

#[macro_use]
mod support;

use std::io::Read;

use serde_json::{Value, from_str};

#[test]
fn test_provides_urls() {
  let server = serve!("tests/fixtures/forum.yml");
  let mut string = String::new();
  server.get("/").send().unwrap().read_to_string(&mut string).unwrap();
  let value = from_str::<Value>(&string).unwrap();
  assert!(value.is_object());
  assert_eq!(value.find("person-collection-url").unwrap(), &Value::String(server.url("/person").serialize()));
  assert_eq!(value.find("post-collection-url").unwrap(), &Value::String(server.url("/post").serialize()));
}
