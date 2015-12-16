#[macro_use]
extern crate mdo;
extern crate bson;
extern crate docopt;
extern crate iron;
extern crate jsonway;
extern crate mongodb;
extern crate rustc_serialize;
extern crate rustless;
extern crate time;
extern crate valico;
extern crate url;
extern crate uuid;
extern crate chrono;

use docopt::Docopt;

use mongodb::{Client, ThreadedClient};
use mongodb::db::Database;

use rustless::prelude::*;
use rustless::batteries::schemes;
use rustless::batteries::swagger;

use valico::json_schema;

use self::db::DatabaseExt;

mod db;
mod serializers;
mod models;
mod api;

#[cfg_attr(rustfmt, rustfmt_skip)]
const USAGE: &'static str = "
Events backend.

Usage:
  backend [--ip=<ip> --port=<port> --dbhost=<dbhost> --dbport=<dbport> --dbname=<dbname>] <command> [<args>...]
  backend [options]
  backend --version
  backend --help

Options:
  -h --help             Show this screen.
  --version             Show version.
  --ip=<ip>             Specify server ip [default: 127.0.0.1]
  --port=<port>         Specify server port [default: 3001]
  --dbhost=<dbhost>     Specify db host [default: 127.0.0.1]
  --dbport=<dbport>     Specify db port [default: 27017]
  --dbname=<dbname>     Specify db name [default: test]
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_command: Option<Command>,
    arg_args: Vec<String>,
    flag_ip: String,
    flag_port: u16,
    flag_dbhost: String,
    flag_dbport: u16,
    flag_dbname: String,
}

#[derive(Debug, RustcDecodable)]
enum Command {
    Run,
}

struct DBOptions {
    host: String,
    port: u16,
    name: String,
}

fn run_db(app: &mut rustless::Application, options: DBOptions) {
    let host = options.host.to_string();
    let client = Client::connect(&*host, options.port).unwrap();

    let name = options.name.to_string();
    let db: mongodb::db::Database = client.db(&*name);
    app.ext.insert::<self::db::AppDb>(db);
}

fn main() {
    let mut app = rustless::Application::new(self::api::root());

    swagger::enable(&mut app,
                    swagger::Spec {
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
                                url: "http://opensource.org/licenses/MIT".to_string(),
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

            let options = DBOptions {
                host: args.flag_dbhost,
                port: args.flag_dbport,
                name: args.flag_dbname,
            };

            run_db(&mut app, options);

            app.root_api.mount(swagger::create_api("api-docs"));

            schemes::enable_schemes(&mut app, json_schema::Scope::new()).unwrap();

            let chain = iron::Chain::new(app);
            let host = args.flag_ip.to_string();
            iron::Iron::new(chain).http((&*host, args.flag_port)).unwrap();
        }
        _ => {}
    }
}
