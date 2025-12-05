use gpui::{px, App, AppContext, Entity, ParentElement, Render, Styled, Window};
use gpui_component::{
    button::Button,
    input::{Input, InputState},
    label::Label,
    notification::Notification,
    v_flex, Sizable, WindowExt,
};

use crate::{
    core::I18n,
    ui::{
        components::Loading,
        state::{AppLoadingState, AppNotificationState},
    },
};
use db_sight_core::{DBError, DatabaseDriver, MySqlDriver};

pub struct CreateMySQLConnectionDialog {
    name: Entity<InputState>,
    host: Entity<InputState>,
    port: Entity<InputState>,
    username: Entity<InputState>,
    password: Entity<InputState>,
}

impl CreateMySQLConnectionDialog {
    pub fn db_name() -> &'static str {
        "MySQL"
    }
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        let placeholder = {
            let i18n = cx.global::<I18n>();
            i18n.t("connection.name-placeholder").to_string()
        };
        let name = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder));
        let placeholder = {
            let i18n = cx.global::<I18n>();
            format!(
                "{}{}",
                i18n.t("connection.please-enter"),
                i18n.t("connection.host")
            )
        };
        let host = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(placeholder)
                .default_value("127.0.0.1")
        });
        let placeholder = {
            let i18n = cx.global::<I18n>();
            format!(
                "{}{}",
                i18n.t("connection.please-enter"),
                i18n.t("connection.port")
            )
        };
        let port = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(placeholder)
                .default_value("3306")
        });
        let placeholder = {
            let i18n = cx.global::<I18n>();
            format!(
                "{}{}",
                i18n.t("connection.please-enter"),
                i18n.t("connection.username")
            )
        };
        let username = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder));
        let placeholder = {
            let i18n = cx.global::<I18n>();
            format!(
                "{}{}",
                i18n.t("connection.please-enter"),
                i18n.t("connection.password")
            )
        };
        let password = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder));
        Self {
            name,
            host,
            port,
            username,
            password,
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn open(window: &mut Window, cx: &mut App) {
        let dialog_entity = Self::view(window, cx);
        window.open_dialog(cx, move |dialog, _, cx| {
            let dialog_entity_clone = dialog_entity.clone();
            let i18n = cx.global::<I18n>();

            dialog
                .overlay_closable(false)
                .width(px(444.))
                .h(px(500.))
                .title(i18n.t_with(
                    "connection.create-new-connection",
                    &[("db", Self::db_name())],
                ))
                .child(dialog_entity_clone.clone())
                .footer(move |_, _, _, cx| {
                    let i18n = cx.global::<I18n>();
                    let dialog_entity_footer = dialog_entity_clone.clone();
                    let dialog_entity_footer_clone = dialog_entity_footer.clone();
                    vec![
                        Button::new("cancel-button")
                            .small()
                            .px_2()
                            .label(i18n.t("connection.cancel"))
                            .on_click(move |_, window, cx| window.close_dialog(cx)),
                        Button::new("test-button")
                            .px_2()
                            .small()
                            .label(i18n.t("connection.test-connection"))
                            .on_click(move |_, window, cx| {
                                cx.update_entity(&dialog_entity_footer.clone(), |this, cx| {
                                    let read_value = |entity: &Entity<InputState>| -> String {
                                        cx.read_entity(entity, |input, _| input.value().to_string())
                                    };
                                    let (timeout_message, auth_failed_message, connection_failed_message, connection_success, empty_input_message) = {
                                        let i18n = cx.global::<I18n>();
                                        (i18n.t("connection-error.connection-timeout"), i18n.t("connection-error.connection-auth-failed"), i18n.t("connection-error.connection-failed"), i18n.t("connection.connection-success"), i18n.t("connection-error.check-input"))
                                    };
                                    let host = read_value(&this.host);
                                    let port = read_value(&this.port);
                                    let username = read_value(&this.username);
                                    let password = read_value(&this.password);
                                    if host.is_empty() || port.is_empty() {
                                        window.push_notification(Notification::error(empty_input_message), cx);
                                        return;
                                    }

                                    let uri = format!("mysql://{}:{}@{}:{}", username, password, host, port);
                                    let app_state = cx.global_mut::<AppLoadingState>();
                                    let loading = app_state.loading.clone();
                                    Loading::open(&loading, cx);
                                    cx.spawn(async move |_, cx| {
                                        let driver = MySqlDriver::new(uri);

                                        let notification = match driver.test_connection().await {
                                            Ok(_) => Notification::success(connection_success),
                                            Err(e) => {
                                                println!("Connection Failed: {}", e);
                                                match e {
                                                    DBError::ConnectionTimeout => {
                                                        Notification::error(timeout_message)
                                                    },
                                                    DBError::AuthFailedError => {
                                                        Notification::error(auth_failed_message)
                                                    }
                                                    _ => {
                                                        Notification::error(connection_failed_message)
                                                    }
                                                }
                                            },
                                        };
                                        // Show notification & Hide loading
                                        cx.update(|app| app.global_mut::<AppNotificationState>().push(notification))?;
                                        cx.update(|app| Loading::hide(&loading, app))?;

                                        Ok::<_, anyhow::Error>(())
                                    }).detach();
                                })
                            }),
                        Button::new("save-button")
                            .small()
                            .px_2()
                            .label(i18n.t("connection.save-connection"))
                            .on_click(move |_, _, cx| {
                                cx.read_entity(&dialog_entity_footer_clone.clone(), |this, cx| {
                                    let read_value = |entity: &Entity<InputState>| -> String {
                                        cx.read_entity(entity, |input, _| input.value().to_string())
                                    };

                                    let name = read_value(&this.name);
                                    let host = read_value(&this.host);
                                    let port = read_value(&this.port);
                                    let username = read_value(&this.username);
                                    let password = read_value(&this.password);

                                    println!(
                                        "Save Connection => name: {name}, host: {host}, port: {port}, username: {username}, password: {password}"
                                    );
                                });
                            }),
                    ]
                })
        })
    }
}

impl Render for CreateMySQLConnectionDialog {
    fn render(&mut self, _: &mut Window, cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        let i18n = cx.global::<I18n>();
        v_flex()
            .gap_3()
            .child(
                v_flex()
                    .mt_3()
                    .gap_2()
                    .child(Label::new(i18n.t("connection.name")))
                    .child(Input::new(&self.name)),
            )
            .child(
                v_flex()
                    .gap_2()
                    .child(Label::new(i18n.t("connection.host")))
                    .child(Input::new(&self.host)),
            )
            .child(
                v_flex()
                    .gap_2()
                    .child(Label::new(i18n.t("connection.port")))
                    .child(Input::new(&self.port)),
            )
            .child(
                v_flex()
                    .gap_2()
                    .child(Label::new(i18n.t("connection.username")))
                    .child(Input::new(&self.username)),
            )
            .child(
                v_flex()
                    .gap_2()
                    .child(Label::new(i18n.t("connection.password")))
                    .child(Input::new(&self.password)),
            )
    }
}
