use gpui::{App, Entity, Global};

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
