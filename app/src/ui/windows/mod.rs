use gpui::{px, size, Pixels, Size};

mod assets;
mod icons;
mod root;
mod themes;
mod window_option;

pub use assets::Assets;
pub use icons::AppIconName;
pub use root::RootApp;
pub use themes::{init_themes, SwitchThemeMode};
pub use window_option::DefaultWindowOptions;

#[derive(Clone, Copy, Debug)]
pub enum WindowsName {
    Main,
}

impl WindowsName {
    pub fn size(&self) -> Size<Pixels> {
        match self {
            WindowsName::Main => size(px(1280.0), px(720.0)),
        }
    }
}
