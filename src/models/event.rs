use models::*;

use std::fmt;

use rustc_serialize::hex::ToHex;

use mdo::result::*;

use bson::Document;
use bson::oid::ObjectId;

use mongodb::db::{Database, ThreadedDatabase};
use mongodb::error::Error;
use mongodb::coll::options::FindOptions;
use mongodb::cursor::Cursor;

use chrono::{DateTime, UTC};

use std::default;

pub struct Event {
    id: Option<ObjectId>,
    date: Option<DateTime<UTC>>,
    date_limit: Option<DateTime<UTC>>,
    sale_start_date: Option<DateTime<UTC>>,
    name: Option<String>,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // This is wrong!
        write!(f,
               "Event(ObjectId(\"{}\"), {})",
               self.id.to_owned().unwrap().bytes().to_hex(),
               self.name.to_owned().unwrap())
    }
}

impl default::Default for Event {
    fn default() -> Event {
        Event {
            id: None,
            date: None,
            date_limit: None,
            sale_start_date: None,
            name: None,
        }
    }
}

impl Event {
    pub fn get_id(&self) -> &Option<ObjectId> {
        &self.id
    }
    pub fn get_name(&self) -> &Option<String> {
        &self.name
    }
    pub fn get_date(&self) -> &Option<DateTime<UTC>> {
        &self.date
    }

    pub fn new(name: String, date: DateTime<UTC>) -> Event {
        Event {
            id: ObjectId::new().ok(),
            name: Some(name),
            date: Some(date),
            ..default::Default::default()
        }
    }

    pub fn with_id(id: ObjectId, name: String, date: DateTime<UTC>) -> Event {
        let mut event = Event::new(name, date);
        event.id = Some(id);
        event
    }

    fn create(id: Option<ObjectId>, name: Option<String>, date: Option<DateTime<UTC>>) -> Event {
        Event {
            id: id,
            name: name,
            date: date,
            ..default::Default::default()
        }
    }

    pub fn latest(db: &Database) -> Result<Vec<Event>, Error> {
        let mut options = FindOptions::new();
        options.limit = 10;

        fn go(cursor: Cursor) -> Result<Vec<Event>, Error> {
            fn f(doc: Result<Document, Error>) -> Option<Event> {

                let event = mdo! {
                    ref res =<< doc;
                    
                    let id = extract_object_id!(res, "_id");
                    let name = extract_string!(res, "name");
                    let date = extract_date!(res, "date");

                    ret ret(Event::create(id, name, date))
                };

                event.ok()
            }

            let mut result = Vec::new();
            Ok(fold!(cursor, result, f))
        }

        let cursor = find!(db, "events", None, Some(options));
        return mdo! {
            c =<< cursor;
            ret go(c)
        };
    }
}
