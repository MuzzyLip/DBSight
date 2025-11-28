use gpui::{AppContext, Application};
use gpui_component::Root;

use crate::{
    core::I18n,
    ui::windows::{init_themes, Assets, DefaultWindowOptions, RootApp, WindowsName},
};

mod core;
mod ui;

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        // Init GPUI Components
        gpui_component::init(cx);
        init_themes(cx);
        let option = DefaultWindowOptions::build(WindowsName::Main, cx);
        let i18n = I18n::new();
        cx.spawn(async move |cx| {
            cx.open_window(option, |window, cx| {
                cx.set_global(i18n);
                cx.new(|cx| Root::new(RootApp::view(window, cx), window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
