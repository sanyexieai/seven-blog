use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use super::user::User;
use super::category::Category;
use super::tag::Tag;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub desc: Option<String>,
    pub content: String,
    pub img: Option<String>,
    pub article_type: i32,  // 1-原创 2-转载 3-翻译
    pub status: i32,        // 1-公开 2-私密 3-草稿
    pub is_top: bool,
    pub is_delete: bool,
    pub original_url: Option<String>,
    pub category_id: i32,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleVO {
    pub id: i32,
    pub title: String,
    pub desc: Option<String>,
    pub content: String,
    pub img: Option<String>,
    pub article_type: i32,
    pub status: i32,
    pub is_top: bool,
    pub is_delete: bool,
    pub original_url: Option<String>,
    pub category_id: i32,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub comment_count: i64,
    pub like_count: i64,
    pub view_count: i64,
    pub category: Option<Category>,
    pub tags: Vec<Tag>,
    pub user: Option<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateArticleDto {
    pub title: String,
    pub desc: Option<String>,
    pub content: String,
    pub img: Option<String>,
    pub article_type: i32,
    pub status: i32,
    pub is_top: bool,
    pub original_url: Option<String>,
    pub category_id: i32,
    pub tag_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateArticleDto {
    pub title: Option<String>,
    pub desc: Option<String>,
    pub content: Option<String>,
    pub img: Option<String>,
    pub article_type: Option<i32>,
    pub status: Option<i32>,
    pub is_top: Option<bool>,
    pub original_url: Option<String>,
    pub category_id: Option<i32>,
    pub tag_ids: Option<Vec<i32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticlePaginationVO {
    pub id: i32,
    pub img: Option<String>,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecommendArticleVO {
    pub id: i32,
    pub img: Option<String>,
    pub title: String,
    pub created_at: DateTime<Utc>,
} 