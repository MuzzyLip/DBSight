use db_sight_core::DatabaseType;
use gpui::prelude::FluentBuilder;
use gpui::{
    px, size, App, AppContext, FocusHandle, Focusable, InteractiveElement, IntoElement,
    MouseButton, ParentElement, Pixels, Render, Size, Styled, TextOverflow, Window,
};
use gpui_component::button::Button;
use gpui_component::label::Label;
use gpui_component::{
    h_flex, v_flex, v_virtual_list, ActiveTheme, IndexPath, Selectable, VirtualListScrollHandle,
};
use std::rc::Rc;

pub struct DatabaseList {
    items: Vec<DatabaseType>,
    selected_index: Option<IndexPath>,
    focus_handle: FocusHandle,
    scroll_handle: VirtualListScrollHandle,
    item_sizes: Rc<Vec<Size<Pixels>>>,
    columns_count: usize,
    pub active_item: Option<DatabaseType>,
}

const ROW_HEIGHT: f32 = 96.;

impl DatabaseList {
    pub fn new(cx: &mut App) -> Self {
        let items = DatabaseType::all().to_vec();
        let first_item = DatabaseType::all().first().unwrap();
        let columns_count = 4;
        let rows = items.len().div_ceil(columns_count);
        let row_size = size(px(0.), px(ROW_HEIGHT + 16.));

        let item_sizes = Rc::new((0..rows).map(|_| row_size).collect::<Vec<_>>());
        Self {
            items,
            selected_index: Some(IndexPath::new(0)),
            focus_handle: cx.focus_handle(),
            scroll_handle: VirtualListScrollHandle::new(),
            item_sizes,
            columns_count,
            active_item: Some(*first_item),
        }
    }
}

impl Focusable for DatabaseList {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DatabaseList {
    fn render(&mut self, _: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let columns = self.columns_count;
        let total = self.items.len();
        let item_sizes = self.item_sizes.clone();
        let scroll_handle = self.scroll_handle.clone();
        let list_entity = cx.entity();

        v_flex().size_full().child(
            v_virtual_list(
                cx.entity().clone(),
                "database-list",
                item_sizes,
                move |this, range, _, cx| {
                    let list_entity = list_entity.clone();
                    range
                        .map(|row| {
                            h_flex().gap_4().children((0..columns).filter_map(|col| {
                                let index = row * columns + col;
                                if index >= total {
                                    return None;
                                }
                                let item = &this.items[index];
                                let ix = IndexPath::new(index);
                                let is_selected = this.selected_index == Some(ix);
                                let list_entity_btn = list_entity.clone();
                                let item_clone = *item;
                                Some(
                                    Button::new(index)
                                        .cursor_pointer()
                                        .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                                            cx.update_entity(&list_entity_btn, move |list, _| {
                                                list.selected_index = Some(ix);
                                                list.active_item = Some(item_clone);
                                            })
                                        })
                                        .w_32()
                                        .h_24()
                                        .rounded_xl()
                                        .px_2()
                                        .selected(is_selected)
                                        .when(is_selected, |this| {
                                            this.border_1().border_color(cx.theme().primary_active)
                                        })
                                        .child(
                                            v_flex()
                                                .gap_1()
                                                .items_center()
                                                .child(item.to_icon().img_view().size_10())
                                                .child(
                                                    Label::new(item.to_string())
                                                        .w_32()
                                                        .text_center()
                                                        .line_clamp(2)
                                                        .line_height(px(20.))
                                                        .text_overflow(TextOverflow::Truncate(
                                                            ".".into(),
                                                        )),
                                                ),
                                        ),
                                )
                            }))
                        })
                        .collect::<Vec<_>>()
                },
            )
            .h_72()
            .track_scroll(&scroll_handle)
            .p_4(),
        )
    }
}
