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
                    .table(Symbols::ServerSettings)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Symbols::ServerId)
                            .big_unsigned()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Symbols::WelcomeChannelId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Symbols::AuthLogChannelId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Symbols::DeauthLogChannelId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Symbols::AuthenticatedRoleId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .take(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Symbols::Accounts)
                    .if_not_exists()
                    .col(ColumnDef::new(Symbols::DiscordId).big_unsigned().not_null())
                    .col(ColumnDef::new(Symbols::ServerId).big_unsigned().not_null())
                    .primary_key(
                        Index::create()
                            .col(Symbols::DiscordId)
                            .col(Symbols::ServerId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-accounts-discord_id")
                            .from(Symbols::Accounts, Symbols::DiscordId)
                            .to(Symbols::Auth, Symbols::DiscordId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-accounts-server_id")
                            .from(Symbols::Accounts, Symbols::ServerId)
                            .to(Symbols::ServerSettings, Symbols::ServerId),
                    )
                    .take(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Symbols::ServerSettings).take())
            .await?;
        manager
            .drop_table(Table::drop().table(Symbols::Accounts).take())
            .await
    }
}
