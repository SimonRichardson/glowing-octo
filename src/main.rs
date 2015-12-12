extern crate rustless;
extern crate time;
extern crate rustc_serialize;
extern crate iron;
extern crate docopt;
extern crate uuid;
extern crate jsonway;
extern crate valico;
extern crate url;

use rustless::prelude::*;
use rustless::batteries::schemes;
use rustless::batteries::swagger;
use valico::json_schema;
use docopt::Docopt;

mod serializers;
mod models;
mod api;

const USAGE: &'static str = "
Events backend.

Usage:
  backend [--ip=<ip> --port=<port>] <command> [<args>...]
  backend [options]
  backend --version
  backend --help

Options:
  -h --help        Show this screen.
  --version        Show version.
  --ip=<ip>        Specify server ip [default: 127.0.0.1]
  --port=<port>    Specify server port [default: 3001]
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_command: Option<Command>,
    arg_args: Vec<String>,
    flag_ip: String,
    flag_port: u16,
}

#[derive(Debug, RustcDecodable)]
enum Command {
    Run,
}

fn main() {
    let mut app = rustless::Application::new(self::api::root());

    swagger::enable(&mut app, swagger::Spec {
        info: swagger::Info {
            title: "Events API".to_string(),
            description: Some("Events API document".to_string()),
            contact: Some(swagger::Contact {
                name: "Simon Richardson".to_string(),
                url: Some("http://dice.fm".to_string()),
                ..std::default::Default::default()
            }),
            license: Some(swagger::License {
                name: "MIT".to_string(),
                url: "http://opensource.org/licenses/MIT".to_string()
            }),
            ..std::default::Default::default()
        },
        ..std::default::Default::default()
    });

    let version = "0.0.1".to_owned();
    let args: Args = Docopt::new(USAGE)
                        .and_then(|dopt| dopt.version(Some(version)).options_first(true).decode())
                        .unwrap_or_else(|e| e.exit());


    match args.arg_command {
        Some(_) => {
            app.root_api.mount(swagger::create_api("api-docs"));
            
            schemes::enable_schemes(&mut app, json_schema::Scope::new()).unwrap();

            let chain = iron::Chain::new(app);
            let host = args.flag_ip.to_string();
            iron::Iron::new(chain).http((&*host, args.flag_port)).unwrap();
        }
        _ => {}
    }
}