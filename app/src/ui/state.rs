use db_sight_core::{events::SelectedTableChanged, ConnectionConfig};
use gpui::{App, AppContext, Entity, EventEmitter, Global, SharedString};
use gpui_component::notification::Notification;
use serde::{Deserialize, Serialize};

use crate::ui::{
    components::{ConnectionTabs, Loading},
    pages::PageRoute,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub theme: SharedString,
    pub collapsed: bool,
    #[serde(skip)]
    pub current_page: PageRoute,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Default for AppState {
    fn default() -> Self {
        Self {
            theme: "Ayu Dark".into(),
            collapsed: false,
            current_page: PageRoute::NoDatabase,
        }
    }
}

impl Global for AppState {}

// Loading State
#[derive(Debug, Clone)]
pub struct AppLoadingState {
    pub loading: Entity<Loading>,
}

impl AppLoadingState {
    pub fn new(cx: &mut App) -> Self {
        let loading = Loading::view(cx);
        Self { loading }
    }
}

impl Global for AppLoadingState {}

// Notification State
pub struct AppNotificationState {
    pub notifications: Vec<Notification>,
}

impl AppNotificationState {
    pub fn new() -> Self {
        Self {
            notifications: Vec::new(),
        }
    }
    pub fn push(&mut self, n: Notification) {
        self.notifications.push(n);
    }

    pub fn take(&mut self) -> Vec<Notification> {
        std::mem::take(&mut self.notifications)
    }
}

impl Global for AppNotificationState {}

// Connection Tabs State
#[derive(Debug, Clone)]
pub struct AppConnectionTabsState {
    pub connection_tabs: Entity<ConnectionTabs>,
}

impl AppConnectionTabsState {
    pub fn new(connection_tabs: Entity<ConnectionTabs>) -> Self {
        Self { connection_tabs }
    }

    /// Add a new connection config and refresh the tabs
    pub fn add_config(&self, config: ConnectionConfig, cx: &mut App) {
        cx.update_entity(&self.connection_tabs, |tabs, cx| {
            tabs.add_config(config, cx);
            cx.notify();
        });
    }
}

impl Global for AppConnectionTabsState {}

pub struct TableSelectionState {
    pub selected_table: Option<String>,
}
impl EventEmitter<SelectedTableChanged> for TableSelectionState {}
pub struct AppTableState {
    pub state: Entity<TableSelectionState>,
}
impl AppTableState {
    pub fn new(cx: &mut App) -> Self {
        let state = cx.new(|_| TableSelectionState {
            selected_table: None,
        });
        Self { state }
    }
}
impl Global for AppTableState {}
