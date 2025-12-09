use db_sight_core::DBManager;
use gpui::{AppContext, Application};
use gpui_component::Root;

use crate::{
    core::I18n,
    ui::{
        state::{AppLoadingState, AppNotificationState},
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
        cx.spawn(async move |cx| {
            cx.open_window(option, |window, cx| {
                // Set Global State
                cx.set_global(i18n);
                cx.set_global(loading_state);
                cx.set_global(notification_state);
                cx.set_global(db_manager);
                cx.new(|cx| Root::new(RootApp::view(window, cx), window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
