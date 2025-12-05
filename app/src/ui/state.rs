use gpui::{App, Entity, Global};
use gpui_component::notification::Notification;

use crate::ui::components::Loading;

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
