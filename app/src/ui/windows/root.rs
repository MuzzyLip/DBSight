use gpui::{
    div, App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window,
};
use gpui_component::{Root, StyledExt};

use crate::ui::{
    components::{SideBar, TopBar},
    pages::PageRoute,
};

pub struct RootApp {
    sidebar: Entity<SideBar>,
    topbar: Entity<TopBar>,
    current_page: PageRoute,
}

impl RootApp {
    fn new(window: &mut Window, cx: &mut App) -> Self {
        let sidebar = SideBar::view(window, cx);
        let topbar = TopBar::view(sidebar.clone(), window, cx);
        Self {
            sidebar,
            topbar,
            current_page: PageRoute::NoDatabase,
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for RootApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dialog_layer = Root::render_dialog_layer(window, cx);
        div()
            .v_flex()
            .size_full()
            .child(self.topbar.clone())
            .child(
                div().h_flex().flex_1().child(self.sidebar.clone()).child(
                    div()
                        .size_full()
                        .justify_center()
                        .items_center()
                        .child("Content"),
                ),
            )
            .children(dialog_layer)
    }
}
