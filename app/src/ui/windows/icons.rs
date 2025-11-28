use gpui_component::IconNamed;

pub enum AppIconName {}

impl IconNamed for AppIconName {
    fn path(self) -> gpui::SharedString {
        "".into()
    }
}
