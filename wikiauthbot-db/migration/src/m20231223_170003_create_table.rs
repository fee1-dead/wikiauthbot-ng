use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Post::Auth)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::DiscordId)
                            .big_unsigned()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Post::WikimediaId).unsigned().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Auth).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Auth,
    DiscordId,
    WikimediaId,
}
