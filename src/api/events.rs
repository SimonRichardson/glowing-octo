use rustless;
use rustless::prelude::*;

use jsonway::{ObjectSerializer, ArraySerializer};

use url;

use super::super::models::event;
use super::super::db::DatabaseExt;
use super::super::serializers::event_serializer;

pub fn events(path: &str) -> rustless::Namespace {
    rustless::Namespace::build(path, |events| {

        events.get("latest", |endpoint| {
            endpoint.desc("Get latest events");
            endpoint.handle(|mut client, _| {

                let db = client.app.db();
                let events = event::Event::latest(&*db);
                match events {
                    Ok(events) => client.json(&event_serializer::EventListSerializer::new(&events).serialize(true)),
                    _ => {
                        client.not_found();
                        client.empty()
                    }
                }
            })
        });
    })
}