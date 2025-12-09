use db_sight_core::ConnectionConfig;
use gpui::{App, AppContext, Entity, Global};
use gpui_component::notification::Notification;

use crate::ui::components::{ConnectionTabs, Loading};

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
            tabs.add_config(config);
            cx.notify();
        });
    }
}

impl Global for AppConnectionTabsState {}
