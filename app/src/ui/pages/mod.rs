pub mod tables;

use std::fmt::Display;

use gpui::{div, App, IntoElement, ParentElement, Styled};
use gpui_component::{ActiveTheme, StyledExt};

use crate::core::I18n;

#[derive(Clone, Copy, Debug, Default)]
pub enum PageRoute {
    #[default]
    NoDatabase,
    ConnectDataBase,
    DatabaseColumns,
    DatabaseViews,
    DatabaseQueries,
}

impl PageRoute {
    pub fn to_element(&self, cx: &mut App) -> impl IntoElement {
        match &self {
            Self::DatabaseColumns => {
                // This should not be called - RootApp should use its page_tables entity directly
                div().child("PageTables should be rendered via RootApp.page_tables").into_any_element()
            }
            Self::NoDatabase => {
                let i18n = cx.global::<I18n>();
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_4()
                            .child(
                                div()
                                    .text_2xl()
                                    .font_semibold()
                                    .child(i18n.t("no-connection.title")),
                            )
                            .child(
                                div()
                                    .text_color(cx.theme().muted_foreground)
                                    .child(i18n.t("no-connection.description")),
                            ),
                    )
                    .into_any_element()
            }
            _ => div().child("Content Coming Soon").into_any_element(),
        }
    }
}

impl Display for PageRoute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PageRoute::NoDatabase => write!(f, "No Database"),
            PageRoute::ConnectDataBase => write!(f, "Connect Database"),
            PageRoute::DatabaseColumns => write!(f, "Database Columns"),
            PageRoute::DatabaseViews => write!(f, "Database Views"),
            PageRoute::DatabaseQueries => write!(f, "Database Queries"),
        }
    }
}
