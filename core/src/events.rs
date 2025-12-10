use crate::ConnectionConfig;

/// Event emitted when active connections list changes
#[derive(Debug, Clone)]
pub struct ActiveConnectionsChanged {
    pub active_configs: Vec<ConnectionConfig>,
}

/// Event emitted when selected connection changes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectedConnectionChanged {
    pub id: Option<uuid::Uuid>,
}
