extern crate ansi_term;
extern crate ardite;
#[macro_use(crate_version)]
extern crate clap;
extern crate iron;
extern crate logger;

mod router;
mod server;

use std::path::PathBuf;

use ansi_term::Colour::Red;
use ansi_term::Style;
use ardite::Service;
use clap::{App, Arg};
use clap::AppSettings::UnifiedHelpMessage;
use iron::{Iron, Chain};
use logger::Logger;

use server::Server;

macro_rules! handle_err {
  ($expr:expr) => {
    match $expr {
      Ok(value) => value,
      Err(error) => {
        println!("\n{}", Red.bold().paint("Error:"));
        println!("{}\n", error);
        return ();
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
      Arg::with_name("port").long("port").short("p").takes_value(true).default_value("3001").value_name("PORT").help("The port that the server will listen on")
    ])
    .get_matches()
  };

  let schema = matches.value_of("schema").unwrap();

  let service = handle_err!(Service::from_file(PathBuf::from(schema)));

  let hostname = matches.value_of("hostname").unwrap();
  let port = handle_err!(matches.value_of("port").unwrap().parse::<u16>());

  let server = Server;

  let mut chain = Chain::new(server);

  let (logger_before, logger_after) = Logger::new(None);

  chain.link_before(logger_before);
  chain.link_after(logger_after);

  println!(
    "REST server listening on {}",
    Style::new().underline().paint(format!("http://{}:{}", hostname, port))
  );

  handle_err!(Iron::new(chain).http((hostname, port)));
}
