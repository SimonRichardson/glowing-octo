use std::collections::HashMap;
use std::sync::Mutex;

pub struct Repository<T> where T: Clone {
    entities: Mutex<HashMap<String, T>>,
}

impl<T> Repository<T> where T: Clone {

    pub fn new() -> Repository<T> {
        Repository { entities: Mutex::new(HashMap::new()) }
    }

    #[allow(dead_code)]
    pub fn add(&self, key: String, value: T) -> T where T: Clone {
        let mut m_entities = self.entities.lock().unwrap();
        m_entities.insert(key, value.clone());
        value
    }

    pub fn all(&self) -> Vec<T> {
        let m_entities = self.entities.lock().unwrap();
        let mut values = Vec::new();
        for (_, v) in m_entities.iter() {
            values.push((*v).clone());
        }
        values
    }
}
