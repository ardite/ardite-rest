use ardite::Service;
use ardite::value::Value;
use ardite::error::{Error, MethodNotAllowed};
use inflections::Inflect;
use iron::prelude::*;
use iron::headers::{ContentType, ContentLength};
use iron::method;
use iron::modifiers::Header;
use iron::status::Status;
use iron::{Handler, Url};
use urlencoded::{UrlEncodedQuery, EmptyQuery};

use case::Case;
use resource::{Resource, Data};
use resource::root::Root;

pub struct Server {
  pub service: Service<'static>,
  pub root_url: Url,
  pub default_case: Case
}

impl Server {
  fn route<'a>(&'a self, path: Vec<String>) -> Option<Box<Resource + 'a>> {
    path.into_iter().fold(Some(Box::new(Root)), |opt_resource, part| {
      if part.is_kebab_case() {
        opt_resource.and_then(|resource| resource.route(self.service.definition(), part))
      } else {
        None
      }
    })
  }

  fn create_url(&self, path: Vec<String>) -> Url {
    let mut url = self.root_url.clone();
    for part in path {
      url.path.push(part.to_kebab_case());
    }
    url
  }

  /// Handles taking a request and turning it into a `Result<Value, Error>.`
  // TODO: Combine this with the `Handler` implementation.
  fn handle(&self, req: &mut Request) -> Result<Data, Error> {
    let mut path = req.url.path.clone();
    let create_url = &|path| self.create_url(path);

    // URLs like `google.com`, or `google.com/` have a path of `vec![""]`. We
    // would rather interpret this as the root path or `vec![]`.
    if path == vec![""] {
      path = vec![];
    }

    let mut resource = try!(self.route(path).ok_or(
      Error::not_found(format!("Resource '{}' not found.", self.create_url(req.url.path.clone())))
      .set_hint("Check the root resource for available top level paths.")
    ));

    // Get the query parameters and mutate the resource with it.
    match req.get_ref::<UrlEncodedQuery>() {
      Ok(ref query) => { resource.query(query); },
      Err(ref error) => {
        match error {
          &EmptyQuery => {},
          error @ _ => { return Err(Error::invalid(format!("{}", error), "Try fixing your query syntax.")); }
        };
      }
    };

    // TODO: `Allow` header.
    match &req.method {
      &method::Get => resource.get(create_url, &self.service),
      &method::Post => resource.post(create_url, &self.service),
      &method::Put => resource.put(create_url, &self.service),
      &method::Patch => resource.patch(create_url, &self.service),
      &method::Delete => resource.delete(create_url, &self.service),
      method @ _ => Err(
        Error::new(MethodNotAllowed, format!("Cannot perform a {} request on any resource in this API.", method))
        .set_hint("Try an OPTIONS request to see what other methods you can use.")
      )
    }
  }
}

// TODO: Combine this with the `Server` handle method.
impl Handler for Server {
  /// Handles taking a `Result<Value, Error>` and turning it into an `IronResult<Response>`.
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    // Allows users to ping the server.
    // TODO: *Only* try to use this after we can’t find any other `Resource`
    // to serve this request.
    if req.method == method::Options {
      use iron::headers::*;
      use iron::method::*;
      return Ok(
        Response::new()
        .set(Status::Ok)
        // TODO: `Allow` header isn’t actually accurate for every resource…
        // consider automating `OPTIONS` for every `Resource` which is always
        // accurate.
        .set(Header(Allow(vec![Get, Post, Put, Patch, Delete, Options])))
      );
    }

    let case = &self.default_case;

    match self.handle(req) {
      Ok(data) => {
        match data {
          Data::None => Ok(Response::new().set(Status::Ok)),
          Data::Value(value) => {
            let mut content = value_keys_to_case(value, case).to_json_pretty().unwrap();
            content.push_str("\n");

            let mut res = Response::new();

            res.set_mut(Status::Ok);
            res.set_mut(Header(ContentType(mime!(Application/Json; Charset=Utf8))));
            res.set_mut(Header(ContentLength(content.len() as u64)));
            res.set_mut(content);

            Ok(res)
          },
          // TODO: actually stream this…
          Data::Stream(iter) => {
            let array = Value::Array(iter.map(|value| value_keys_to_case(value, case)).collect());
            let mut content = array.to_json_pretty().unwrap();
            content.push_str("\n");

            let mut res = Response::new();

            res.set_mut(Status::Ok);
            res.set_mut(Header(ContentType(mime!(Application/Json; Charset=Utf8))));
            res.set_mut(Header(ContentLength(content.len() as u64)));
            res.set_mut(content);

            Ok(res)
          }
        }
      },
      Err(error) => {
        let mut content = error.to_value().to_json_pretty().unwrap();
        content.push_str("\n");

        let mut res = Response::new();

        res.set_mut(Status::from_u16(error.code().to_u16()));
        res.set_mut(Header(ContentType(mime!(Application/Json; Charset=Utf8))));
        res.set_mut(Header(ContentLength(content.len() as u64)));
        res.set_mut(content);

        Err(IronError {
          error: Box::new(error),
          response: res
        })
      }
    }
  }
}

fn value_keys_to_case(value: Value, case: &Case) -> Value {
  match value {
    value @ Value::Array(_) => value.map_values(|value| value_keys_to_case(value, case)),
    value @ _ => value.map_entries(|(key, value)| (case.to_case(&key), value_keys_to_case(value, case)))
  }
}
