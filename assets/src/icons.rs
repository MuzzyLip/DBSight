use gpui::{img, Img};
use gpui_component::{Icon, IconNamed};

pub enum AppIconName {
    IconDatabase,
    IconLight,
    IconDark,
    IconChevronsUpDown,
    IconConnection,
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
            AppIconName::IconChevronsUpDown => "icons/icon-chevrons-up-down.svg",
            AppIconName::IconConnection => "icons/icon-connection.svg",
            AppIconName::DBMySql => "icons/db-mysql.svg",
            AppIconName::DBPostgre => "icons/db-postgre.svg",
            AppIconName::DBMariaDB => "icons/db-mariadb.svg",
            AppIconName::DBRedis => "icons/db-redis.svg",
            AppIconName::DBSqlite => "icons/db-sqlite.svg",
            AppIconName::DBOracle => "icons/db-oracle.svg",
            AppIconName::DBMongoDB => "icons/db-mongodb.svg",
            AppIconName::DBMicrosoftSQLServer => "icons/db-microsoft-sql-server.svg",
        }
        .into()
    }
}

impl AppIconName {
    pub fn icon_view(self) -> Icon {
        Icon::new(self)
    }

    pub fn img_view(self) -> Img {
        img(self.path())
    }
}
