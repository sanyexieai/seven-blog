use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("认证错误: {0}")]
    AuthError(String),
    #[error("数据库错误: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("JWT错误: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("密码错误")]
    PasswordError,
    #[error("用户不存在")]
    UserNotFound,
    #[error("用户已存在")]
    UserExists,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl<'r> rocket::response::Responder<'r, 'static> for AppError {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let (status, error_message) = match self {
            AppError::AuthError(msg) => (Status::Unauthorized, msg),
            AppError::DatabaseError(_) => (Status::InternalServerError, "数据库错误".to_string()),
            AppError::JwtError(_) => (Status::Unauthorized, "JWT错误".to_string()),
            AppError::PasswordError => (Status::Unauthorized, "密码错误".to_string()),
            AppError::UserNotFound => (Status::NotFound, "用户不存在".to_string()),
            AppError::UserExists => (Status::Conflict, "用户已存在".to_string()),
        };

        let response = Json(ErrorResponse {
            error: error_message,
        }).respond_to(request)?;
        
        Ok(response)
    }
} 