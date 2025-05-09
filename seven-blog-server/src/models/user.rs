use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use utoipa::ToSchema;
use sqlx::{FromRow, Row, sqlite::SqliteRow};

/// 用户信息
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub user_info_id: i32,
    pub login_type: i32,
    pub ip_address: Option<String>,
    pub ip_source: Option<String>,
    pub last_login_time: Option<DateTime<Utc>>,
    pub is_disable: bool,
    pub is_super: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, SqliteRow> for User {
    fn from_row(row: &'r SqliteRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.get("id"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            login_type: row.get("login_type"),
            ip_address: row.get("ip_address"),
            ip_source: row.get("ip_source"),
            last_login_time: row.get("last_login_time"),
            is_disable: row.get("is_disable"),
            is_super: row.get("is_super"),
            user_info_id: row.get("user_info_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}

/// 用户信息
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub email: Option<String>,
    pub nickname: String,
    pub avatar: String,
    pub intro: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, SqliteRow> for UserInfo {
    fn from_row(row: &'r SqliteRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.get("id"),
            email: row.get("email"),
            nickname: row.get("nickname"),
            avatar: row.get("avatar"),
            intro: row.get("intro"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}

/// 创建用户请求
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserDto {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
    /// 昵称
    pub name: String,
    /// 邮箱
    pub email: String,
}

/// 登录请求
#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginDto {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
}

/// 忘记密码请求
#[derive(Debug, Deserialize, ToSchema)]
pub struct ForgotPasswordDto {
    /// 用户名
    pub username: String,
}

/// 重置密码请求
#[derive(Debug, Deserialize, ToSchema)]
pub struct ResetPasswordDto {
    /// 重置令牌
    pub token: String,
    /// 新密码
    pub new_password: String,
}

/// 令牌响应
#[derive(Debug, Serialize, ToSchema)]
pub struct TokenResponse {
    /// JWT 令牌
    pub token: String,
}