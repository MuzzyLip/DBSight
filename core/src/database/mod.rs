use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseType {
    Postgre,
    MySql,
    Sqlite,
    MariaDB,
    Oracle,
    Redis,
    MongoDB,
    MicrosoftSQLServer,
}

impl DatabaseType {
    pub fn to_string(&self) -> String {
        match self {
            DatabaseType::Postgre => "PostgreSQL",
            DatabaseType::MySql => "MySQL",
            DatabaseType::Sqlite => "SQLite",
            DatabaseType::MariaDB => "MariaDB",
            DatabaseType::Oracle => "Oracle",
            DatabaseType::Redis => "Redis",
            DatabaseType::MongoDB => "MongoDB",
            DatabaseType::MicrosoftSQLServer => "Microsoft SQL Server",
        }
        .to_string()
    }

    pub fn to_icon(&self) {}

    pub const ALL: [DatabaseType; 8] = [
        DatabaseType::Postgre,
        DatabaseType::MySql,
        DatabaseType::Sqlite,
        DatabaseType::MariaDB,
        DatabaseType::Oracle,
        DatabaseType::Redis,
        DatabaseType::MongoDB,
        DatabaseType::MicrosoftSQLServer,
    ];

    pub fn all() -> &'static [DatabaseType] {
        &Self::ALL
    }

    pub fn all_vec() -> Vec<DatabaseType> {
        Self::ALL.to_vec()
    }
}

#[derive(Debug, Clone)]
pub enum Endpoint {
    TCP(String, u16),
    Unix(PathBuf),
}

#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub id: uuid::Uuid,
    pub name: String,
    pub db_type: DatabaseType,
    pub endpoint: Endpoint,
    // Configuration
    pub remember_password: bool,
    // Auth
    pub username: String,
    pub saved_password_len: Option<u8>,
    // Using keyring crate to store password, Credentials are stored in the system keychain
}
