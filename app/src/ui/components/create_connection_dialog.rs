use gpui::{px, App, AppContext, Entity, ParentElement, Render, Styled, Window};
use gpui_component::{button::Button, Sizable, WindowExt};

use crate::{
    core::I18n,
    ui::components::{import_url_dialog::ImportUrlDialog, list_database::DatabaseList},
};

pub struct CreateConnectionDialog {
    pub db_list: Entity<DatabaseList>,
}

impl CreateConnectionDialog {
    pub fn new(cx: &mut App) -> Self {
        let db_list = cx.new(|cx| DatabaseList::new(cx));
        Self { db_list }
    }

    pub fn view(_: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(cx))
    }

    pub fn open(window: &mut Window, cx: &mut App) {
        let dialog_entity = Self::view(window, cx);
        window.open_dialog(cx, move |dialog, _, cx| {
            let i18n = cx.global::<I18n>();
            let dialog_entity_clone = dialog_entity.clone();

            dialog
                .overlay_closable(false)
                .width(px(644.))
                .h(px(360.))
                .title(i18n.t("connection.choose-database"))
                .child(dialog_entity_clone.clone())
                .footer(move |_, _, _, cx| {
                    let i18n = cx.global::<I18n>();
                    let dialog_entity_footer = dialog_entity_clone.clone();
                    vec![
                        Button::new("cancel-button")
                            .small()
                            .px_2()
                            .label(i18n.t("connection.cancel"))
                            .on_click(move |_, window, cx| window.close_dialog(cx)),
                        Button::new("import-button")
                            .px_2()
                            .small()
                            .label(i18n.t("connection.import-from-url"))
                            .on_click(move |_, window, cx| ImportUrlDialog::open(window, cx)),
                        Button::new("confirm-button")
                            .small()
                            .px_2()
                            .label(i18n.t("connection.create"))
                            .on_click(move |_, _, cx| {
                                cx.update_entity(&dialog_entity_footer, |this, cx| {
                                    cx.update_entity(&this.db_list, |db_list, _| {
                                        println!("Check active item {:?}", db_list.active_item);
                                    })
                                })
                            }),
                    ]
                })
        });
    }
}

impl Render for CreateConnectionDialog {
    fn render(&mut self, _: &mut Window, _: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        self.db_list.clone()
    }
}
