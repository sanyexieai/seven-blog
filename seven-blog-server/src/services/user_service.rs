use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait, QueryFilter, ColumnTrait};
use crate::{
    error::AppError,
    models::user::{CreateUserDto, LoginDto, TokenResponse},
    entity::user,
    entity::user_info,
    utils::jwt::create_token,
};
use chrono;

pub struct UserService {
    db: DatabaseConnection,
    jwt_secret: String,
}

impl UserService {
    pub fn new(db: DatabaseConnection, jwt_secret: String) -> Self {
        Self { db, jwt_secret }
    }

    pub async fn register(&self, dto: CreateUserDto) -> Result<String, AppError> {
        // 检查用户是否已存在
        let exists = user::Entity::find()
            .filter(user::Column::Username.eq(&dto.username))
            .one(&self.db)
            .await?
            .is_some();

        if exists {
            return Err(AppError::UserExists);
        }

        // 创建用户信息
        let user_info = user_info::ActiveModel {
            nickname: Set(dto.name),
            avatar: Set("default_avatar.png".to_string()),
            intro: Set("这个人很懒，什么都没写...".to_string()),
            email: Set(Some(dto.email)),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        let user_info = user_info.insert(&self.db).await?;

        // 哈希密码
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(dto.password.as_bytes(), &salt)
            .map_err(|_| AppError::AuthError("密码哈希失败".to_string()))?
            .to_string();

        // 创建用户
        let user = user::ActiveModel {
            username: Set(dto.username),
            password_hash: Set(password_hash),
            user_info_id: Set(user_info.id),
            login_type: Set(0),
            is_disable: Set(false),
            is_super: Set(false),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        let user = user.insert(&self.db).await?;
        let user_id = user.id;

        // 生成 JWT token
        let token = create_token(user_id, &self.jwt_secret)?;
        Ok(token)
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<String, AppError> {
        // 查询用户
        let user = user::Entity::find()
            .filter(user::Column::Username.eq(username))
            .one(&self.db)
            .await?
            .ok_or(AppError::UserNotFound)?;

        // 验证密码
        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|_| AppError::AuthError("密码哈希解析失败".to_string()))?;
        
        if !Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
        {
            return Err(AppError::AuthError("密码错误".to_string()));
        }

        // 更新最后登录时间
        let user_id = user.id;
        let mut user: user::ActiveModel = user.into();
        user.last_login_time = Set(Some(chrono::Utc::now()));
        user.update(&self.db).await?;

        // 生成 JWT token
        let token = create_token(user_id, &self.jwt_secret)?;
        Ok(token)
    }
} 