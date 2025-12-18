use gpui::{
    div, App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window,
};
use gpui_component::{Root, StyledExt, WindowExt};

use crate::ui::{
    components::{SideBar, TopBar},
    pages::tables::table::PageTables,
    state::{AppLoadingState, AppNotificationState, AppState},
};

pub struct RootApp {
    sidebar: Entity<SideBar>,
    topbar: Entity<TopBar>,
    page_tables: Entity<PageTables>,
}

impl RootApp {
    fn new(window: &mut Window, cx: &mut App) -> Self {
        let sidebar = SideBar::view(window, cx);
        let topbar = TopBar::view(sidebar.clone(), window, cx);
        let page_tables = PageTables::view(cx);
        Self {
            sidebar,
            topbar,
            page_tables,
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for RootApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dialog_layer = Root::render_dialog_layer(window, cx);
        let notification_layer = Root::render_notification_layer(window, cx);
        let loading = {
            let app_state = cx.global::<AppLoadingState>();
            app_state.loading.clone()
        };
        let current_page = cx.global::<AppState>().current_page;
        let notifications = cx.global_mut::<AppNotificationState>().take();
        for notification in notifications {
            window.push_notification(notification, cx);
        }

        div()
            .v_flex()
            .size_full()
            .child(self.topbar.clone())
            .child(
                div()
                    .h_flex()
                    .flex_1()
                    .child(self.sidebar.clone())
                    .child(match current_page {
                        crate::ui::pages::PageRoute::DatabaseColumns => {
                            self.page_tables.clone().into_any_element()
                        }
                        _ => current_page.to_element(cx).into_any_element(),
                    }),
            )
            .children(dialog_layer)
            .children(notification_layer)
            .child(loading)
    }
}
