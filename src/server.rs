use ardite::Service;
use ardite::error::{Error, MethodNotAllowed};
use iron::prelude::*;
use iron::headers::{ContentType, ContentLength};
use iron::method::Method::*;
use iron::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use iron::modifiers::Header;
use iron::status::Status;
use iron::Handler;

use router::Route;

pub struct RestServer {
  service: Service
}

impl Handler for RestServer {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    match (Route::resolve_from_root(req.url.path.clone()), &req.method) {
      (Route::Root, &Get) => {
        Err(Error::unimplemented("Root route has not yet been implemented."))
      },
      (Route::Root, method @ _) => {
        Err(Error::new(MethodNotAllowed, format!("Cannot perform a {} request to the root resource.", method)).set_hint("Try a GET request instead."))
      },
      (Route::None, _) => {
        Err(Error::not_found(format!("Resource '{}' not found.", req.url)).set_hint("Check the root resource for available top level paths."))
      }
    }.map_err(into_iron_error)
  }
}

fn into_iron_error(error: Error) -> IronError {
  let mut res = Response::new();

  res.set_mut(Status::from_u16(error.code().to_u16()));

  // Tries to send the error as a response in JSON, however, if that fails
  // the error is sent as plain text.
  if let Ok(content) = error.to_value().to_json_pretty() {
    res.set_mut(Header(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)]))));
    res.set_mut(Header(ContentLength(content.len() as u64)));
    res.set_mut(content);
  } else {
    res.set_mut(Header(ContentType(Mime(TopLevel::Text, SubLevel::Plain, vec![(Attr::Charset, Value::Utf8)]))));
    let content = format!("{}", error);
    res.set_mut(Header(ContentLength(content.len() as u64)));
    res.set_mut(content);
  }

  IronError {
    error: Box::new(error),
    response: res
  }
}
