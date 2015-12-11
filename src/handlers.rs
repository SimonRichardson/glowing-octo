extern crate bodyparser;

use std::sync::Arc;

use rustc_serialize::json;
use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;

use repository::Repository;
use event::Event;

macro_rules! handler {
    ($x:ident) => {
        pub struct $x {
            repository: Arc<Repository<Event>>,
        }

        impl $x {
            pub fn new(repository: Arc<Repository<Event>>) -> $x {
                $x { repository: repository }
            }
        }
    }
}

// GET `/events`

handler!(GETEventsHandler);
impl Handler for GETEventsHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let all = self.repository.all();
        let all_json = json::encode(&all).unwrap();
        Ok(Response::with((status::Ok, all_json)))
    }
}
