use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum PageRoutes {
    NoDatabase,
    ConnectDataBase,
    DatabaseColumns,
    DatabaseViews,
    DatabaseQueries,
}

impl Display for PageRoutes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PageRoutes::NoDatabase => write!(f, "No Database"),
            PageRoutes::ConnectDataBase => write!(f, "Connect Database"),
            PageRoutes::DatabaseColumns => write!(f, "Database Columns"),
            PageRoutes::DatabaseViews => write!(f, "Database Views"),
            PageRoutes::DatabaseQueries => write!(f, "Database Queries"),
        }
    }
}
