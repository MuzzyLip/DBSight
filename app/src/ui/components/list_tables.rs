use gpui::{
    div, App, InteractiveElement, ParentElement, SharedString, StatefulInteractiveElement, Styled,
    TextOverflow,
};
use gpui_component::{
    list::{ListDelegate, ListItem, ListState},
    tooltip::Tooltip,
    IndexPath,
};

pub struct ListTables {
    items: Vec<String>,
    selected_index: Option<IndexPath>,
}

impl ListTables {
    pub fn new(items: Vec<String>) -> Self {
        Self {
            items,
            selected_index: Some(IndexPath::new(0)),
        }
    }

    pub fn set_items(&mut self, items: Vec<String>) {
        self.items = items;
    }
}

impl ListDelegate for ListTables {
    type Item = ListItem;

    fn items_count(&self, _section: usize, _cx: &App) -> usize {
        self.items.len()
    }

    fn render_item(
        &self,
        ix: IndexPath,
        _window: &mut gpui::Window,
        _cx: &mut App,
    ) -> Option<Self::Item> {
        self.items.get(ix.row).map(|item| {
            let fullname = item.clone();
            let id = format!("table-name-{}", fullname.clone());
            let showname = fullname.clone();
            ListItem::new(ix)
                .child(
                    div()
                        .id(SharedString::from(id))
                        .child(showname)
                        .text_overflow(TextOverflow::Truncate("...".into()))
                        .tooltip(move |window, cx| {
                            Tooltip::new(fullname.clone()).build(window, cx)
                        }),
                )
                .selected(Some(ix) == self.selected_index)
        })
    }

    fn set_selected_index(
        &mut self,
        ix: Option<IndexPath>,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<ListState<Self>>,
    ) {
        self.selected_index = ix;
        cx.notify();
    }
}
