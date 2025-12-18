use async_trait::async_trait;
use sqlx::{
    decode::Decode,
    error::Error as SqlxError,
    mysql::{MySqlPoolOptions, MySqlValueRef},
    types::{chrono::NaiveDateTime, JsonValue},
    MySql, MySqlPool, Row, ValueRef,
};
use std::{borrow::Cow, time::Duration};

use crate::{
    driver::{DBError, DatabaseDriver},
    model::{
        schema::DBSchema,
        table::{TableColumn, TableDataPage, TableInfo},
    },
};

pub struct MySqlDriver {
    pub uri: String,
    pub pool: Option<MySqlPool>,
}

impl MySqlDriver {
    pub fn new(uri: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            pool: None,
        }
    }

    fn pool(&self) -> Result<&MySqlPool, DBError> {
        self.pool
            .as_ref()
            .ok_or(DBError::ConnectionError("Not connected".to_string()))
    }

    fn format_mysql_value(v: MySqlValueRef<'_>) -> String {
        if v.is_null() {
            return "NULL".to_string();
        }

        // Try to decode as date/time types first
        if let Ok(dt) = <NaiveDateTime as Decode<MySql>>::decode(v.clone()) {
            return dt.format("%Y-%m-%d %H:%M:%S").to_string();
        }

        // Try to decode as date type
        if let Ok(date) = <chrono::NaiveDate as Decode<MySql>>::decode(v.clone()) {
            return date.format("%Y-%m-%d").to_string();
        }

        // Try to decode as time type
        if let Ok(time) = <chrono::NaiveTime as Decode<MySql>>::decode(v.clone()) {
            return time.format("%H:%M:%S").to_string();
        }

        // Try integer types before string to avoid empty string issues
        if let Ok(n) = <i64 as Decode<MySql>>::decode(v.clone()) {
            return n.to_string();
        }
        if let Ok(n) = <i32 as Decode<MySql>>::decode(v.clone()) {
            return n.to_string();
        }
        if let Ok(n) = <u64 as Decode<MySql>>::decode(v.clone()) {
            return n.to_string();
        }
        if let Ok(n) = <u32 as Decode<MySql>>::decode(v.clone()) {
            return n.to_string();
        }

        // Try floating point types
        if let Ok(f) = <f64 as Decode<MySql>>::decode(v.clone()) {
            return f.to_string();
        }
        if let Ok(f) = <f32 as Decode<MySql>>::decode(v.clone()) {
            return f.to_string();
        }

        // Try boolean
        if let Ok(b) = <bool as Decode<MySql>>::decode(v.clone()) {
            return b.to_string();
        }

        // Try JSON
        if let Ok(j) = <JsonValue as Decode<MySql>>::decode(v.clone()) {
            return j.to_string();
        }

        // Try string (after numeric types to avoid empty string issues)
        if let Ok(s) = <String as Decode<MySql>>::decode(v.clone()) {
            // Only return string if it's not empty, otherwise try bytes
            if !s.is_empty() {
                return s;
            }
        }

        // Try bytes as fallback
        if let Ok(bytes) = <Vec<u8> as Decode<MySql>>::decode(v.clone()) {
            if let Ok(s) = std::str::from_utf8(&bytes) {
                if !s.is_empty() {
                    return s.to_string();
                }
            }
            return "<binary>".to_string();
        }

        "<unsupported>".to_string()
    }

    fn is_auth_error(e: &SqlxError) -> bool {
        if let SqlxError::Database(db_err) = e {
            if let Some(code) = db_err.code() {
                return code == Cow::Borrowed("28000")
                    && db_err.message().contains("Access denied");
            }
        }
        false
    }

    fn is_timeout_error(e: &SqlxError) -> bool {
        if let SqlxError::PoolTimedOut = e {
            return true;
        }
        false
    }

    fn build_pool_options() -> MySqlPoolOptions {
        MySqlPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(10))
    }
}

