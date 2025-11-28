use gpui_component::IconNamed;

pub enum AppIconName {
    IconDatabase,
    IconLight,
    IconDark,
}

impl IconNamed for AppIconName {
    fn path(self) -> gpui::SharedString {
        match self {
            AppIconName::IconDatabase => "icons/icon-database.svg",
            AppIconName::IconLight => "icons/icon-sun.svg",
            AppIconName::IconDark => "icons/icon-moon.svg",
        }
        .into()
    }
}
