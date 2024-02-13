use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DbBackend, DbErr, EntityTrait,
    QueryFilter, Schema,
};
use wikiauthbot_db_entity::prelude::{Accounts, Auth, ServerSettings};
use wikiauthbot_db_entity::{accounts, auth, server_settings};

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

    pub async fn test_connect() -> Result<DatabaseConnection, DbErr> {
        let db = sea_orm::Database::connect("sqlite::memory:").await?;
        let schema = Schema::new(DbBackend::Sqlite);
        let stmts = [
            schema.create_table_from_entity(Accounts),
            schema.create_table_from_entity(Auth),
            schema.create_table_from_entity(ServerSettings),
        ];
        for stmt in stmts {
            db.execute(db.get_database_backend().build(&stmt)).await?;
        }

        let conn = DatabaseConnection { inner: db };

        Ok(conn)
    }
}

pub struct DatabaseConnection {
    inner: sea_orm::DatabaseConnection,
}

pub struct WhoisResult {
    pub wikimedia_id: u32,
}

#[derive(Clone)]
pub struct ServerSettingsData {
    pub welcome_channel_id: u64,
    pub auth_log_channel_id: u64,
    pub deauth_log_channel_id: u64,
    pub authenticated_role_id: u64,
    pub server_language: String,
    pub allow_banned_users: bool,
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

    pub async fn whois(
        &self,
        discord_id: u64,
        discord_server_id: u64,
    ) -> Result<Option<WhoisResult>, DbErr> {
        let res = Auth::find_by_id(discord_id)
            .inner_join(Accounts)
            .filter(accounts::Column::ServerId.eq(discord_server_id))
            .one(&self.inner)
            .await?;
        Ok(res.map(
            |auth::Model {
                 discord_id: _,
                 wikimedia_id,
             }| WhoisResult { wikimedia_id },
        ))
    }

    pub async fn get_all_server_settings(
        &self,
    ) -> Result<impl Iterator<Item = (u64, ServerSettingsData)>, DbErr> {
        let models = ServerSettings::find().all(&self.inner).await?;
        Ok(models.into_iter().map(
            |server_settings::Model {
                 server_id,
                 welcome_channel_id,
                 auth_log_channel_id,
                 deauth_log_channel_id,
                 authenticated_role_id,
                 server_language,
                 allow_banned_users,
             }| {
                (
                    server_id,
                    ServerSettingsData {
                        welcome_channel_id,
                        auth_log_channel_id,
                        deauth_log_channel_id,
                        authenticated_role_id,
                        server_language,
                        allow_banned_users,
                    },
                )
            },
        ))
    }

    pub async fn get_server_settings(
        &self,
        discord_server_id: u64,
    ) -> Result<Option<ServerSettingsData>, DbErr> {
        let model = ServerSettings::find_by_id(discord_server_id)
            .one(&self.inner)
            .await?;
        Ok(model.map(
            |server_settings::Model {
                 server_id: _,
                 welcome_channel_id,
                 auth_log_channel_id,
                 deauth_log_channel_id,
                 authenticated_role_id,
                 server_language,
                 allow_banned_users,
             }| ServerSettingsData {
                welcome_channel_id,
                auth_log_channel_id,
                deauth_log_channel_id,
                authenticated_role_id,
                server_language,
                allow_banned_users,
            },
        ))
    }

    pub async fn set_server_settings(
        &self,
        discord_server_id: u64,
        ServerSettingsData {
            welcome_channel_id,
            auth_log_channel_id,
            deauth_log_channel_id,
            authenticated_role_id,
            server_language,
            allow_banned_users,
        }: ServerSettingsData,
    ) -> Result<bool, DbErr> {
        let model = ServerSettings::find_by_id(discord_server_id)
            .one(&self.inner)
            .await?;
        if model.is_some() {
            Ok(false)
        } else {
            server_settings::ActiveModel {
                server_id: ActiveValue::Set(discord_server_id),
                welcome_channel_id: ActiveValue::Set(welcome_channel_id),
                auth_log_channel_id: ActiveValue::Set(auth_log_channel_id),
                deauth_log_channel_id: ActiveValue::Set(deauth_log_channel_id),
                authenticated_role_id: ActiveValue::Set(authenticated_role_id),
                server_language: ActiveValue::Set(server_language),
                allow_banned_users: ActiveValue::Set(allow_banned_users),
            }
            .insert(&self.inner)
            .await?;
            Ok(true)
        }
    }
}
