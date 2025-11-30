use gpui::{App, Window};
use gpui_component::list::{ListDelegate, ListItem};
use gpui_component::IndexPath;

pub struct DatabaseList {
    items: Vec<String>,
    selected_index: Option<IndexPath>,
}

impl ListDelegate for DatabaseList {
    type Item = ListItem;

    fn items_count(&self, _: usize, _: &App) -> usize {
        self.items.len()
    }

    fn render_item(&self, ix: IndexPath, window: &mut Window, cx: &mut App) -> Option<Self::Item> {
        self.items
            .get(ix.row)
            .map(|item| ListItem::new(ix).selected(Some(ix) == self.selected_index))
    }

    fn set_selected_index(
        &mut self,
        ix: Option<IndexPath>,
        window: &mut Window,
        cx: &mut gpui::Context<gpui_component::list::ListState<Self>>,
    ) {
        self.selected_index = ix;
        cx.notify();
    }
}
