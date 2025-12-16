#[derive(Debug, Clone)]
pub struct TableInfo {
    pub name: String,
    pub table_type: String,
}

pub struct TableColumn {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub default: Option<String>,
}

pub struct TableDataPage {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub total: u64,
}
