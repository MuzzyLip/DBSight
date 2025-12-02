use db_sight_assets::icons::AppIconName;
use gpui::{
    div, prelude::FluentBuilder, px, App, AppContext, Context, Entity, IntoElement, ParentElement,
    Render, Styled, Window,
};
use gpui_component::{
    button::Button, sidebar::SidebarToggleButton, Icon, Sizable, StyledExt, TitleBar, WindowExt,
};

use crate::{
    core::I18n,
    ui::components::{create_connection_dialog::CreateConnectionDialog, SideBar},
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

    // Create an import from url dialog
    pub fn create_import_from_url_dialog(window: &mut Window, cx: &mut App) {
        window.open_dialog(cx, move |dialog, _, cx| {
            let i18n = cx.global::<I18n>();
            dialog
                .overlay_closable(false)
                .width(px(444.))
                .h(px(200.))
                .title(i18n.t("connection.connection-url"))
        });
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
                        CreateConnectionDialog::open(window, cx);
                    }),
            )
            .when(is_mac, |this| this.child(div().mr_4().child("I'M LOGO")))
    }
}
