use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub user_info_id: i32,
    pub login_type: i32,
    pub ip_address: Option<String>,
    pub last_login_time: Option<DateTime<Utc>>,
    pub is_disable: bool,
    pub is_super: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user_info::Entity",
        from = "Column::UserInfoId",
        to = "super::user_info::Column::Id"
    )]
    UserInfo,
    #[sea_orm(has_many = "super::article::Entity")]
    Article,
    #[sea_orm(has_many = "super::comment::Entity")]
    Comment,
}

impl Related<super::user_info::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserInfo.def()
    }
}

impl Related<super::article::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Article.def()
    }
}

impl Related<super::comment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {} 