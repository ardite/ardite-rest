use ardite::{Error, Service};
use iron::Url;

use resource::{Resource, Data};

pub struct Collection {
  name: String
}

impl Collection {
  pub fn new(name: String) -> Self {
    Collection {
      name: name
    }
  }
}

impl Resource for Collection {
  fn get(&self, _: &Fn(Vec<String>) -> Url, service: &Service) -> Result<Data, Error> {
    service.read(
      &self.name,
      Default::default(),
      Default::default(),
      Default::default()
    ).map(Data::Stream)
  }
}
