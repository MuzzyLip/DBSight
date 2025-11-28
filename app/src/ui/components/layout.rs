use gpui::{
    div, px, App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window,
};
use gpui_component::{
    sidebar::{
        Sidebar as SidebarComponents, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu,
        SidebarMenuItem, SidebarToggleButton,
    },
    Side, StyledExt, TitleBar,
};

pub struct TopBar {
    sidebar: Entity<SideBar>,
    collapsed: bool,
}

impl TopBar {
    pub fn new(sidebar: Entity<SideBar>) -> Self {
        Self {
            collapsed: false,
            sidebar,
        }
    }

    pub fn view(sidebar: Entity<SideBar>, _: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self::new(sidebar))
    }
}

impl Render for TopBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sidebar_entity = self.sidebar.clone();
        let topbar_entity = cx.entity();
        TitleBar::new().child(
            div()
                .w(if self.collapsed { px(100.) } else { px(245.) })
                .h_full()
                .h_flex()
                .justify_between()
                .items_center()
                .child("I'M LOGO")
                .child(
                    div().child(
                        SidebarToggleButton::left()
                            .on_click(move |_, _, app| {
                                app.update_entity(&topbar_entity, |topbar: &mut Self, cx| {
                                    topbar.collapsed = !topbar.collapsed;
                                    let new_val = topbar.collapsed;
                                    cx.update_entity(&sidebar_entity, |sidebar: &mut SideBar, _| {
                                        sidebar.collapsed = new_val;
                                    })
                                })
                            })
                            .collapsed(self.collapsed),
                    ),
                ),
        )
    }
}

pub struct SideBar {
    side: Side,
    pub(crate) collapsed: bool,
}

impl SideBar {
    pub fn new() -> Self {
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
            .width(px(if self.collapsed { 110. } else { 255. }))
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
