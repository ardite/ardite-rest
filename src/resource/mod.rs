pub mod root;

use ardite::Service;
use ardite::error::{Error, MethodNotAllowed};
use ardite::schema::Definition;
use ardite::value::Value;
use iron::Url;

/// Abstracts routes away into resources. This trait also doesn’t handle
/// `Request`/`Response` objects, rather it only handles an abstracted
/// interface. This helps with compliance to the Uniform Interface constraint.
pub trait Resource {
  fn route(&self, _: &Definition, _: String) -> Option<Box<Resource>> {
    None
  }

  fn get(&self, _: &Fn(Vec<String>) -> Url, _: &Service) -> Result<Value, Error> {
    Err(Error::new(MethodNotAllowed, "Cannot perform a GET request to this resource."))
  }

  fn post(&self, _: &Fn(Vec<String>) -> Url, _: &Service) -> Result<Value, Error> {
    Err(Error::new(MethodNotAllowed, "Cannot perform a POST request to this resource."))
  }

  fn put(&self, _: &Fn(Vec<String>) -> Url, _: &Service) -> Result<Value, Error> {
    Err(Error::new(MethodNotAllowed, "Cannot perform a PUT request to this resource."))
  }

  fn patch(&self, _: &Fn(Vec<String>) -> Url, _: &Service) -> Result<Value, Error> {
    Err(Error::new(MethodNotAllowed, "Cannot perform a PATCH request to this resource."))
  }

  fn delete(&self, _: &Fn(Vec<String>) -> Url, _: &Service) -> Result<Value, Error> {
    Err(Error::new(MethodNotAllowed, "Cannot perform a DELETE request to this resource."))
  }
}
