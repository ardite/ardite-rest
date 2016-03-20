use ardite::error::{Error, MethodNotAllowed, NotFound};
use iron::prelude::*;
use iron::method::Method::*;
use iron::Handler;

use router::Route;

pub struct Server;

impl Handler for Server {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    match (Route::resolve_from_root(req.url.path.clone()), &req.method) {
      (Route::Root, &Get) => {
        Err(Error::unimplemented("Root route has not yet been implemented.").into())
      },
      (Route::Root, method @ _) => {
        Err(Error::new(MethodNotAllowed, format!("Cannot perform a {} request to the root resource.", method)).set_hint("Try a GET request instead.").into())
      },
      (Route::None, _) => {
        Err(Error::new(NotFound, "Resource not found.").set_hint("Check the root resource for available top level paths.").into())
      }
    }
  }
}
