use ardite::error::{Error, NotFound};
use iron::prelude::*;
use iron::Handler;

pub struct Server;

impl Handler for Server {
  fn handle(&self, _: &mut Request) -> IronResult<Response> {
    Err(Error::new(
      NotFound,
      "Resource not found.",
      Some("Check the root path for available top level paths.")
    ).into())
  }
}
