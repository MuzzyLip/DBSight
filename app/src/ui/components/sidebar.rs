use db_sight_assets::icons::AppIconName;
use gpui::{
    div, prelude::FluentBuilder, px, Action, App, AppContext, Context, Entity, IntoElement,
    ParentElement, Render, Styled, Window,
};
use gpui_component::{
    sidebar::{
        Sidebar as SidebarComponents, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem,
    },
    tab::{Tab, TabBar},
    Icon, Side, StyledExt, ThemeMode,
};

use crate::{core::I18n, ui::windows::SwitchThemeMode};

pub struct SideBar {
    side: Side,
    pub(crate) collapsed: bool,
    active_theme_ix: usize,
}

impl SideBar {
    pub fn new() -> Self {
        Self {
            side: Side::Left,
            collapsed: false,
            active_theme_ix: 1,
        }
    }

    pub fn view(_: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self::new())
    }
}

impl Render for SideBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let i18n = cx.global::<I18n>();
        SidebarComponents::new(self.side)
            .width(px(if self.collapsed { 110. } else { 255. }))
            // The header displays basic information about the current database.
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
            .footer(
                div()
                    .h_flex()
                    .gap_2()
                    .py_2()
                    .w_full()
                    .justify_between()
                    .w_full()
                    .child(
                        TabBar::new("theme-tab")
                            .cursor_pointer()
                            .segmented()
                            .w_full()
                            .h_flex()
                            .on_click(cx.listener(|this, ev, window, cx: &mut Context<Self>| {
                                this.active_theme_ix = *ev;
                                let theme = if this.active_theme_ix == 0 {
                                    ThemeMode::Light
                                } else {
                                    ThemeMode::Dark
                                };
                                window.dispatch_action(SwitchThemeMode(theme).boxed_clone(), cx);
                            }))
                            .selected_index(self.active_theme_ix)
                            .child(
                                Tab::new().flex_1().h_flex().child(
                                    div()
                                        .h_flex()
                                        .gap_2()
                                        .child(Icon::new(AppIconName::IconLight))
                                        .when(!self.collapsed, |this| {
                                            this.child(i18n.t("theme-light"))
                                        }),
                                ),
                            )
                            .child(
                                Tab::new().flex_1().h_flex().child(
                                    div()
                                        .h_flex()
                                        .gap_2()
                                        .child(Icon::new(AppIconName::IconDark))
                                        .when(!self.collapsed, |this| {
                                            this.child(i18n.t("theme-dark"))
                                        }),
                                ),
                            ),
                    ),
            )
    }
}
