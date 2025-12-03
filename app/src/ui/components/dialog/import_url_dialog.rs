use gpui::{div, px, App, AppContext, Entity, ParentElement, Render, Styled, Window};
use gpui_component::{
    button::Button,
    input::{Input, InputState},
    Sizable, WindowExt,
};

use crate::core::I18n;

pub struct ImportUrlDialog {
    pub url_state: Entity<InputState>,
}

impl ImportUrlDialog {
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        let url_state = cx.new(|cx| {
            let placeholder = {
                let i18n = cx.global::<I18n>();
                i18n.t("connection.input-url").to_string()
            };
            InputState::new(window, cx)
                .placeholder(placeholder)
                .multi_line()
                .auto_grow(1, 4)
        });
        Self { url_state }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn open(window: &mut Window, cx: &mut App) {
        let dialog_entity = Self::view(window, cx);
        window.open_dialog(cx, move |dialog, _, cx| {
            let i18n = cx.global::<I18n>();

            dialog
                .overlay_closable(false)
                .width(px(444.))
                .h(px(230.))
                .title(i18n.t("connection.connection-url"))
                .child(dialog_entity.clone())
                .footer(move |_, _, _, cx| {
                    let i18n = cx.global::<I18n>();
                    vec![
                        Button::new("cancel-button")
                            .small()
                            .px_2()
                            .label(i18n.t("connection.cancel"))
                            .on_click(move |_, window, cx| window.close_dialog(cx)),
                        Button::new("import-button")
                            .small()
                            .px_2()
                            .label(i18n.t("connection.import"))
                            .on_click(move |_, window, cx| {
                                println!("Click Import");
                            }),
                    ]
                })
        });
    }
}

impl Render for ImportUrlDialog {
    fn render(&mut self, _: &mut Window, _: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        let url_state = self.url_state.clone();
        div().child(Input::new(&url_state).cleanable(true).h_24())
    }
}
