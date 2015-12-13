use time;

use bson::oid::ObjectId;

use mongodb::db::{Database, ThreadedDatabase};
use mongodb::coll::options::{FindOptions};

#[macro_export]
macro_rules! find {
    ($db:expr, $name:expr, $query:expr) => {{
        find!($db, $name, $query, None)
    }};
    ($db:expr, $name:expr, $query:expr, $options:expr) => {{
        let coll = $db.collection($name);
        coll.find($query, $options).ok().expect("Not Found.")
    }};
}

#[macro_export]
macro_rules! extract_object_id {
    ($res:expr, $name:expr) => {{
        match $res.get_object_id($name) {
            Ok(val) => val.to_owned(),
            _ => panic!("Not Found.")
        }
    }}
}

#[macro_export]
macro_rules! extract_string {
    ($res:expr, $name:expr) => {{
        match $res.get_str($name) {
            Ok(val) => val.to_string(),
            _ => panic!("Not Found.")
        }
    }}
}

pub struct Event {
    id: ObjectId,
    name: String,
    date: time::Timespec
}

impl Event {

    pub fn get_id(&self) -> &ObjectId { &self.id }
    pub fn get_name(&self) -> &str { &self.name }
    pub fn get_date(&self) -> &time::Timespec { &self.date }

    pub fn new(name: String, date: time::Timespec) -> Event {
        Event {
            id: ObjectId::new().unwrap(),
            name: name,
            date: date
        }
    }

    pub fn with_id(id: ObjectId, name: String, date: time::Timespec) -> Event {
        let mut event = Event::new(name, date);
        event.id = id;
        event
    }

    pub fn latest(db: &Database) -> Vec<Event> {
        let mut options = FindOptions::new();
        options.limit = 10;

        let cursor = find!(db, "events", None, Some(options));

        let mut events = vec![];
        for doc in cursor {
            match doc {
                Ok(res) => {
                    let id = extract_object_id!(res, "_id");
                    let name = extract_string!(res, "name");
                    let now = time::now().to_timespec();
                    events.push(Event::with_id(id, name, now));
                },
                _ => panic!("Fucked!")
            }
        }
        events
    }
}

