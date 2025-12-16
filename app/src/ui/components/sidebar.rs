use db_sight_assets::icons::AppIconName;
use gpui::{
    div, prelude::FluentBuilder, px, Action, App, AppContext, Context, Div, Entity,
    InteractiveElement, IntoElement, ParentElement, Render, SharedString,
    StatefulInteractiveElement, Styled, TextOverflow, Window,
};
use gpui_component::{
    h_flex,
    label::Label,
    list::{List, ListState},
    sidebar::Sidebar as SidebarComponents,
    tab::{Tab, TabBar},
    tooltip::Tooltip,
    v_flex, Collapsible, Icon, Side, StyledExt, ThemeMode,
};

use keyring::Entry;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    core::I18n,
    ui::{
        components::list_tables::ListTables,
        state::{AppConnectionTabsState, AppState},
        windows::SwitchThemeMode,
    },
};
use db_sight_core::{
    events::{ActiveConnectionsChanged, SelectedConnectionChanged},
    ConnectionConfig, DBManager, DatabaseDriver, MySqlDriver, TableInfo,
};

pub struct SideBar {
    side: Side,
    active_theme_ix: usize,
    selected_connection_id: Option<Uuid>,
    active_connections: Vec<ConnectionConfig>,
    content: Option<Entity<SidebarContent>>,
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
                    // Update SidebarContent when selection changes
                    if let Some(content_entity) = &this.content {
                        let new_connection = event.id.and_then(|id| {
                            this.active_connections.iter().find(|c| c.id == id).cloned()
                        });
                        cx.update_entity(content_entity, |content: &mut SidebarContent, cx| {
                            content.connection = new_connection.clone();
                            if let Some(conn) = &new_connection {
                                content.load_tables(conn.id, cx);
                            } else {
                                content.tables.clear();
                                content.sync_list_state(cx);
                            }
                        });
                    }
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

        // Entity must be created in App context, so delay until first render
        let content = None;

        Self {
            side: Side::Left,
            active_theme_ix: 1,
            selected_connection_id,
            active_connections,
            content,
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
        let collapsed = cx.global::<AppState>().collapsed;

        // Initialize content once during first render
        if self.content.is_none() {
            let connection = self.get_selected_connection_config();
            self.content = Some(cx.new(|cx| SidebarContent::new(cx, connection.clone())));
        }

        let content = self
            .content
            .clone()
            .expect("SidebarContent should be initialized");

        let content_wrapped = SidebarContentContainer(
            v_flex()
                .flex_1()
                .h_full()
                .min_h_128()
                .h_3_4()
                .child(SidebarContentEntity(content)),
        );

        SidebarComponents::new(self.side)
            .width(px(if collapsed { 110. } else { 255. }))
            .child(content_wrapped)
            .footer(self.render_theme(cx))
    }
}

pub struct SidebarContent {
    collapsed: bool,
    connection: Option<ConnectionConfig>,
    tables: Vec<TableInfo>,
    loading_tables: bool,
    selected_tab: usize,
    list_state: Option<Entity<ListState<ListTables>>>,
}

impl SidebarContent {
    pub fn new(cx: &mut Context<Self>, connection: Option<ConnectionConfig>) -> Self {
        let mut content = Self {
            collapsed: false,
            connection: connection.clone(),
            tables: Vec::new(),
            loading_tables: false,
            selected_tab: 0,
            list_state: None,
        };

        // If a connection exists, load table list asynchronously
        if let Some(conn) = &connection {
            content.load_tables(conn.id, cx);
        }

        // Subscribe to selection change events
        if cx.has_global::<AppConnectionTabsState>() {
            let tabs = cx
                .global::<AppConnectionTabsState>()
                .connection_tabs
                .clone();

            cx.subscribe(
                &tabs,
                |this: &mut Self, _, event: &SelectedConnectionChanged, cx| {
                    let new_connection = event.id.and_then(|id| {
                        if cx.has_global::<AppConnectionTabsState>() {
                            let tabs = cx
                                .global::<AppConnectionTabsState>()
                                .connection_tabs
                                .clone();
                            tabs.read(cx)
                                .active_configs()
                                .iter()
                                .find(|c| c.id == id)
                                .cloned()
                        } else {
                            None
                        }
                    });

                    this.connection = new_connection.clone();
                    if let Some(conn) = &new_connection {
                        this.load_tables(conn.id, cx);
                    } else {
                        this.tables.clear();
                    }
                    this.sync_list_state(cx);
                    cx.notify();
                },
            )
            .detach();
        }

        content
    }

    fn sync_list_state(&mut self, cx: &mut Context<Self>) {
        if let Some(list_state) = &self.list_state {
            let items: Vec<String> = self.tables.iter().map(|t| t.name.clone()).collect();
            list_state.update(cx, |state, cx| {
                state.delegate_mut().set_items(items);
                cx.notify();
            });
        }
    }

