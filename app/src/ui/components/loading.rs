use gpui::{
    div, rgba, App, AppContext, Entity, FocusHandle, InteractiveElement, ParentElement, Render,
    Styled, Window,
};
use gpui_component::{spinner::Spinner, IconName, Sizable, StyledExt};

pub struct Loading {
    active: bool,
    focus_handle: FocusHandle,
}

impl Loading {
    pub fn new(cx: &mut App) -> Self {
        let focus_handle = cx.focus_handle();
        Self {
            active: false,
            focus_handle,
        }
    }

    pub fn view(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(cx))
    }

    pub fn open(entity: &Entity<Self>, cx: &mut App) {
        let e = entity.clone();
        cx.update_entity(&e, |loading: &mut Self, _| loading.active = true)
    }

    pub fn hide(entity: &Entity<Self>, cx: &mut App) {
        let e = entity.clone();
        cx.update_entity(&e, |loading: &mut Self, _| loading.active = false)
    }
}

impl Render for Loading {
    fn render(
        &mut self,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        if !self.active {
            return div();
        }
        self.focus_handle.focus(window);
        cx.notify();

        div()
            .absolute()
            .occlude()
            .size_full()
            .inset_0()
            .bg(rgba(0x0000004d))
            .h_flex()
            .justify_center()
            .items_center()
            .child(Spinner::new().large().icon(IconName::LoaderCircle))
    }
}
