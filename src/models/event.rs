use uuid;
use time;

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

    pub fn latest() -> Vec<Event> {
        let events = vec![];
        events
    }
}