    fn load_tables(&mut self, connection_id: Uuid, cx: &mut Context<Self>) {
        if self.loading_tables {
            return;
        }

        self.loading_tables = true;
        self.tables.clear();
        cx.notify();

        let db_manager = cx.global::<DBManager>().clone();
        let connection_id_str = connection_id.to_string();
        let entity = cx.entity().clone();

        cx.spawn(async move |_, cx| {
            // Try to reuse existing driver; otherwise build and connect one.
            let mut driver = db_manager.get_connection(&connection_id_str).await;

            if driver.is_none() {
                // Fetch connection config to build URI
                let config_opt = db_manager.get_config_by_id(&connection_id).await;
                if let Some(config) = config_opt {
                    // Only MySQL is supported right now
                    if let db_sight_core::Endpoint::Tcp(host, port) = &config.endpoint {
                        // Pull password from keyring if saved
                        let password = if let Some(_) = config.saved_password_len {
                            match Entry::new("db-sight", &config.id.to_string()) {
                                Ok(entry) => entry.get_password().ok(),
                                Err(_) => None,
                            }
                        } else {
                            None
                        };

                        if let Some(pwd) = password {
                            let uri =
                                format!("mysql://{}:{}@{}:{}", config.username, pwd, host, port);
                            let mut mysql_driver = MySqlDriver::new(uri);
                            if mysql_driver.connect().await.is_ok() {
                                let arc = Arc::new(mysql_driver);
                                db_manager
                                    .add_connection(connection_id_str.clone(), arc.clone())
                                    .await;
                                driver = Some(arc);
                            } else {
                                eprintln!("Connect failed, will not fetch tables");
                            }
                        } else {
                            eprintln!("Password missing, please prompt user to input.");
                        }
                    } else {
                        eprintln!("Unsupported endpoint type for table listing");
                    }
                } else {
                    eprintln!("No config found for connection {}", connection_id_str);
                }
            }

            println!("Check Driver Instance {:?}", driver.is_some());
            if let Some(driver) = driver {
                // Fetch all schemas
                match driver.list_schemas().await {
                    Ok(schemas) => {
                        let mut all_tables = Vec::new();

                        // Traverse all schemas to fetch table list
                        for schema in schemas {
                            if matches!(
                                schema.name.to_lowercase().as_str(),
                                "information_schema" | "mysql" | "performance_schema" | "sys"
                            ) {
                                continue;
                            }

                            match driver.list_tables(&schema.name).await {
                                Ok(tables) => {
                                    all_tables.extend(tables);
                                }
                                Err(e) => {
                                    eprintln!(
                                        "Failed to list tables for schema {}: {}",
                                        schema.name, e
                                    );
                                }
                            }
                        }

                        // Update UI with loaded tables
                        cx.update_entity(&entity, |content: &mut Self, cx| {
                            content.tables = all_tables;
                            content.loading_tables = false;
                            content.sync_list_state(cx);
                        })?;
                    }
                    Err(e) => {
                        eprintln!("Failed to list schemas: {}", e);
                        cx.update_entity(&entity, |content: &mut Self, _| {
                            content.loading_tables = false;
                        })?;
                    }
                }
            } else {
                // Connection missing, clear tables
                println!("No connection found.");
                cx.update_entity(&entity, |content: &mut Self, cx| {
                    content.tables.clear();
                    content.loading_tables = false;
                    content.sync_list_state(cx);
                })?;
            }

            Ok::<_, anyhow::Error>(())
        })
        .detach();
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

// SidebarComponents::child requires a type implementing Collapsible.
// Wrap Entity<SidebarContent> with a local newtype to satisfy the trait bound.
#[derive(Clone)]
struct SidebarContentEntity(Entity<SidebarContent>);

impl Collapsible for SidebarContentEntity {
    fn is_collapsed(&self) -> bool {
        false
    }

    fn collapsed(self, _collapsed: bool) -> Self {
        self
    }
}

impl IntoElement for SidebarContentEntity {
    type Element = Entity<SidebarContent>;

    fn into_element(self) -> Self::Element {
        self.0.into_element()
    }
}

// Wrapper to satisfy Collapsible for layout containers passed into SidebarComponents
struct SidebarContentContainer(Div);

impl Collapsible for SidebarContentContainer {
    fn is_collapsed(&self) -> bool {
        false
    }

    fn collapsed(self, _collapsed: bool) -> Self {
        self
    }
}

impl IntoElement for SidebarContentContainer {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        self.0
    }
}

impl Render for SidebarContent {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let collapsed = cx.global::<AppState>().collapsed;
        let i18n = cx.global::<I18n>();
        let base = v_flex()
            .relative()
            .justify_start()
            .p_6()
            .gap_2()
            .flex_1()
            .h_full()
            .w(px(if collapsed { 110. } else { 255. }));

        match &self.connection {
            Some(connection) => {
                let name = connection.name.clone();
                let full_name: SharedString = name.clone().into();
                let endpoint = connection.endpoint.clone();
                let full_endpoint: SharedString = endpoint.clone().into();

                base.child(
                    // The header shows basic information about the current database.
                    // TODO: Right-click context menu for connection operations
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
                .when(!collapsed, |this| {
                    this.child(
                        TabBar::new("connection-operations-tab")
                            .h_flex()
                            .selected_index(self.selected_tab)
                            .underline()
                            .on_click(cx.listener(|this, ev, _, cx| {
                                this.selected_tab = *ev;
                                cx.notify();
                            }))
                            .child(Tab::new().flex_1().label(i18n.t("database.tables")))
                            .child(Tab::new().flex_1().label(i18n.t("database.views")))
                            .child(Tab::new().flex_1().label(i18n.t("database.queries"))),
                    )
                })
                .when(!collapsed && self.selected_tab == 0, |this| {
                    if self.list_state.is_none() {
                        let list_tables = ListTables::new(
                            self.tables.iter().map(|table| table.name.clone()).collect(),
                        );
                        self.list_state =
                            Some(cx.new(|cx| {
                                ListState::new(list_tables, window, cx).selectable(true)
                            }));
                    }
                    let table_state = self.list_state.clone().unwrap();

                    this.child(
                        v_flex()
                            .flex_1()
                            .h_full()
                            .child(List::new(&table_state).flex_1().h_full()),
                    )
                })
            }
            None => base,
        }
    }
}
