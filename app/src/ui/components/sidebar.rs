use db_sight_assets::icons::AppIconName;
use gpui::{
    div, prelude::FluentBuilder, px, Action, App, AppContext, Context, Div, Entity,
    InteractiveElement, IntoElement, ParentElement, Render, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, TextOverflow, Window,
};
use gpui_component::{
    h_flex,
    label::Label,
    sidebar::Sidebar as SidebarComponents,
    tab::{Tab, TabBar},
    tooltip::Tooltip,
    v_flex, Collapsible, Icon, Side, StyledExt, ThemeMode,
};

use uuid::Uuid;

use crate::{
    core::I18n,
    ui::{state::{AppConnectionTabsState, AppState}, windows::SwitchThemeMode},
};
use db_sight_core::{
    events::{ActiveConnectionsChanged, SelectedConnectionChanged},
    ConnectionConfig, DBManager,
};

pub struct SideBar {
    side: Side,
    active_theme_ix: usize,
    selected_connection_id: Option<Uuid>,
    active_connections: Vec<ConnectionConfig>,
}

impl SideBar {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let mut selected_connection_id = None;
        if cx.has_global::<DBManager>() {
            selected_connection_id = cx.global::<DBManager>().get_selected_connection();
        }

        let mut active_connections = Vec::new();
        if cx.has_global::<AppConnectionTabsState>() {
            let tabs = cx
                .global::<AppConnectionTabsState>()
                .connection_tabs
                .clone();

            active_connections = tabs.read(cx).active_configs().clone();

            cx.subscribe(
                &tabs,
                |this: &mut Self, _, event: &SelectedConnectionChanged, cx| {
                    this.selected_connection_id = event.id;
                    cx.notify();
                },
            )
            .detach();

            cx.subscribe(
                &tabs,
                |this: &mut Self, _, event: &ActiveConnectionsChanged, cx| {
                    this.active_connections = event.active_configs.clone();
                    cx.notify();
                },
            )
            .detach();
        }

        if selected_connection_id.is_none() && active_connections.len() == 1 {
            selected_connection_id = Some(active_connections[0].id);
        }

        Self {
            side: Side::Left,
            active_theme_ix: 1,
            selected_connection_id,
            active_connections,
        }
    }

    pub fn get_selected_connection_config(&self) -> Option<ConnectionConfig> {
        self.selected_connection_id
            .and_then(|id| self.active_connections.iter().find(|c| c.id == id))
            .cloned()
    }

    pub fn view(_: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(cx))
    }

    fn render_theme(&self, cx: &mut Context<Self>) -> Div {
        let i18n = cx.global::<I18n>();
        let collapsed = cx.global::<AppState>().collapsed;
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
                    .on_click(cx.listener(|this, ev, window, cx| {
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
                                .when(!collapsed, |this| this.child(i18n.t("theme-light"))),
                        ),
                    )
                    .child(
                        Tab::new().flex_1().h_flex().child(
                            div()
                                .h_flex()
                                .gap_2()
                                .child(Icon::new(AppIconName::IconDark))
                                .when(!collapsed, |this| this.child(i18n.t("theme-dark"))),
                        ),
                    ),
            )
    }
}

impl Render for SideBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let connection = self.get_selected_connection_config();
        let collapsed = cx.global::<AppState>().collapsed;

        let content = SidebarContent::new(connection);

        SidebarComponents::new(self.side)
            .width(px(if collapsed { 110. } else { 255. }))
            .child(content)
            .footer(self.render_theme(cx))
    }
}

#[derive(IntoElement, Debug, Clone)]
pub struct SidebarContent {
    collapsed: bool,
    connection: Option<ConnectionConfig>,
}

impl SidebarContent {
    pub fn new(connection: Option<ConnectionConfig>) -> Self {
        Self {
            collapsed: false,
            connection,
        }
    }
}

impl Collapsible for SidebarContent {
    fn is_collapsed(&self) -> bool {
        self.collapsed
    }

    fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }
}

impl RenderOnce for SidebarContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let collapsed = cx.global::<AppState>().collapsed;
        let _i18n = cx.global::<I18n>();
        let base = v_flex()
            .relative()
            .justify_center()
            .p_6()
            .gap_2()
            .w(px(if collapsed { 110. } else { 255. }));
        match self.connection {
            Some(connection) => {
                let name = connection.name;
                let full_name: SharedString = name.clone().into();
                let endpoint = connection.endpoint;
                let full_endpoint: SharedString = endpoint.clone().into();
                base.child(
                    // The header displays basic information about the current database.
                    // TODO: Right Context Menu To Show Connection Operation
                    h_flex()
                        .justify_between()
                        .when(collapsed, |this| this.justify_center())
                        .items_center()
                        .gap_3()
                        .cursor_pointer()
                        .child(connection.db_type.to_icon().img_view().size(px(40.)))
                        .when(!collapsed, |this| {
                            this.child(
                                v_flex()
                                    .overflow_x_hidden()
                                    .flex_1()
                                    .gap_0p5()
                                    .child(
                                        div()
                                            .id("connection-name")
                                            .child(name)
                                            .overflow_x_hidden()
                                            .text_overflow(TextOverflow::Truncate("...".into()))
                                            .tooltip(move |window, cx| {
                                                Tooltip::new(full_name.clone()).build(window, cx)
                                            }),
                                    )
                                    .child(
                                        div()
                                            .id("connection-endpoint")
                                            .child(Label::new(endpoint))
                                            .overflow_x_hidden()
                                            .text_overflow(TextOverflow::Truncate("...".into()))
                                            .tooltip(move |window, cx| {
                                                Tooltip::new(full_endpoint.clone())
                                                    .build(window, cx)
                                            }),
                                    ),
                            )
                        }),
                )
            }
            None => base,
        }
    }
}
