use db_sight_assets::icons::AppIconName;
use gpui::{
    div, prelude::FluentBuilder, px, Action, App, AppContext, Context, Entity, IntoElement,
    ParentElement, Render, Styled, Window,
};
use gpui_component::{
    button::Button,
    sidebar::{
        Sidebar as SidebarComponents, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem,
        SidebarToggleButton,
    },
    tab::{Tab, TabBar},
    Icon, Side, Sizable, StyledExt, ThemeMode, TitleBar, WindowExt,
};

use crate::{
    core::I18n,
    ui::{components::list_database::DatabaseList, windows::SwitchThemeMode},
};

pub struct TopBar {
    sidebar: Entity<SideBar>,
    collapsed: bool,
}

impl TopBar {
    pub fn new(sidebar: Entity<SideBar>, _: &mut Window, _: &mut App) -> Self {
        Self {
            collapsed: false,
            sidebar,
        }
    }

    pub fn view(sidebar: Entity<SideBar>, window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(sidebar, window, cx))
    }
}

impl Render for TopBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let i18n = cx.global::<I18n>();
        let sidebar_entity = self.sidebar.clone();
        let topbar_entity = cx.entity();
        let is_mac = cfg!(target_os = "macos");
        TitleBar::new()
            .child(
                div()
                    .when(!is_mac, |this| {
                        this.w(if self.collapsed { px(100.) } else { px(245.) })
                    })
                    .h_full()
                    .h_flex()
                    .justify_between()
                    .items_center()
                    .when(!is_mac, |this| this.child("I'M LOGO"))
                    .child(
                        div().cursor_pointer().child(
                            SidebarToggleButton::left()
                                .on_click(move |_, _, app| {
                                    app.update_entity(&topbar_entity, |topbar: &mut Self, cx| {
                                        topbar.collapsed = !topbar.collapsed;
                                        let new_val = topbar.collapsed;
                                        cx.update_entity(
                                            &sidebar_entity,
                                            |sidebar: &mut SideBar, _| {
                                                sidebar.collapsed = new_val;
                                            },
                                        )
                                    })
                                })
                                .collapsed(self.collapsed),
                        ),
                    ),
            )
            .child(div().flex_1())
            .child(
                Button::new("db-connection")
                    .cursor_pointer()
                    .mr_2()
                    .ml_2()
                    .gap_1p5()
                    .small()
                    .label(i18n.t("new-connection"))
                    .icon(Icon::new(AppIconName::IconDatabase))
                    .on_click(move |_, window, cx| {
                        let db_list = cx.new(|cx| DatabaseList::new(cx));
                        window.open_dialog(cx, move |dialog, _, cx| {
                            let i18n = cx.global::<I18n>();
                            dialog
                                .overlay_closable(false)
                                .width(px(644.))
                                .h(px(400.))
                                .title(i18n.t("connection.choose-database"))
                                .child(db_list.clone())
                        });
                    }),
            )
            .when(is_mac, |this| this.child(div().mr_4().child("I'M LOGO")))
    }
}

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
