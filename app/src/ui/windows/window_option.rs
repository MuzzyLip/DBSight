use gpui::{App, Bounds, SharedString, WindowBounds, WindowOptions};
use gpui_component::TitleBar;

use super::WindowName;

pub struct DefaultWindowOptions {}

impl DefaultWindowOptions {
    pub fn build(window_name: WindowName, cx: &mut App) -> WindowOptions {
        let bounds = Bounds::centered(None, window_name.size(), cx);
        let mut titlebar_options = TitleBar::title_bar_options();
        titlebar_options.title = Some(SharedString::from("DBSight"));
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(titlebar_options),
            focus: true,
            show: true,
            is_movable: true,
            is_resizable: true,
            is_minimizable: true,
            window_min_size: Some(window_name.size()),
            ..WindowOptions::default()
        }
    }
}
