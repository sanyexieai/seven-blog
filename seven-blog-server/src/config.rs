use dotenv::dotenv;
use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("环境变量 {0} 未设置")]
    MissingEnvVar(String),
    #[error("无效的数据库类型: {0}")]
    InvalidDatabaseType(String),
}

#[derive(Debug, Clone)]
pub enum DatabaseType {
    Sqlite,
    Postgres,
    MySql,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_type: DatabaseType,
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();

        let database_type = match env::var("DATABASE_TYPE").unwrap_or_else(|_| "sqlite".to_string()).to_lowercase().as_str() {
            "sqlite" => DatabaseType::Sqlite,
            "postgres" => DatabaseType::Postgres,
            "mysql" => DatabaseType::MySql,
            _ => return Err(ConfigError::InvalidDatabaseType(env::var("DATABASE_TYPE").unwrap_or_default())),
        };

        let database_url = env::var("DATABASE_URL").map_err(|_| ConfigError::MissingEnvVar("DATABASE_URL".to_string()))?;
        let jwt_secret = env::var("JWT_SECRET").map_err(|_| ConfigError::MissingEnvVar("JWT_SECRET".to_string()))?;
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse()
            .expect("PORT must be a number");

        Ok(Config {
            database_type,
            database_url,
            jwt_secret,
            port,
        })
    }

    pub fn database_url(&self) -> String {
        match self.database_type {
            DatabaseType::Sqlite => {
                let db_path = self.database_url.replace("sqlite:", "");
                if !std::path::Path::new(&db_path).exists() {
                    std::fs::File::create(&db_path).expect("Failed to create SQLite database file");
                }
                self.database_url.clone()
            }
            _ => self.database_url.clone(),
        }
    }
} 