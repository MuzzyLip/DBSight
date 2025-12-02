use gpui::{px, size, Pixels, Size};

mod assets;
mod root;
mod themes;
mod window_option;

pub use assets::Assets;
pub use root::RootApp;
pub use themes::{init_themes, SwitchThemeMode};
pub use window_option::DefaultWindowOptions;

#[derive(Clone, Copy, Debug)]
pub enum WindowName {
    Main,
}

impl WindowName {
    pub fn size(&self) -> Size<Pixels> {
        match self {
            WindowName::Main => size(px(1280.0), px(720.0)),
        }
    }
}
