use uuid;
use time;

use bson::Bson;
use mongodb::db::{Database, ThreadedDatabase};
use mongodb::coll::options::{FindOptions};

pub struct Event {
    id: uuid::Uuid,
    name: String,
    date: time::Timespec
}

impl Event {

    pub fn get_id(&self) -> &uuid::Uuid { &self.id }
    pub fn get_name(&self) -> &str { &self.name }
    pub fn get_date(&self) -> &time::Timespec { &self.date }

    pub fn new(name: String, date: time::Timespec) -> Event {
        Event {
            id: uuid::Uuid::new_v4(),
            name: name,
            date: date
        }
    }

    pub fn latest(db: &Database) -> Vec<Event> {
        let coll = db.collection("events");

        let mut options = FindOptions::new();
        options.limit = 10;

        let mut cursor = coll.find(None, Some(options)).ok().expect("Failed.");

        let mut events = vec![];
        for doc in cursor {
            match doc {
                Ok(res) => {
                    let mut name = "";
                    match res.get("name") {
                        Some(&Bson::String(ref val)) => name = val,
                        _ => panic!("Unexpected type"),
                    }
                    let now = time::now().to_timespec();
                    events.push(Event::new(name.to_string(), now));
                },
                _ => panic!("Fucked!")
            }
        }
        events
    }
}

