use iron::prelude::*;
use iron::status::Status;
use iron::Handler;

pub struct Server;

impl Handler for Server {
  fn handle(&self, _: &mut Request) -> IronResult<Response> {
    Ok(Response::with((Status::Ok, "Hello, world!")))
  }
}
