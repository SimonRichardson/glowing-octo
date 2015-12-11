extern crate bodyparser;

use std::sync::Arc;

use rustc_serialize::json;

use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;

use uuid::Uuid;

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

handler!(GetEventsHandler);
impl Handler for GetEventsHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let all = self.repository.all();
        let all_json = json::encode(&all).unwrap();
        Ok(Response::with((status::Ok, all_json)))
    }
}

// POST `/events`
handler!(PostEventsHandler);
impl Handler for PostEventsHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let json_body = req.get::<bodyparser::Json>();
        return match json_body {
            Ok(Some(json_body)) => {
                let id = Uuid::new_v4().to_hyphenated_string();
                let json_object = json_body.as_object().unwrap();

                let new_name: String =  {
                    if json_object.get("name").is_some() {
                        String::from(json_object.get("name").unwrap().as_string().unwrap())
                    } else {
                        String::from("")
                    }
                };

                let event = Event::new(id.clone(), new_name);
                let event = self.repository.add(id, event);
                Ok(Response::with((status::Created, json::encode(&event).unwrap())))
            }
            Ok(None) => panic!("No body"),
            Err(err) => panic!("Error: {:?}", err)
        }
    }
}

// DELETE `/events`
handler!(DeleteEventsHandler);
impl Handler for DeleteEventsHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        self.repository.delete_all();
        Ok(Response::with(status::Ok))
    }
}