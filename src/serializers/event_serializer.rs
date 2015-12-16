use rustc_serialize::hex::ToHex;

use jsonway::{self, ObjectSerializer};

use models::event;

#[macro_export]
macro_rules! set {
    ($json: expr, $name: expr, $f: expr) => {{
        $json.set($name, )
    }};
}

pub struct EventSerializer;

impl jsonway::ObjectSerializer<event::Event> for EventSerializer {
    fn root(&self) -> Option<&str> {
        Some("event")
    }

    fn build(&self, event: &event::Event, json: &mut jsonway::ObjectBuilder) {

        // We should use the mdo syntax here!

        event.get_id().to_owned().map(|x| { json.set("id", x.bytes().to_hex().to_string()); x });
        event.get_name().to_owned().map(|x| { json.set("name", x.to_string()); x });
        event.get_date().to_owned().map(|x| { json.set("date", x.to_rfc3339()); x });
    }
}

pub struct EventListSerializer<'a> {
    events: &'a Vec<event::Event>,
}

impl<'a> EventListSerializer<'a> {
    pub fn new(events: &'a Vec<event::Event>) -> EventListSerializer<'a> {
        EventListSerializer { events: events }
    }
}

impl<'a> jsonway::ArraySerializer for EventListSerializer<'a> {
    fn root(&self) -> Option<&str> {
        Some("events")
    }

    fn build(&self, array: &mut jsonway::ArrayBuilder) {
        for event in self.events.iter() {
            array.push(EventSerializer.serialize(event, false));
        }
    }
}
