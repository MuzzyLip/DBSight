use db_sight_core::{events::SelectedTableChanged, DBManager, TableDataPage};
use gpui::{div, App, AppContext, Context, Entity, ParentElement, Render, Styled, Window};
use gpui_component::{
    table::{Table, TableState},
    v_flex, ActiveTheme, StyledExt,
};

use crate::{
    core::I18n,
    ui::{pages::tables::table_delegate::DatabaseTableDelegate, state::AppTableState},
};

pub struct PageTables {
    data: Option<TableDataPage>,
    loading: bool,
    current_table: Option<String>,
    table_state: Option<Entity<TableState<DatabaseTableDelegate>>>,
}

impl PageTables {
    fn new(cx: &mut Context<Self>) -> Self {
        let table_state = cx.global::<AppTableState>().state.clone();
        cx.subscribe(
            &table_state,
            |this: &mut Self, _, event: &SelectedTableChanged, cx| {
                this.load_table_data(event.table_name.clone(), cx);
            },
        )
        .detach();
        Self {
            data: None,
            loading: false,
            current_table: None,
            table_state: None,
        }
    }

    fn load_table_data(&mut self, table_name: String, cx: &mut Context<Self>) {
        self.loading = true;
        self.current_table = Some(table_name.clone());
        self.data = None;
        cx.notify();

        let db_manager = cx.global::<DBManager>().clone();
        let connection_id = db_manager.get_selected_connection();

        if let Some(conn_id) = connection_id {
            let conn_id_str = conn_id.to_string();
            let entity = cx.entity().clone();
            cx.spawn(async move |_, cx| {
                // Fetch driver asynchronously
                let driver = db_manager.get_connection(&conn_id_str).await;

                if let Some(driver) = driver {
                    // Find schema
                    let mut target_schema = None;
                    if let Ok(schemas) = driver.list_schemas().await {
                        for schema in schemas {
                            if let Ok(tables) = driver.list_tables(&schema.name).await {
                                if tables.iter().any(|t| t.name == table_name) {
                                    target_schema = Some(schema.name);
                                    break;
                                }
                            }
                        }
                    }

                    if let Some(schema) = target_schema {
                        match driver.fetch_table_data(&schema, &table_name, 0, 100).await {
                            Ok(page) => {
                                match cx.update_entity(&entity, |this, cx| {
                                    this.data = Some(page.clone());
                                    this.loading = false;

                                    // Update table state if it exists
                                    if let Some(table_state) = &this.table_state {
                                        table_state.update(cx, |table_state, cx| {
                                            table_state.delegate_mut().update_data(page.clone());
                                            table_state.delegate_mut().set_loading(false);
                                            cx.notify();
                                        });
                                    }

                                    cx.notify();
                                }) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        eprintln!("Failed to update entity: {:?}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to fetch table data: {}", e);
                                if let Err(update_err) = cx.update_entity(&entity, |this, cx| {
                                    this.loading = false;
                                    cx.notify();
                                }) {
                                    eprintln!(
                                        "Failed to update entity after error: {:?}",
                                        update_err
                                    );
                                }
                            }
                        }
                    } else {
                        eprintln!("Table {} not found in any schema", table_name);
                        if let Err(e) = cx.update_entity(&entity, |this, cx| {
                            this.loading = false;
                            cx.notify();
                        }) {
                            eprintln!("Failed to update entity: {:?}", e);
                        }
                    }
                } else {
                    if let Err(e) = cx.update_entity(&entity, |this, cx| {
                        this.loading = false;
                        cx.notify();
                    }) {
                        eprintln!("Failed to update entity: {:?}", e);
                    }
                }

                Ok::<_, anyhow::Error>(())
            })
            .detach();
        } else {
            self.loading = false;
            cx.notify();
        }
    }

    pub fn view(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(cx))
    }
}

impl Render for PageTables {
    fn render(
        &mut self,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let i18n = cx.global::<I18n>();
        v_flex()
            .size_full()
            .bg(gpui::white())
            .text_color(gpui::black())
            .child(if self.data.is_some() {
                let data = self.data.clone().unwrap();
                // Initialize table state if needed (only when we have data)
                if self.table_state.is_none() {
                    let delegate = DatabaseTableDelegate::new(data.clone());
                    let table_state = cx.new(|cx| TableState::new(delegate, window, cx));
                    self.table_state = Some(table_state);
                } else if let Some(table_state) = &self.table_state {
                    // Update loading state and data
                    table_state.update(cx, |table_state, cx| {
                        let delegate = table_state.delegate_mut();
                        delegate.set_loading(self.loading);
                        delegate.update_data(data.clone());
                        cx.notify();
                    });
                    cx.notify();
                }

                // Render table
                if let Some(table_state) = &self.table_state {
                    div()
                        .size_full()
                        .bg(cx.theme().background)
                        .child(Table::new(table_state).scrollbar_visible(true, true))
                } else {
                    div().size_full().bg(cx.theme().background).child(
                        v_flex()
                            .flex_1()
                            .items_center()
                            .justify_center()
                            .text_color(cx.theme().muted_foreground)
                            .child("Loading..."),
                    )
                }
            } else if self.loading {
                // Show loading state when no data but loading
                div().size_full().bg(cx.theme().background).child(
                    v_flex()
                        .flex_1()
                        .items_center()
                        .justify_center()
                        .text_color(cx.theme().muted_foreground)
                        .child("Loading..."),
                )
            } else {
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .bg(cx.theme().background)
                    .justify_center()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_4()
                            .child(
                                div()
                                    .text_xl()
                                    .font_semibold()
                                    .child(i18n.t("table.no-table-selected")),
                            )
                            .child(
                                div()
                                    .text_color(cx.theme().muted_foreground)
                                    .child(i18n.t("table.select-table-hint")),
                            ),
                    )
            })
    }
}
