use gpui::{AppContext, Application};
use gpui_component::Root;

use crate::ui::windows::{DefaultWindowOptions, Layout, WindowsName};

mod ui;

fn main() {
    let app = Application::new();

    app.run(move |cx| {
        // Init GPUI Components
        gpui_component::init(cx);
        let option = DefaultWindowOptions::build(WindowsName::Main, cx);

        cx.spawn(async move |cx| {
            cx.open_window(option, |window, cx| {
                cx.new(|cx| Root::new(Layout::view(window, cx), window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
