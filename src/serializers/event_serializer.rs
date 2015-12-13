use rustc_serialize::hex::ToHex;

use jsonway::{self, ObjectSerializer};

use time;

use super::super::models::event;

pub struct EventSerializer;

impl jsonway::ObjectSerializer<event::Event> for EventSerializer {

    fn root(&self) -> Option<&str> { Some("event") }

    fn build(&self, event: &event::Event, json: &mut jsonway::ObjectBuilder) {
        json.set("id", event.get_id().bytes().to_hex().to_string());
        json.set("name", event.get_name().to_string());
        json.set("date", time::at_utc(event.get_date().clone()).rfc3339().to_string());
    }
}

pub struct EventListSerializer<'a> {
    events: &'a Vec<event::Event>
}

impl<'a> EventListSerializer<'a> {
    pub fn new(events: &'a Vec<event::Event>) -> EventListSerializer<'a> {
        EventListSerializer{
            events: events
        }
    }
}

impl<'a> jsonway::ArraySerializer for EventListSerializer<'a> {
    
    fn root(&self) -> Option<&str> { Some("events") }

    fn build(&self, array: &mut jsonway::ArrayBuilder) {
        for event in self.events.iter() {
            array.push(EventSerializer.serialize(event, false));
        }
    }
}