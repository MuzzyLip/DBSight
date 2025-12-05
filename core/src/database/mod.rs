use db_sight_assets::icons::AppIconName;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn to_icon(&self) -> AppIconName {
        match self {
            DatabaseType::Postgre => AppIconName::DBPostgre,
            DatabaseType::MySql => AppIconName::DBMySql,
            DatabaseType::Sqlite => AppIconName::DBSqlite,
            DatabaseType::MariaDB => AppIconName::DBMariaDB,
            DatabaseType::Oracle => AppIconName::DBOracle,
            DatabaseType::Redis => AppIconName::DBRedis,
            DatabaseType::MongoDB => AppIconName::DBMongoDB,
            DatabaseType::MicrosoftSQLServer => AppIconName::DBMicrosoftSQLServer,
        }
    }

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
}

impl Display for DatabaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
        .fmt(f)
    }
}

impl From<DatabaseType> for String {
    fn from(value: DatabaseType) -> Self {
        value.to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Endpoint {
    Tcp(String, u16),
    Unix(PathBuf),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
