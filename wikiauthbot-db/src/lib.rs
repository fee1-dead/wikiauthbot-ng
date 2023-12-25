use sea_orm::{ActiveModelTrait, ActiveValue, DbErr, EntityTrait};
use wikiauthbot_db_entity::auth;
use wikiauthbot_db_entity::prelude::Auth;

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
}
