extern crate ansi_term;
#[macro_use]
extern crate ardite;
#[macro_use]
extern crate clap;
extern crate inflector;
extern crate iron;
extern crate logger;
#[macro_use]
extern crate mime;

mod case;
mod router;
mod server;

use std::path::PathBuf;

use ansi_term::Colour::Red;
use ansi_term::Style;
use ardite::Service;
use clap::{App, Arg};
use clap::AppSettings::UnifiedHelpMessage;
use iron::{Iron, Chain, Url};
use logger::Logger;

use case::Case;
use server::RestServer;

macro_rules! handle_err {
  ($expr:expr) => {
    match $expr {
      Ok(value) => value,
      Err(error) => {
        println!("{} {}\n", Red.bold().paint("Error:"), error);
        std::process::exit(1);
      }
    }
  }
}

fn main() {
  let matches = {
    App::new("Ardite REST Server")
    .version(crate_version!())
    .author("Caleb Meredith <calebmeredith8@gmail.com>")
    .about("Ardite service providing a RESTful interface over HTTP.")
    .version_short("v")
    .setting(UnifiedHelpMessage)
    .args(&[
      Arg::with_name("schema").takes_value(true).required(true).default_value("ardite.yml").value_name("FILE").help("The Ardite schema file to be used"),
      Arg::with_name("hostname").long("hostname").short("n").takes_value(true).default_value("localhost").value_name("STRING").help("The host name that the server will listen on"),
      Arg::with_name("port").long("port").short("p").takes_value(true).default_value("3001").value_name("PORT").help("The port that the server will listen on"),
      Arg::with_name("mount").long("mount").short("m").takes_value(true).value_name("URL").help("All reported URLs will have this as the prefix"),
      Arg::with_name("case").long("case").short("c").takes_value(true).possible_values(&["camel", "snake", "kebab"]).default_value("kebab").value_name("CASE").help("The default case that the API will use, may be overrided with the `Prefer` header")
    ])
    .get_matches()
  };

  let schema_path = PathBuf::from(matches.value_of("schema").unwrap());
  let service = handle_err!(Service::from_file(schema_path.clone()));
  let mount_url = matches.value_of("mount").map(|url| handle_err!(Url::parse(url)));
  let default_case = Case::from_str(matches.value_of("case").unwrap()).unwrap();

  // Some properties on URL is not allowed, so throw some errors.
  if let Some(ref mount_url) = mount_url {
    if mount_url.username.is_some() { handle_err!(Err(format!("Username not allowed in mount URL '{}'.", mount_url))); }
    if mount_url.password.is_some() { handle_err!(Err(format!("Password not allowed in mount URL '{}'.", mount_url))); }
    if mount_url.query.is_some() { handle_err!(Err(format!("Query not allowed in mount URL '{}'.", mount_url))); }
    if mount_url.fragment.is_some() { handle_err!(Err(format!("Fragment not allowed in mount URL '{}'.", mount_url))); }
  }

  println!(
    "Loaded schema from {}",
    Style::new().underline().paint(format!("{}", schema_path.display()))
  );

  let server = RestServer {
    service: service,
    mount_url: mount_url,
    default_case: default_case
  };

  let mut chain = Chain::new(server);

  let (logger_before, logger_after) = Logger::new(None);

  chain.link_before(logger_before);
  chain.link_after(logger_after);

  let hostname = matches.value_of("hostname").unwrap();
  let port = handle_err!(matches.value_of("port").unwrap().parse::<u16>());

  println!(
    "REST server listening on {}",
    Style::new().underline().paint(format!("http://{}:{}", hostname, port))
  );

  handle_err!(Iron::new(chain).http((hostname, port)));
}
