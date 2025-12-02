use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum PageRoute {
    NoDatabase,
    ConnectDataBase,
    DatabaseColumns,
    DatabaseViews,
    DatabaseQueries,
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
