#[macro_use(crate_version)]
extern crate clap;

use clap::{App, Arg};
use clap::AppSettings::UnifiedHelpMessage;

fn main() {
  let matches = {
    App::new("Ardite REST Server")
    .version(crate_version!())
    .author("Caleb Meredith <calebmeredith8@gmail.com>")
    .about("Ardite service providing a RESTful interface over HTTP.")
    .version_short("v")
    .setting(UnifiedHelpMessage)
    .args(&[
      Arg::with_name("hostname").long("hostname").short("n").takes_value(true).value_name("STRING").help("The host name that the server will listen on"),
      Arg::with_name("port").long("port").short("p").takes_value(true).value_name("PORT").help("The port that the server will listen on")
    ])
    .get_matches()
  };

  let hostname = matches.value_of("hostname").unwrap_or("localhost");
  let port = matches.value_of("port").unwrap_or("3002").parse::<u16>().unwrap();
}
