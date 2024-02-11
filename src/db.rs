use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bytes::Bytes;

#[derive(Clone, Debug)]
pub struct Db {
    data: Arc<Mutex<State>>,
}

impl Db {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let data = Arc::new(Mutex::new(State {
            entries: HashMap::default(),
        }));
        Self { data }
    }

    pub fn get(&self, k: &str) -> Option<Bytes> {
        let state = self.data.lock().unwrap();
        state.entries.get(k).cloned()
    }

    pub fn set(&mut self, k: String, v: Bytes) {
        let mut state = self.data.lock().unwrap();
        state.entries.insert(k, v);
    }
}

#[derive(Debug)]
struct State {
    entries: HashMap<String, Bytes>,
}
