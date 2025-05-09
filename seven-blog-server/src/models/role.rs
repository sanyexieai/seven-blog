use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserRole {
    pub user_id: i32,
    pub role_id: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoleDto {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoleDto {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignRoleDto {
    pub user_id: i32,
    pub role_ids: Vec<i32>,
} 