use gpui_component::IconNamed;

pub enum AppIconName {
    IconDatabase,
    IconLight,
    IconDark,
    DBMySql,
    DBPostgre,
    DBMariaDB,
    DBRedis,
    DBSqlite,
    DBOracle,
    DBMongoDB,
    DBMicrosoftSQLServer,
}

impl IconNamed for AppIconName {
    fn path(self) -> gpui::SharedString {
        match self {
            AppIconName::IconDatabase => "icons/icon-database.svg",
            AppIconName::IconLight => "icons/icon-sun.svg",
            AppIconName::IconDark => "icons/icon-moon.svg",
            AppIconName::DBMySql => "icons/db-mysql.svg",
            AppIconName::DBPostgre => "icons/db-postgre.svg",
            AppIconName::DBMariaDB => "icons/db-maria.svg",
            AppIconName::DBRedis => "icons/db-redis.svg",
            AppIconName::DBSqlite => "icons/db-sqlite.svg",
            AppIconName::DBOracle => "icons/db-oracle.svg",
            AppIconName::DBMongoDB => "icons/db-mongodb.svg",
            AppIconName::DBMicrosoftSQLServer => "icons/db-microsoft-sql-server.svg",
        }
        .into()
    }
}
