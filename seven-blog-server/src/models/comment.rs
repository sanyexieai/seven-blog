use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use super::user::User;
use super::post::Article;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: i32,
    pub user_id: i32,
    pub reply_user_id: Option<i32>,
    pub topic_id: i32,
    pub parent_id: i32,
    pub content: String,
    pub comment_type: i32,  // 1-文章 2-友链 3-说说
    pub is_review: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentVO {
    pub id: i32,
    pub user_id: i32,
    pub reply_user_id: Option<i32>,
    pub topic_id: i32,
    pub parent_id: i32,
    pub content: String,
    pub comment_type: i32,
    pub is_review: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub like_count: i32,
    pub reply_count: i32,
    pub reply_list: Vec<CommentVO>,
    pub user: Option<User>,
    pub reply_user: Option<User>,
    pub article: Option<Article>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommentDto {
    pub user_id: i32,
    pub topic_id: i32,
    pub content: String,
    pub comment_type: i32,
    pub is_review: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyCommentDto {
    pub user_id: i32,
    pub reply_user_id: i32,
    pub parent_id: i32,
    pub content: String,
    pub is_review: bool,
} 