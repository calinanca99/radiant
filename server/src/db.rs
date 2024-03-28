use std::collections::HashMap;

use bytes::Bytes;

#[derive(Debug)]
pub struct Db {
    entries: HashMap<String, Bytes>,
}

impl Db {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            entries: HashMap::default(),
        }
    }

    pub fn get(&self, k: &str) -> Option<Bytes> {
        self.entries.get(k).cloned()
    }

    pub fn set(&mut self, k: String, v: Bytes) {
        self.entries.insert(k, v);
    }

    pub fn del(&mut self, k: String) -> Option<()> {
        self.entries.remove(k.as_str()).map(|_| ())
    }
}
