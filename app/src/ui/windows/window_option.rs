use gpui::{App, Bounds, SharedString, TitlebarOptions, WindowBounds, WindowOptions};

use super::WindowsName;

pub struct DefaultWindowOptions {}

impl DefaultWindowOptions {
    pub fn build(window_name: WindowsName, cx: &mut App) -> WindowOptions {
        let bounds = Bounds::centered(None, window_name.size(), cx);
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions {
                title: Some(SharedString::new("DBSight")),
                appears_transparent: true,
                traffic_light_position: None,
            }),
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
