#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseType {
    Postgres,
    MySql,
    Sqlite,
}

#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub id: uuid::Uuid,
    pub name: String,
    pub db_type: DatabaseType,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub remember_password: bool,
    // Auth
    pub username: String,
    pub saved_password_len: Option<u8>,
    // Using keyring crate to store password, Credentials are stored in the system keychain
}
