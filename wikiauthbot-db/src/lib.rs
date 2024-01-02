use sea_orm::{ActiveModelTrait, ActiveValue, DbErr, EntityTrait, QuerySelect, QueryFilter, ColumnTrait};
use wikiauthbot_db_entity::{auth, server_settings, accounts};
use wikiauthbot_db_entity::prelude::{Auth, Accounts, ServerSettings};

pub struct Database {}

impl Database {
    pub async fn connect() -> Result<DatabaseConnection, DbErr> {
        Ok(DatabaseConnection {
            inner: sea_orm::Database::connect(
                dotenvy::var("DATABASE_URL").expect("expected DATABASE_URL to be set"),
            )
            .await?,
        })
    }
}

pub struct DatabaseConnection {
    inner: sea_orm::DatabaseConnection,
}

pub struct WhoisResult {
    pub wikimedia_id: u32,
}

impl DatabaseConnection {
    pub async fn find_user(&self, discord_id: u64) -> Result<Option<auth::Model>, DbErr> {
        Auth::find_by_id(discord_id).one(&self.inner).await
    }
    pub async fn add_auth_user(&self, discord_id: u64, wikimedia_id: u32) -> Result<(), DbErr> {
        wikiauthbot_db_entity::auth::ActiveModel {
            discord_id: ActiveValue::Set(discord_id),
            wikimedia_id: ActiveValue::Set(wikimedia_id),
        }
        .insert(&self.inner)
        .await?;
        Ok(())
    }

    pub async fn whois(&self, discord_id: u64, discord_server_id: u64) -> Result<Option<WhoisResult>, DbErr> {
        let res = Auth::find_by_id(discord_id).inner_join(Accounts).filter(accounts::Column::ServerId.eq(discord_server_id)).one(&self.inner).await?;
        Ok(res.map(|model| WhoisResult { wikimedia_id: model.wikimedia_id }))
    }
}
