// Unified Database Interface Layer
mod database;
mod db_config;
mod db_manager;
mod driver;
pub mod events;
mod model;

pub use database::{ConnectionConfig, DatabaseType, Endpoint};
pub use db_config::DBConfig;
pub use db_manager::DBManager;
pub use driver::{DBError, DatabaseDriver, MySqlDriver};
pub use model::table::TableInfo;
