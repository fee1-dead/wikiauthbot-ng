//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "server_settings")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub server_id: u64,
    pub welcome_channel_id: u64,
    pub auth_log_channel_id: u64,
    pub deauth_log_channel_id: u64,
    pub authenticated_role_id: u64,
    pub server_language: String,
    pub allow_banned_users: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::accounts::Entity")]
    Accounts,
}

impl Related<super::accounts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Accounts.def()
    }
}

impl Related<super::auth::Entity> for Entity {
    fn to() -> RelationDef {
        super::accounts::Relation::Auth.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::accounts::Relation::ServerSettings.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
