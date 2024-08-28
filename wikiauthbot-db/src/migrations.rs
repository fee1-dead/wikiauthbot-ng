use sqlx::{Executor, SqlitePool};

use crate::DatabaseConnection;

impl DatabaseConnection {
    pub async fn do_init_sql(pool: &SqlitePool) -> color_eyre::Result<()> {
        pool.execute(include_str!("init.sql")).await?;
        Ok(())
    }
    pub async fn do_add_whois_ephemeral_sql(pool: &SqlitePool) -> color_eyre::Result<()> {
        pool.execute(include_str!("add_whois_ephemeral.sql"))
            .await?;
        Ok(())
    }
}
