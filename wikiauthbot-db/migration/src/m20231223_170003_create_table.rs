use sea_orm_migration::prelude::*;

use crate::Symbols;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Symbols::Auth)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Symbols::DiscordId)
                            .big_unsigned()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Symbols::WikimediaId).unsigned().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Symbols::Auth).to_owned())
            .await
    }
}
