use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagVO {
    pub id: i32,
    pub name: String,
    pub article_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTagDto {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTagDto {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ArticleTag {
    pub article_id: i32,
    pub tag_id: i32,
} 