#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Event {
    pub id: String,
    pub name: String,
}

impl Event {
    #[allow(dead_code)]
    pub fn new(id: String, name: String) -> Event {
        Event { id: id, name: name }
    }
}
