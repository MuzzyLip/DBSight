use gpui::{
    div, App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window,
};
use gpui_component::{
    sidebar::{
        Sidebar as SidebarComponents, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu,
        SidebarMenuItem,
    },
    Side, StyledExt, TitleBar,
};

pub struct Layout {}

impl Layout {
    fn new() -> Self {
        Self {}
    }

    pub fn view(_: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self::new())
    }
}

impl Render for Layout {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .size_full()
            .child(TopBar::view(window, cx))
            .child(
                div().child(SideBar::view(window, cx)).child(
                    div()
                        .size_full()
                        .justify_center()
                        .items_center()
                        .child("Content"),
                ),
            )
    }
}

struct TopBar {}

impl TopBar {
    fn new() -> Self {
        Self {}
    }

    pub fn view(_: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self::new())
    }
}

impl Render for TopBar {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        TitleBar::new()
    }
}

struct SideBar {
    side: Side,
    collapsed: bool,
}

impl SideBar {
    fn new() -> Self {
        Self {
            side: Side::Left,
            collapsed: false,
        }
    }

    pub fn view(_: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self::new())
    }
}

impl Render for SideBar {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        SidebarComponents::new(self.side)
            .header(SidebarHeader::new().child("My Application"))
            .child(
                SidebarGroup::new("Navigation").child(
                    SidebarMenu::new()
                        .child(
                            SidebarMenuItem::new("Dashboard")
                                .on_click(|_, _, _| println!("Dashboard clicked")),
                        )
                        .child(
                            SidebarMenuItem::new("Settings")
                                .on_click(|_, _, _| println!("Settings clicked")),
                        ),
                ),
            )
            .footer(SidebarFooter::new().child("User Profile"))
    }
}
