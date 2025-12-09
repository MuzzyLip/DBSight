use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use uuid::Uuid;

use crate::ConnectionConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DBConfig {
    pub version: String,
    pub connections: Vec<ConnectionConfig>,
    pub active_connection_ids: Vec<Uuid>,
}

impl DBConfig {
    pub fn new() -> Self {
        Self {
            version: "1.0".to_string(),
            connections: Vec::new(),
            active_connection_ids: Vec::new(),
        }
    }

    /// Load configuration from file
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: DBConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to file
    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }
}

impl Default for DBConfig {
    fn default() -> Self {
        Self::new()
    }
}
