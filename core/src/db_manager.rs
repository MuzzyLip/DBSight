use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::driver::DatabaseDriver;

pub struct DBManager {
    connections: RwLock<HashMap<String, Arc<dyn DatabaseDriver>>>,
}

impl DBManager {
    pub fn new() -> Self {
        Self {
            connections: RwLock::new(HashMap::new()),
        }
    }

    pub async fn add_connection(&self, key: String, driver: Arc<dyn DatabaseDriver>) {
        self.connections.write().await.insert(key, driver);
    }

    pub async fn get_connection(&self, key: &str) -> Option<Arc<dyn DatabaseDriver>> {
        self.connections.read().await.get(key).cloned()
    }
}
