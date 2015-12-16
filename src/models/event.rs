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

pub struct Event {
    id: ObjectId,
    name: String,
    date: DateTime<UTC>,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Event(ObjectId(\"{}\"), {})",
               self.id.bytes().to_hex(),
               self.name)
    }
}

impl Event {
    pub fn get_id(&self) -> &ObjectId {
        &self.id
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_date(&self) -> &DateTime<UTC> {
        &self.date
    }

    pub fn new(name: String, date: DateTime<UTC>) -> Event {
        Event {
            id: ObjectId::new().unwrap(),
            name: name,
            date: date,
        }
    }

    pub fn with_id(id: ObjectId, name: String, date: DateTime<UTC>) -> Event {
        let mut event = Event::new(name, date);
        event.id = id;
        event
    }

    pub fn latest(db: &Database) -> Result<Vec<Event>, Error> {
        let mut options = FindOptions::new();
        options.limit = 10;

        fn go(cursor: Cursor) -> Result<Vec<Event>, Error> {
            fn f(doc: Result<Document, Error>) -> Option<Event> {

                let event = mdo! {
                    ref res =<< doc;
                    ret ret(mdo! {
                        ref id =<< extract_object_id!(res, "_id");
                        name =<< extract_string!(res, "name");
                        date =<< extract_date!(res, "date");

                        ret ret(Event::with_id(id.to_owned(), name.to_owned(), date))
                    })
                };

                event.and_then(|x| x.or(Err(NOT_FOUND))).ok()
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
