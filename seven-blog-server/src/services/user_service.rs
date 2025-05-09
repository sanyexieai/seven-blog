use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sqlx::{Pool, Sqlite, Postgres, MySql};
use crate::{
    error::AppError,
    models::user::{CreateUserDto, User, UserInfo},
    utils::jwt::create_token,
    DbPool,
};
use uuid;
use chrono;

pub struct UserService {
    pool: DbPool,
    jwt_secret: String,
}

impl UserService {
    pub fn new(pool: DbPool, jwt_secret: String) -> Self {
        Self { pool, jwt_secret }
    }

    pub async fn register(&self, dto: CreateUserDto) -> Result<String, AppError> {
        // 检查用户是否已存在
        let exists = false; // 暂时跳过用户存在检查
        /*
        let exists = match &self.pool {
            DbPool::Sqlite(pool) => {
                sqlx::query!(
                    "SELECT EXISTS(SELECT 1 FROM users WHERE username = ?) as exists",
                    dto.username
                )
                .fetch_one(pool)
                .await?
                .exists
                .unwrap_or(false)
            }
            DbPool::Postgres(pool) => {
                sqlx::query!(
                    "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1) as exists",
                    dto.username
                )
                .fetch_one(pool)
                .await?
                .exists
                .unwrap_or(false)
            }
            DbPool::MySql(pool) => {
                sqlx::query!(
                    "SELECT EXISTS(SELECT 1 FROM users WHERE username = ?) as exists",
                    dto.username
                )
                .fetch_one(pool)
                .await?
                .exists
                .unwrap_or(false)
            }
        };
        */

        if exists {
            return Err(AppError::UserExists);
        }

        // 创建用户信息
        let user_info = UserInfo {
            id: None,
            nickname: dto.name,
            avatar: "default_avatar.png".to_string(),
            intro: "这个人很懒，什么都没写...".to_string(),
            email: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // 哈希密码
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(dto.password.as_bytes(), &salt)
            .map_err(|_| AppError::AuthError("密码哈希失败".to_string()))?
            .to_string();

        // 创建用户
        let user = User {
            id: None,
            username: dto.username,
            password_hash,
            user_info_id: user_info.id.unwrap_or(0),
            login_type: 0,
            ip_address: None,
            ip_source: None,
            last_login_time: None,
            is_disable: false,
            is_super: false,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // TODO: 插入数据库获取 ID
        // 临时使用固定 ID 用于测试
        let user_id = 1;

        // 生成 JWT token
        let token = create_token(user_id, &self.jwt_secret)?;
        Ok(token)
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<String, AppError> {
        // 暂时返回错误
        Err(AppError::UserNotFound)
    }
} 