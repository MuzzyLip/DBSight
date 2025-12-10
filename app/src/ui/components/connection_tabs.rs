use db_sight_assets::icons::AppIconName;
use db_sight_core::{
    events::{ActiveConnectionsChanged, SelectedConnectionChanged},
    ConnectionConfig, DBManager,
};
use gpui::{
    div, prelude::FluentBuilder, px, App, AppContext, Context, CursorStyle, Entity,
    InteractiveElement, IntoElement, ParentElement, Render, SharedString,
    StatefulInteractiveElement, Styled, TextOverflow, Window,
};
use gpui_component::{
    button::{Button, ButtonCustomVariant, ButtonVariants},
    h_flex, ActiveTheme, Icon, IconName, Selectable, Sizable,
};
use uuid::Uuid;

/// Maximum number of connection labels displayed
const MAX_VISIBLE_TABS: usize = 8;
/// Maximum Display Width for Label Name (pixels)
const TAB_NAME_MAX_WIDTH: f32 = 100.0;

/// Connect Tab Component
/// Display the currently active database connection in the TopBar
pub struct ConnectionTabs {
    /// Active Connection Configuration List
    active_configs: Vec<ConnectionConfig>,
    /// Currently selected connection ID
    selected_id: Option<Uuid>,
}

impl ConnectionTabs {
    pub fn new(_cx: &mut App) -> Self {
        Self {
            active_configs: Vec::new(),
            selected_id: None,
        }
    }

    pub fn view(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(cx))
    }

    pub fn update_active_configs(
        &mut self,
        configs: Vec<ConnectionConfig>,
        cx: &mut Context<Self>,
    ) {
        self.active_configs = configs;
        if let Some(id) = self.selected_id {
            if !self.active_configs.iter().any(|c| c.id == id) {
                self.selected_id = None;
                // Selected ID changed to None
                cx.emit(SelectedConnectionChanged { id: None });
                let db_manager = cx.global::<DBManager>();
                db_manager.set_selected_connection(None);
            }
        }
        if self.selected_id.is_none() && !self.active_configs.is_empty() {
            let new_id = self.active_configs[0].id;
            self.selected_id = Some(new_id);
            // Selected ID changed to Some
            cx.emit(SelectedConnectionChanged { id: Some(new_id) });
            let db_manager = cx.global::<DBManager>();
            db_manager.set_selected_connection(Some(new_id));
        }

        cx.emit(ActiveConnectionsChanged {
            active_configs: self.active_configs.clone(),
        });
    }

    pub fn set_selected(&mut self, id: Uuid, cx: &mut Context<Self>) {
        if self.active_configs.iter().any(|c| c.id == id) {
            self.selected_id = Some(id);
            let db_manager = cx.global::<DBManager>();
            db_manager.set_selected_connection(Some(id));
            cx.emit(SelectedConnectionChanged { id: Some(id) });
        }
    }

    pub fn selected_id(&self) -> Option<Uuid> {
        self.selected_id
    }

    pub fn active_configs(&self) -> &Vec<ConnectionConfig> {
        &self.active_configs
    }

    fn render_tab(
        &self,
        config: &ConnectionConfig,
        is_selected: bool,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let config_id = config.id;
        let name = config.name.clone();
        let full_name = name.clone();
        let entity = cx.entity().clone();
        let entity_for_close = entity.clone();

        let theme = cx.theme();
        let custom_variant = ButtonCustomVariant::new(cx)
            .active(theme.list_active)
            .border(if is_selected {
                theme.list_active_border
            } else {
                theme.primary_foreground
            })
            .hover(if is_selected {
                theme.primary
            } else {
                theme.accent
            });

        Button::new(SharedString::from(format!("button-{}", config_id)))
            .cursor(CursorStyle::PointingHand)
            .custom(custom_variant)
            .selected(is_selected)
            .small()
            .child(
                h_flex()
                    .w(px(TAB_NAME_MAX_WIDTH))
                    .child(
                        div()
                            .pr_2()
                            .overflow_hidden()
                            .text_overflow(TextOverflow::Truncate("...".into()))
                            .child(name),
                    )
                    .child(
                        h_flex()
                            .absolute()
                            .right_0()
                            .hover(|this| this.bg(cx.theme().border))
                            .id(SharedString::from(format!("close-{}", config_id)))
                            .child(Icon::new(IconName::Close))
                            .on_click(move |_, _, cx| {
                                cx.update_entity(&entity_for_close, |tabs: &mut Self, cx| {
                                    tabs.remove_tab(config_id, cx);
                                });
                            }),
                    ),
            )
            .on_click(move |_, _, cx| {
                cx.update_entity(&entity, |tabs: &mut Self, cx| {
                    tabs.set_selected(config_id, cx);
                })
            })
            .tooltip(full_name)
    }

    fn remove_tab(&mut self, config_id: Uuid, cx: &mut Context<Self>) {
        self.active_configs.retain(|c| c.id != config_id);
        if self.selected_id == Some(config_id) {
            self.selected_id = self.active_configs.first().map(|c| c.id);
            // Sync selection change if happened
            let db_manager = cx.global::<DBManager>();
            db_manager.set_selected_connection(self.selected_id);
            cx.emit(SelectedConnectionChanged {
                id: self.selected_id,
            });
        }
        cx.emit(ActiveConnectionsChanged {
            active_configs: self.active_configs.clone(),
        });

        let db_manager = cx.global::<DBManager>().clone();
        cx.background_executor()
            .spawn(async move {
                let _ = db_manager.remove_active_connection(&config_id).await;
            })
            .detach();
        cx.notify();
    }

    fn render_more_button(&self, _remaining_count: usize) -> impl IntoElement {
        // TODO: More connection Button
        Button::new("all-connection-list")
            .absolute()
            .right_0()
            .icon(AppIconName::IconDatabase)
            .cursor_pointer()
    }

    /// Add a new connection config and select it
    pub fn add_config(&mut self, config: ConnectionConfig, cx: &mut Context<Self>) {
        if let Some(pos) = self.active_configs.iter().position(|c| c.id == config.id) {
            self.active_configs[pos] = config.clone();
        } else {
            self.active_configs.push(config.clone());
        }
        self.set_selected(config.id, cx);
        cx.emit(ActiveConnectionsChanged {
            active_configs: self.active_configs.clone(),
        });
    }
}

impl gpui::EventEmitter<SelectedConnectionChanged> for ConnectionTabs {}
impl gpui::EventEmitter<ActiveConnectionsChanged> for ConnectionTabs {}

impl Render for ConnectionTabs {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let visible_configs: Vec<_> = self
            .active_configs
            .iter()
            .take(MAX_VISIBLE_TABS)
            .cloned()
            .collect();
        let remaining_count = self.active_configs.len().saturating_sub(MAX_VISIBLE_TABS);

        h_flex()
            .id("connection-tabs")
            .flex_1()
            .overflow_x_scroll()
            .relative()
            .gap_1()
            .px_2()
            .items_center()
            .children(visible_configs.iter().map(|config| {
                let is_selected = self.selected_id() == Some(config.id);
                self.render_tab(config, is_selected, window, cx)
            }))
            .child(self.render_more_button(remaining_count))
    }
}
