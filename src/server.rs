use ardite::{Service, Value};
use ardite::error::{Error, MethodNotAllowed};
use ardite::value::Object;
use iron::prelude::*;
use iron::headers::{ContentType, ContentLength};
use iron::method::Method::*;
use iron::modifiers::Header;
use iron::status::Status;
use iron::{Handler, Url};

use case::Case;
use case::Case::Kebab;
use router::Route;

pub struct RestServer {
  pub service: Service<'static>,
  pub mount_url: Option<Url>,
  pub default_case: Case
}

impl RestServer {
  fn handle(&self, req: &mut Request) -> Result<Response, Error> {
    let mut res = Response::new();

    res.set_mut(Status::Ok);
    res.set_mut(Header(ContentType(mime!(Application/Json; Charset=Utf8))));

    let case = &self.default_case;

    match (Route::resolve_from_root(req.url.path.clone()), &req.method) {
      (Route::Root, &Get) => {
        let mut object = Object::new();

        for (name, _) in self.service.types() {
          let key = case.to_url_key(name.clone());
          let value = value!(format!("{}", self.from_root_url(&req.url, vec![Kebab.to_case(name.clone())])));
          object.insert(key, value);
        }

        let content = try!(Value::Object(object).to_json_pretty());

        res.set_mut(Header(ContentLength(content.len() as u64)));
        res.set_mut(content);

        Ok(res)
      },
      (Route::Root, method @ _) => Err(
        Error::new(MethodNotAllowed, format!("Cannot perform a {} request to the root resource.", method))
        .set_hint("Try a GET request instead.")
      ),
      (Route::Nothing, _) => Err(
        Error::not_found(format!("Resource '{}' not found.", self.mounted_url(&req.url)))
        .set_hint("Check the root resource for available top level paths.")
      )
    }
  }
}

impl RestServer {
  fn mounted_url(&self, url: &Url) -> Url {
    if let Some(ref mount_url) = self.mount_url {
      let mut mount_url = mount_url.clone();
      mount_url.username = url.username.clone();
      mount_url.password = url.password.clone();
      mount_url.query = url.query.clone();
      mount_url.fragment = url.fragment.clone();
      for part in url.path.iter() {
        mount_url.path.push(part.clone());
      }
      mount_url
    } else {
      url.clone()
    }
  }

  fn from_root_url(&self, url: &Url, path: Vec<String>) -> Url {
    if let Some(ref mount_url) = self.mount_url {
      let mut new_url = mount_url.clone();
      new_url.username = url.username.clone();
      new_url.password = url.password.clone();
      new_url.query = url.query.clone();
      new_url.fragment = url.fragment.clone();
      for part in path {
        new_url.path.push(part);
      }
      new_url
    } else {
      let mut new_url = url.clone();
      new_url.path = path;
      new_url
    }
  }
}

impl Handler for RestServer {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    self.handle(req).map_err(|error| {
      let mut res = Response::new();

      res.set_mut(Status::from_u16(error.code().to_u16()));

      // Tries to send the error as a response in JSON, however, if that fails
      // the error is sent as plain text.
      if let Ok(content) = error.to_value().to_json_pretty() {
        res.set_mut(Header(ContentType(mime!(Application/Json; Charset=Utf8))));
        res.set_mut(Header(ContentLength(content.len() as u64)));
        res.set_mut(content);
      } else {
        res.set_mut(Header(ContentType(mime!(Text/Plain; Charset=Utf8))));
        let content = format!("{}", error);
        res.set_mut(Header(ContentLength(content.len() as u64)));
        res.set_mut(content);
      }

      IronError {
        error: Box::new(error),
        response: res
      }
    })
  }
}
