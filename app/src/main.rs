use db_sight_core::DBManager;
use gpui::{AppContext, Application};
use gpui_component::Root;

use crate::{
    core::I18n,
    ui::{
        components::ConnectionTabs,
        state::{
            AppConnectionTabsState, AppLoadingState, AppNotificationState, AppState, AppTableState,
        },
        windows::{init_themes, Assets, DefaultWindowOptions, RootApp, WindowName},
    },
};

mod core;
mod ui;

#[tokio::main]
async fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        // Init GPUI Components
        gpui_component::init(cx);
        init_themes(cx);
        let option = DefaultWindowOptions::build(WindowName::Main, cx);
        let i18n = I18n::new();
        let loading_state = AppLoadingState::new(cx);
        let notification_state = AppNotificationState::new();
        let db_manager = DBManager::default();
        let connection_tabs = AppConnectionTabsState::new(ConnectionTabs::view(cx));
        let app_state = AppState::new();
        let table_state = AppTableState::new(cx);
        cx.spawn(async move |cx| {
            cx.open_window(option, |window, cx| {
                // Set Global State
                cx.set_global(i18n);
                cx.set_global(loading_state);
                cx.set_global(notification_state);
                cx.set_global(db_manager.clone());
                cx.set_global(connection_tabs.clone());
                cx.set_global(app_state.clone());
                cx.set_global(table_state);
                cx.new(|cx| Root::new(RootApp::view(window, cx), window, cx))
            })?;

            // Load configuration and restore active connections
            let _ = db_manager.load_config().await;
            let active_configs = db_manager.get_active_configs().await;

            if !active_configs.is_empty() {
                cx.update(|cx| {
                    cx.update_entity(&connection_tabs.connection_tabs, |tabs, cx| {
                        tabs.update_active_configs(active_configs, cx);
                        cx.notify();
                    });
                })?;
            }

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
