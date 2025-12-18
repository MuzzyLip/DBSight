use db_sight_core::TableDataPage;
use gpui::{
    div, prelude::FluentBuilder, App, Context, InteractiveElement, IntoElement, ParentElement,
    SharedString, Styled, TextAlign, Window,
};
use gpui_component::{
    label::Label,
    table::{Column, TableDelegate, TableState},
    StyledExt,
};
use std::ops::Range;

pub struct DatabaseTableDelegate {
    data: TableDataPage,
    columns: Vec<Column>,
    loading: bool,
    visible_rows: Range<usize>,
    visible_cols: Range<usize>,
}

impl DatabaseTableDelegate {
    pub fn new(data: TableDataPage) -> Self {
        let columns: Vec<Column> = data
            .columns
            .iter()
            .enumerate()
            .map(|(idx, col_name)| {
                // First few columns can be fixed to left
                if idx < 3 {
                    Column::new(col_name.clone(), col_name.clone())
                        .width(120.)
                        .resizable(true)
                } else {
                    Column::new(col_name.clone(), col_name.clone())
                        .width(120.)
                        .resizable(true)
                }
            })
            .collect();

        Self {
            data,
            columns,
            loading: false,
            visible_rows: Range::default(),
            visible_cols: Range::default(),
        }
    }

    pub fn update_data(&mut self, data: TableDataPage) {
        let columns_length = data.columns.len();
        // Update columns if needed
        if self.columns.len() != columns_length {
            self.data = data.clone();
            self.columns = data
                .columns
                .iter()
                .enumerate()
                .map(|(idx, col_name)| {
                    if idx < 3 {
                        Column::new(col_name.clone(), col_name.clone())
                            .width(120.)
                            .resizable(true)
                    } else {
                        Column::new(col_name.clone(), col_name.clone())
                            .width(120.)
                            .resizable(true)
                    }
                })
                .collect();
        } else {
            self.data = data;
        }
    }

    pub fn set_loading(&mut self, loading: bool) {
        self.loading = loading;
    }
}

impl TableDelegate for DatabaseTableDelegate {
    fn columns_count(&self, _: &App) -> usize {
        let count = self.columns.len();
        count
    }

    fn rows_count(&self, _: &App) -> usize {
        let count = self.data.rows.len();
        count
    }

    fn column(&self, col_ix: usize, _cx: &App) -> &Column {
        &self.columns[col_ix]
    }

    fn render_th(&self, col_ix: usize, _: &mut Window, _cx: &mut App) -> impl IntoElement {
        let col = self.columns.get(col_ix).unwrap();
        let col_name = self
            .data
            .columns
            .get(col_ix)
            .map(|s| s.clone())
            .unwrap_or_else(|| col.name.to_string());

        div()
            .px_2()
            .py_1()
            .when(col.align == TextAlign::Right, |this| {
                this.h_flex().w_full().justify_end()
            })
            .child(Label::new(SharedString::from(col_name)))
    }

    fn render_td(
        &self,
        row_ix: usize,
        col_ix: usize,
        _: &mut Window,
        _cx: &mut App,
    ) -> impl IntoElement {
        let col = self.columns.get(col_ix).unwrap();
        let value = self
            .data
            .rows
            .get(row_ix)
            .and_then(|row| row.get(col_ix))
            .cloned()
            .unwrap_or_else(|| {
                eprintln!("Missing data for row {}, col {}", row_ix, col_ix);
                "<missing>".to_string()
            });

        div()
            .px_2()
            .py_1()
            .h_full()
            .when(col.align == TextAlign::Right, |this| {
                this.h_flex().justify_end()
            })
            .child(Label::new(SharedString::from(value)))
    }

    fn render_tr(
        &self,
        row_ix: usize,
        _window: &mut Window,
        _cx: &mut App,
    ) -> gpui::Stateful<gpui::Div> {
        div().id(SharedString::from(format!("row-{}", row_ix)))
    }

    fn loading(&self, _cx: &App) -> bool {
        self.loading
    }

    fn visible_rows_changed(
        &mut self,
        visible_range: Range<usize>,
        _: &mut Window,
        _: &mut Context<TableState<Self>>,
    ) {
        self.visible_rows = visible_range;
    }

    fn visible_columns_changed(
        &mut self,
        visible_range: Range<usize>,
        _: &mut Window,
        _: &mut Context<TableState<Self>>,
    ) {
        self.visible_cols = visible_range;
    }
}
