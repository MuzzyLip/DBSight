use gpui::{AppContext, Application};
use gpui_component::Root;

use crate::ui::windows::{init_themes, Assets, DefaultWindowOptions, RootApp, WindowsName};

mod core;
mod ui;

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        // Init GPUI Components
        gpui_component::init(cx);
        init_themes(cx);
        let option = DefaultWindowOptions::build(WindowsName::Main, cx);

        cx.spawn(async move |cx| {
            cx.open_window(option, |window, cx| {
                cx.new(|cx| Root::new(RootApp::view(window, cx), window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
