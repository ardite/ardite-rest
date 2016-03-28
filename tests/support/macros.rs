use std::process::Child;
use std::ops::Drop;

use hyper::Url;
use hyper::client::{Client, RequestBuilder};
use hyper::method::Method;

pub static mut PORT: u32 = 3000;

pub struct Server {
  pub port: u32,
  pub process: Child,
  pub client: Client
}

impl Server {
  pub fn url(&self, path: &'static str) -> Url {
    Url::parse(&format!("http://localhost:{}{}", self.port, path)).unwrap()
  }

  #[inline]
  pub fn request(&self, method: Method, path: &'static str) -> RequestBuilder {
    self.client.request(method, self.url(path))
  }

  #[inline] pub fn get(&self, path: &'static str) -> RequestBuilder { self.request(Method::Get, path) }

  // TODO: Use these so I don’t have to allow dead code…
  #[allow(dead_code)] #[inline] pub fn post(&self, path: &'static str) -> RequestBuilder { self.request(Method::Post, path) }
  #[allow(dead_code)] #[inline] pub fn put(&self, path: &'static str) -> RequestBuilder { self.request(Method::Put, path) }
  #[allow(dead_code)] #[inline] pub fn patch(&self, path: &'static str) -> RequestBuilder { self.request(Method::Patch, path) }
  #[allow(dead_code)] #[inline] pub fn delete(&self, path: &'static str) -> RequestBuilder { self.request(Method::Delete, path) }
}

// This struct drops children 😉
impl Drop for Server {
  fn drop(&mut self) {
    // Pray we don’t unwrap an `Err`…
    self.process.kill().unwrap();
  }
}

macro_rules! serve {
  ($($arg:expr),*) => {{
    use ::std::process::{Stdio, Command};
    use $crate::hyper::{Url, Client};
    use $crate::hyper::method::Method::Options;
    use $crate::support::macros::{PORT, Server};

    // Get the next port value. Pray there is no thread wierdness.
    let port: u32 = unsafe { PORT += 1; PORT };

    let mut command = Command::new("target/debug/ardite-rest");
    command.stdout(Stdio::null());
    command.args(&["--port", &port.to_string()]);

    // Add the arguments.
    $(
      command.arg($arg);
    )*

    let process = command.spawn().unwrap();

    let client = Client::new();

    {
      let url = Url::parse(&format!("http://localhost:{}", port)).unwrap();
      let mut pings = 0;

      // Keep trying to ping the server until it responds.
      loop {
        pings += 1;

        if pings >= 10 {
          panic!("Failed to launch the server after 10 pings.");
        }

        // If there was no error, break the loop, otherwise let the loop repeat
        // after a 0.1 second rest.
        match client.request(Options, url.clone()).send() {
          Ok(_) => { break; },
          Err(_) => { Command::new("sleep").arg("0.1").output().unwrap(); }
        }
      }
    }

    Server {
      port: port,
      process: process,
      client: client
    }
  }}
}
