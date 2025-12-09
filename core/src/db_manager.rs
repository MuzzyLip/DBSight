use anyhow::Result;
use gpui::Global;
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::{driver::DatabaseDriver, ConnectionConfig};

#[derive(Clone)]
pub struct DBManager {
    // Active database-driven instances (connected)
    connections: Arc<RwLock<HashMap<String, Arc<dyn DatabaseDriver>>>>,
    // Persistent Connection Configuration
    configs: Arc<RwLock<Vec<ConnectionConfig>>>,
    // config file path
    config_path: Arc<PathBuf>,
}

impl DBManager {
    pub fn new() -> Self {
        let config_dir = dirs::config_dir()
            .map(|d| d.join("db-sight"))
            .unwrap_or_else(|| PathBuf::from("."));
        if !config_dir.exists() {
            let _ = fs::create_dir_all(&config_dir);
        }
        let config_path = config_dir.join("connections.json");
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            configs: Arc::new(RwLock::new(Vec::new())),
            config_path: Arc::new(config_path),
        }
    }

    /// Load Configs
    pub async fn load_configs(&self) -> Result<()> {
        if !self.config_path.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&*self.config_path)?;
        let configs: Vec<ConnectionConfig> = serde_json::from_str(&content)?;

        *self.configs.write().await = configs;
        Ok(())
    }

    /// Save Configs
    pub async fn save_config(
        &self,
        mut config: ConnectionConfig,
        password: Option<String>,
    ) -> Result<()> {
        if let Some(pwd) = password {
            let entry = keyring::Entry::new("db-sight", &config.id.to_string())?;
            entry.set_password(&pwd)?;
            config.saved_password_len = Some(pwd.len() as u8);
        }

        let mut configs = self.configs.write().await;
        // If Exists Update. Else Push
        if let Some(idx) = configs.iter().position(|c| c.id == config.id) {
            configs[idx] = config;
        } else {
            configs.push(config);
        }
        let json = serde_json::to_string_pretty(&*configs)?;
        fs::write(&*self.config_path, json)?;

        Ok(())
    }

    pub async fn add_connection(&self, key: String, driver: Arc<dyn DatabaseDriver>) {
        self.connections.write().await.insert(key, driver);
    }

    pub async fn get_connection(&self, key: &str) -> Option<Arc<dyn DatabaseDriver>> {
        self.connections.read().await.get(key).cloned()
    }
}

impl Global for DBManager {}

impl Default for DBManager {
    fn default() -> Self {
        Self::new()
    }
}
