use gpui::{px, size, Pixels, Size};

mod layout;
mod window_option;

pub use layout::Layout;
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
