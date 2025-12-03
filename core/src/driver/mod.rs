use async_trait::async_trait;
use sqlx::Error as SqlxError;
use thiserror::Error;

use crate::model::{
    schema::DBSchema,
    table::{TableColumn, TableDataPage, TableInfo},
};

mod mysql;

pub use mysql::MySqlDriver;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Database connection error: {0}")]
    ConnectionError(String),
    #[error("Database connection timeout")]
    ConnectionTimeout,
    #[error("Database auth failed")]
    AuthFailedError,
    #[error("Database query error: {0}")]
    QueryError(String),
    #[error("Database transaction error: {0}")]
    TransactionError(String),
    #[error("Sqlx error: {0}")]
    SqlxError(SqlxError),
}

impl From<SqlxError> for DBError {
    fn from(value: SqlxError) -> Self {
        DBError::SqlxError(value)
    }
}

#[async_trait]
pub trait DatabaseDriver: Send + Sync {
    /// Database Type. E.g. MySQL, PostgreSQL, SQLite, etc.
    fn name(&self) -> &'static str;

    /// Connect to database
    async fn connect(&mut self) -> Result<(), DBError>;

    /// Test connection
    async fn test_connection(&self) -> Result<(), DBError>;

    /// Get All Schemas
    async fn list_schemas(&self) -> Result<Vec<DBSchema>, DBError>;

    /// Get All Tables
    async fn list_tables(&self, schema: &str) -> Result<Vec<TableInfo>, DBError>;

    /// Get Table Columns
    async fn get_table_columns(
        &self,
        schema: &str,
        table: &str,
    ) -> Result<Vec<TableColumn>, DBError>;

    /// Get Table Data
    async fn fetch_table_data(
        &self,
        schema: &str,
        table: &str,
        offset: u64,
        limit: u64,
    ) -> Result<TableDataPage, DBError>;
}
