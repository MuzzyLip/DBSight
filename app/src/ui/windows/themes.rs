use gpui::{Action, App};
use gpui_component::{Theme, ThemeMode, ThemeRegistry};
use std::path::PathBuf;

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
pub struct SwitchThemeMode(pub(crate) ThemeMode);
