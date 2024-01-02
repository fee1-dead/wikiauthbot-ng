use sea_orm_migration::prelude::*;

use crate::Symbols;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(Table::alter().table(Symbols::ServerSettings).add_column(ColumnDef::new(Symbols::ServerLanguage).string().not_null().default("en")).take()).await?;
        manager.alter_table(Table::alter().table(Symbols::ServerSettings).add_column(ColumnDef::new(Symbols::AllowBannedUsers).boolean().not_null().default(false)).take()).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(Table::alter().table(Symbols::ServerSettings).drop_column(Symbols::ServerLanguage).take()).await?;
        manager.alter_table(Table::alter().table(Symbols::ServerSettings).drop_column(Symbols::AllowBannedUsers).take()).await?;
        Ok(())
    }
}
