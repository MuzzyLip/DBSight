use db_sight_assets::icons::AppIconName;
use gpui::{
    div, prelude::FluentBuilder, px, App, AppContext, Context, CursorStyle, Entity, IntoElement,
    ParentElement, Render, Styled, Window,
};
use gpui_component::{
    button::Button, sidebar::SidebarToggleButton, Icon, Sizable, StyledExt, TitleBar,
};

use crate::{
    core::I18n,
    ui::{
        components::{
            connection_tabs::ConnectionTabs,
            dialog::create_connection_dialog::CreateConnectionDialog, SideBar,
        },
        state::AppConnectionTabsState,
    },
};

pub struct TopBar {
    sidebar: Entity<SideBar>,
    collapsed: bool,
    connection_tabs: Entity<ConnectionTabs>,
}

impl TopBar {
    pub fn new(sidebar: Entity<SideBar>, _: &mut Window, cx: &mut App) -> Self {
        let connection_tabs = cx
            .global::<AppConnectionTabsState>()
            .connection_tabs
            .clone();
        Self {
            collapsed: false,
            sidebar,
            connection_tabs,
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
                        div().cursor(CursorStyle::PointingHand).child(
                            SidebarToggleButton::left()
                                .on_click(move |_, _, app| {
                                    app.update_entity(&topbar_entity, |topbar: &mut Self, cx| {
                                        topbar.collapsed = !topbar.collapsed;
                                        let new_val = topbar.collapsed;
                                        cx.update_entity(
                                            &sidebar_entity,
                                            |sidebar: &mut SideBar, cx| {
                                                sidebar.collapsed = new_val;
                                                cx.notify();
                                            },
                                        )
                                    })
                                })
                                .collapsed(self.collapsed),
                        ),
                    ),
            )
            .child(self.connection_tabs.clone())
            .child(
                Button::new("db-connection")
                    .cursor_pointer()
                    .mr_2()
                    .ml_2()
                    .gap_1p5()
                    .small()
                    .label(i18n.t("new-connection"))
                    .icon(Icon::new(AppIconName::IconConnection))
                    .on_click(move |_, window, cx| {
                        CreateConnectionDialog::open(window, cx);
                    }),
            )
            .when(is_mac, |this| this.child(div().mr_4().child("I'M LOGO")))
    }
}
