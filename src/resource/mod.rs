pub mod root;
pub mod collection;

use ardite::Service;
use ardite::error::{Error, MethodNotAllowed};
use ardite::schema::Definition;
use ardite::value::{Value, Iter};
use iron::Url;
use urlencoded::QueryMap;

/// Abstracts routes away into resources. This trait also doesn’t handle
/// `Request`/`Response` objects, rather it only handles an abstracted
/// interface. This helps with compliance to the Uniform Interface constraint.
pub trait Resource {
  // TODO: doc
  fn route<'a>(&self, _: &'a Definition, _: String) -> Option<Box<Resource + 'a>> {
    None
  }

  /// Modify self using the query.
  fn query(&mut self, _: &QueryMap) {}

  // TODO: doc
  fn get(&self, _: &Fn(Vec<String>) -> Url, _: &Service) -> Result<Data, Error> {
    Err(Error::new(MethodNotAllowed, "Cannot perform a GET request to this resource."))
  }

  // TODO: doc
  fn post(&self, _: &Fn(Vec<String>) -> Url, _: &Service) -> Result<Data, Error> {
    Err(Error::new(MethodNotAllowed, "Cannot perform a POST request to this resource."))
  }

  // TODO: doc
  fn put(&self, _: &Fn(Vec<String>) -> Url, _: &Service) -> Result<Data, Error> {
    Err(Error::new(MethodNotAllowed, "Cannot perform a PUT request to this resource."))
  }

  // TODO: doc
  fn patch(&self, _: &Fn(Vec<String>) -> Url, _: &Service) -> Result<Data, Error> {
    Err(Error::new(MethodNotAllowed, "Cannot perform a PATCH request to this resource."))
  }

  // TODO: doc
  fn delete(&self, _: &Fn(Vec<String>) -> Url, _: &Service) -> Result<Data, Error> {
    Err(Error::new(MethodNotAllowed, "Cannot perform a DELETE request to this resource."))
  }
}

// TODO: doc
pub enum Data {
  None,
  Value(Value),
  Stream(Iter)
}

impl From<()> for Data {
  #[inline]
  fn from(_: ()) -> Self {
    Data::None
  }
}

impl From<Value> for Data {
  #[inline]
  fn from(value: Value) -> Self {
    Data::Value(value)
  }
}

impl From<Iter> for Data {
  #[inline]
  fn from(iter: Iter) -> Self {
    Data::Stream(iter)
  }
}