#[async_trait]
impl DatabaseDriver for MySqlDriver {
    fn name(&self) -> &'static str {
        "MySQL"
    }

    async fn connect(&mut self) -> Result<(), DBError> {
        match Self::build_pool_options().connect(&self.uri).await {
            Ok(pool) => {
                self.pool = Some(pool);
                return Ok(());
            }
            Err(e) => {
                if Self::is_auth_error(&e) {
                    return Err(DBError::AuthFailedError);
                } else if Self::is_timeout_error(&e) {
                    return Err(DBError::ConnectionTimeout);
                } else {
                    return Err(DBError::ConnectionError(format!("{:?}", e)));
                }
            }
        }
    }

    async fn test_connection(&self) -> Result<(), DBError> {
        match Self::build_pool_options().connect(&self.uri).await {
            Ok(pool) => {
                sqlx::query("SELECT 1").fetch_one(&pool).await?;
                return Ok(());
            }
            Err(e) => {
                eprintln!("Test Connection Failed, Reason: {:?}", e);
                if Self::is_auth_error(&e) {
                    return Err(DBError::AuthFailedError);
                } else if Self::is_timeout_error(&e) {
                    return Err(DBError::ConnectionTimeout);
                } else {
                    return Err(DBError::ConnectionError(format!("{:?}", e)));
                }
            }
        }
    }

    async fn list_schemas(&self) -> Result<Vec<DBSchema>, DBError> {
        let rows = sqlx::query("SHOW DATABASES")
            .fetch_all(self.pool()?)
            .await?;

        Ok(rows
            .into_iter()
            .map(|row| {
                // Handle both VARCHAR and VARBINARY types
                let name: String = match row.try_get::<String, _>(0) {
                    Ok(s) => s,
                    Err(_) => {
                        // Fallback: try to decode as bytes
                        let bytes: Vec<u8> = row.get(0);
                        String::from_utf8_lossy(&bytes).to_string()
                    }
                };
                DBSchema { name }
            })
            .collect())
    }

    async fn list_tables(&self, schema: &str) -> Result<Vec<TableInfo>, DBError> {
        let sql = format!("SHOW FULL TABLES FROM `{}`", schema);

        let rows = sqlx::query(&sql).fetch_all(self.pool()?).await?;

        let tables = rows
            .into_iter()
            .map(|row| {
                // Handle both VARCHAR and VARBINARY types for table name
                let name: String = match row.try_get::<String, _>(0) {
                    Ok(s) => s,
                    Err(_) => {
                        let bytes: Vec<u8> = row.get(0);
                        String::from_utf8_lossy(&bytes).to_string()
                    }
                };
                // Some MySQL variants may return this column as BINARY instead of VARCHAR,
                // so we fall back to decoding bytes and converting to String.
                let table_type: String = match row.try_get::<String, _>(1) {
                    Ok(s) => s,
                    Err(_) => {
                        let bytes: Vec<u8> = row.try_get::<Vec<u8>, _>(1).unwrap_or_default();
                        String::from_utf8_lossy(&bytes).into_owned()
                    }
                };
                TableInfo { name, table_type }
            })
            .collect();

        Ok(tables)
    }

    async fn get_table_columns(
        &self,
        schema: &str,
        table: &str,
    ) -> Result<Vec<TableColumn>, DBError> {
        let rows = sqlx::query(
            r#"
            SELECT 
                CAST(COLUMN_NAME AS CHAR(255)) AS COLUMN_NAME, 
                CAST(COLUMN_TYPE AS CHAR(255)) AS COLUMN_TYPE, 
                CAST(IS_NULLABLE AS CHAR(10)) AS IS_NULLABLE, 
                COLUMN_DEFAULT
            FROM INFORMATION_SCHEMA.COLUMNS
            WHERE TABLE_SCHEMA = ? AND TABLE_NAME = ?
            ORDER BY ORDINAL_POSITION
            "#,
        )
        .bind(schema)
        .bind(table)
        .fetch_all(self.pool()?)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| TableColumn {
                name: row.get("COLUMN_NAME"),
                data_type: row.get("COLUMN_TYPE"),
                nullable: row.get::<String, _>("IS_NULLABLE") == "YES",
                default: row.try_get("COLUMN_DEFAULT").ok(),
            })
            .collect())
    }

    async fn fetch_table_data(
        &self,
        schema: &str,
        table: &str,
        offset: u64,
        limit: u64,
    ) -> Result<TableDataPage, DBError> {
        // 1. Get columns
        let columns = self.get_table_columns(schema, table).await?;
        if columns.is_empty() {
            return Ok(TableDataPage {
                columns: vec![],
                rows: vec![],
                total: 0,
            });
        }

        let col_names: Vec<String> = columns.iter().map(|c| c.name.clone()).collect();
        let quoted_cols = col_names
            .iter()
            .map(|c| format!("`{}`", c))
            .collect::<Vec<_>>()
            .join(",");

        let full_table = format!("`{}`.`{}`", schema, table);

        // 2. Fetch rows
        let sql = format!(
            "SELECT {} FROM {} LIMIT {} OFFSET {}",
            quoted_cols, full_table, limit, offset
        );

        let rows = sqlx::query(&sql).fetch_all(self.pool()?).await?;

        let mut parsed_rows = Vec::new();

        for row in rows {
            let mut r = Vec::new();
            for col in &col_names {
                let val = row.try_get_raw(col.as_str());

                let s = match val {
                    Ok(v) => Self::format_mysql_value(v),
                    Err(_) => "<err>".to_string(),
                };

                r.push(s);
            }
            parsed_rows.push(r);
        }

        // 3. Count total
        let total_sql = format!("SELECT COUNT(*) AS cnt FROM {}", full_table);
        let total: i64 = sqlx::query_scalar(&total_sql)
            .fetch_one(self.pool()?)
            .await?;

        Ok(TableDataPage {
            columns: col_names,
            rows: parsed_rows,
            total: total as u64,
        })
    }
}
