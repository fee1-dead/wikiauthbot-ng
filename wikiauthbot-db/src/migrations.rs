use sqlx::Executor;

use crate::DatabaseConnection;

impl DatabaseConnection {
    pub async fn do_init_sql(&self) -> color_eyre::Result<()> {
        self.sqlite.execute(include_str!("init.sql")).await?;
        Ok(())
    }
    pub async fn do_add_whois_ephemeral_sql(&self) -> color_eyre::Result<()> {
        self.sqlite
            .execute(include_str!("add_whois_ephemeral.sql"))
            .await?;
        Ok(())
    }
}
