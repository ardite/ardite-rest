use ardite::Service;
use ardite::case::{Case, Kebab};
use ardite::error::{Error, MethodNotAllowed};
use iron::prelude::*;
use iron::headers::{ContentType, ContentLength};
use iron::method;
use iron::modifiers::Header;
use iron::status::Status;
use iron::{Handler, Url};

use resource::Resource;
use resource::root::Root;

pub struct Server {
  pub service: Service<'static>,
  pub root_url: Url,
  pub default_case: Case
}

impl Server {
  fn route(&self, path: Vec<String>) -> Option<Box<Resource>> {
    path.into_iter().fold(Some(Box::new(Root)), |opt_resource, part| {
      opt_resource.and_then(|resource| resource.route(&self.service.definition(), part))
    })
  }

  fn create_url(&self, path: Vec<String>) -> Url {
    let mut url = self.root_url.clone();
    for part in path {
      url.path.push(Kebab.to_case(part));
    }
    url
  }
}

impl Handler for Server {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let mut path = req.url.path.clone();
    let create_url = &|path| self.create_url(path);
    let case = &self.default_case;

    // URLs like `google.com`, or `google.com/` have a path of `vec![""]`. We
    // would rather interpret this as the root path or `vec![]`.
    if path == vec![""] {
      path = vec![];
    }

    let resource = self.route(path);

    let result = match resource {
      Some(resource) => match &req.method {
        &method::Get => resource.get(create_url, &self.service),
        &method::Post => resource.post(create_url, &self.service),
        &method::Put => resource.put(create_url, &self.service),
        &method::Patch => resource.patch(create_url, &self.service),
        &method::Delete => resource.delete(create_url, &self.service),
        method @ _ => Err(
          Error::new(MethodNotAllowed, format!("Cannot perform a {} request on any resource in this API.", method))
          .set_hint("Try a HEAD, GET, POST, PUT, PATCH, or DELETE request instead.")
        )
      },
      None => Err(
        Error::not_found(format!("Resource '{}' not found.", self.create_url(req.url.path.clone())))
        .set_hint("Check the root resource for available top level paths.")
      )
    };

    match result {
      Ok(value) => {
        let mut content = value.keys_to_case(case).to_json_pretty().unwrap();
        content.push_str("\n");

        let mut res = Response::new();

        res.set_mut(Status::Ok);
        res.set_mut(Header(ContentType(mime!(Application/Json; Charset=Utf8))));
        res.set_mut(Header(ContentLength(content.len() as u64)));
        res.set_mut(content);

        Ok(res)
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
