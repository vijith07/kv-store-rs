use std::{collections::HashMap, time::Instant};

pub struct Store {
    data: HashMap<String, Entry>
}

struct Entry {
    t : Option<Instant>,
    value: String
}

impl Store {
    pub fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }



    pub fn get(&self, key: String) -> Option<String> {
        let entry = self.data.get(key.as_str());
        if let Some(entry) = entry {
            if let Some(t) = entry.t {
                if t.elapsed().as_secs() > 0 {
                    return None;
                }
            }
            return Some(entry.value.clone());
        }
        None
    }



    pub fn set(&mut self, key: String, value: String) {
        let entry = Entry {
            t: None,
            value
        };
        self.data.insert(key, entry);
    }

    pub fn set_with_expiry(&mut self, key: String, value: String, seconds: u64) {
        let entry = Entry {
            t: Some(Instant::now() + std::time::Duration::from_secs(seconds)),
            value
        };
        self.data.insert(key, entry);
    }

    pub fn del(&mut self, key: String) {
        self.data.remove(key.as_str());
    }

    pub fn expire(&mut self, key: String, seconds: u64) -> bool {
        let entry = self.data.get_mut(key.as_str());
        if let Some(entry) = entry {
            entry.t = Some(Instant::now());
            return true;
        }
        false
    }

    pub fn persist(&mut self, key: String) -> bool {
        let entry = self.data.get_mut(key.as_str());
        if let Some(entry) = entry {
            entry.t = None;
            return true;
        }
        false
    }

    pub fn ttl(&self, key: String) -> Option<u64> {
        let entry = self.data.get(key.as_str());
        if let Some(entry) = entry {
            if let Some(t) = entry.t {
                let now = Instant::now();
                let duration = now.duration_since(t);
                let seconds = duration.as_secs();
                return Some(seconds);
            }
        }
        None
    }

    //exists 
    pub fn exists(&self, key: String) -> bool {
        self.data.contains_key(key.as_str())
    }

    pub fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    pub fn flush(&mut self) {
        self.data.clear();
    }
}