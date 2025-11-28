use std::path::PathBuf;

use gpui::{Action, App, SharedString};
use gpui_component::{Theme, ThemeMode, ThemeRegistry};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AppState {
    theme: SharedString,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            theme: "Ayu Dark".into(),
        }
    }
}

pub fn init_themes(cx: &mut App) {
    // TODO: Cache To AppData
    if let Err(err) = ThemeRegistry::watch_dir(PathBuf::from("./themes"), cx, move |cx| {
        if let Some(theme) = ThemeRegistry::global(cx).themes().get("Ayu Dark").cloned() {
            Theme::global_mut(cx).apply_config(&theme);
        }
    }) {
        eprintln!("Failed to watch themes directory: {}", err);
    }
    cx.refresh_windows();
    cx.observe_global::<Theme>(|_cx| {
        // TODO: Save to AppData
    })
    .detach();
    cx.on_action(|switch: &SwitchThemeMode, cx| {
        let mode = switch.0;
        Theme::change(mode, None, cx);
        cx.refresh_windows();
    });
}

#[derive(Action, Clone, PartialEq)]
#[action(namespace = themes, no_json)]
pub(crate) struct SwitchThemeMode(pub(crate) ThemeMode);
