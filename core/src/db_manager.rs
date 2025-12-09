use anyhow::Result;
use gpui::{EventEmitter, Global};
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{driver::DatabaseDriver, ConnectionConfig, DBConfig};

/// Event emitted when active connections list changes
#[derive(Debug, Clone)]
pub struct ActiveConnectionsChanged {
    pub active_configs: Vec<ConnectionConfig>,
}

#[derive(Clone)]
pub struct DBManager {
    /// Active database-driven instances (connected)
    connections: Arc<RwLock<HashMap<String, Arc<dyn DatabaseDriver>>>>,
    /// Unified configuration (connections + active IDs)
    config: Arc<RwLock<DBConfig>>,
    /// Config file Directory
    config_dir: Arc<PathBuf>,
}

impl DBManager {
    pub fn new() -> Self {
        let config_dir = dirs::config_dir()
            .map(|d| d.join("db-sight"))
            .unwrap_or_else(|| PathBuf::from("."));
        if !config_dir.exists() {
            let _ = fs::create_dir_all(&config_dir);
        }
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            config: Arc::new(RwLock::new(DBConfig::new())),
            config_dir: Arc::new(config_dir),
        }
    }

    // ========== Configuration file path ==========

    fn db_config_path(&self) -> PathBuf {
        self.config_dir.join("db_config.json")
    }

    // ========== Configuration Management ==========

    /// Load configuration from file
    pub async fn load_config(&self) -> Result<()> {
        let new_path = self.db_config_path();

        let loaded_config = if new_path.exists() {
            // Load from new unified format
            DBConfig::load_from_file(&new_path)?
        } else {
            // No existing config, use default
            DBConfig::new()
        };

        *self.config.write().await = loaded_config;
        Ok(())
    }

    /// Persist configuration to file
    async fn persist_config(&self) -> Result<()> {
        let config = self.config.read().await;
        config.save_to_file(&self.db_config_path())?;
        Ok(())
    }

    // ========== Connection Configuration Management ==========

    /// Save individual connection configuration (new / updated)
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

        let mut db_config = self.config.write().await;
        // If Exists Update. Else Push
        if let Some(idx) = db_config.connections.iter().position(|c| c.id == config.id) {
            db_config.connections[idx] = config;
        } else {
            db_config.connections.push(config);
        }
        drop(db_config);

        self.persist_config().await?;
        Ok(())
    }

    /// Get all connection configurations
    pub async fn get_all_configs(&self) -> Vec<ConnectionConfig> {
        self.config.read().await.connections.clone()
    }

    /// Get connection configuration by ID
    pub async fn get_config_by_id(&self, id: &Uuid) -> Option<ConnectionConfig> {
        self.config
            .read()
            .await
            .connections
            .iter()
            .find(|c| c.id == *id)
            .cloned()
    }

    // ========== Active Connection Management ==========

    /// Add active connection
    pub async fn add_active_connection(&self, config_id: Uuid) -> Result<()> {
        let mut config = self.config.write().await;
        if !config.active_connection_ids.contains(&config_id) {
            config.active_connection_ids.push(config_id);
        }
        drop(config);
        self.persist_config().await
    }

    /// Remove active connections
    pub async fn remove_active_connection(&self, config_id: &Uuid) -> Result<()> {
        let mut config = self.config.write().await;
        config.active_connection_ids.retain(|id| id != config_id);
        drop(config);
        self.persist_config().await
    }

    /// Get the configuration list for active connections
    pub async fn get_active_configs(&self) -> Vec<ConnectionConfig> {
        let config = self.config.read().await;
        config
            .active_connection_ids
            .iter()
            .filter_map(|id| config.connections.iter().find(|c| c.id == *id).cloned())
            .collect()
    }

    /// Get the list of active connection IDs
    pub async fn get_active_config_ids(&self) -> Vec<Uuid> {
        self.config.read().await.active_connection_ids.clone()
    }

    // ========== Database-Driven Instance Management ==========

    /// Add connected database driver instances
    pub async fn add_connection(&self, key: String, driver: Arc<dyn DatabaseDriver>) {
        self.connections.write().await.insert(key, driver);
    }

    /// Get a database driver instance
    pub async fn get_connection(&self, key: &str) -> Option<Arc<dyn DatabaseDriver>> {
        self.connections.read().await.get(key).cloned()
    }

    /// Get all database driver instances
    pub async fn get_all_connections(&self) -> HashMap<String, Arc<dyn DatabaseDriver>> {
        self.connections.read().await.clone()
    }

    // ========== Combined Operations ==========

    /// Save connection config and add to active connections in one operation.
    /// This is the recommended method to call when user saves a new connection.
    pub async fn save_and_activate_connection(
        &self,
        config: ConnectionConfig,
        password: Option<String>,
    ) -> Result<ConnectionConfig> {
        let config_id = config.id;
        self.save_config(config.clone(), password).await?;
        self.add_active_connection(config_id).await?;
        Ok(config)
    }
}

impl EventEmitter<ActiveConnectionsChanged> for DBManager {}

impl Global for DBManager {}

impl Default for DBManager {
    fn default() -> Self {
        Self::new()
    }
}
