use std::{collections::HashMap, sync::Arc};

use bytes::Bytes;
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct Db {
    data: Arc<RwLock<State>>,
}

impl Db {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let data = Arc::new(RwLock::new(State {
            entries: HashMap::default(),
        }));
        Self { data }
    }

    pub async fn get(&self, k: &str) -> Option<Bytes> {
        let state = self.data.read().await;
        state.entries.get(k).cloned()
    }

    pub async fn set(&mut self, k: String, v: Bytes) {
        let mut state = self.data.write().await;
        state.entries.insert(k, v);
    }
}

#[derive(Debug)]
struct State {
    entries: HashMap<String, Bytes>,
}
