use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub enum DatabaseType {
    Postgres,
    Sqlite,
    MySql,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub database_type: DatabaseType,
    pub jwt_secret: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database_type = match env::var("DATABASE_TYPE").unwrap_or_else(|_| "sqlite".to_string()).as_str() {
            "postgres" => DatabaseType::Postgres,
            "sqlite" => DatabaseType::Sqlite,
            "mysql" => DatabaseType::MySql,
            _ => panic!("Unsupported database type"),
        };
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse()
            .expect("PORT must be a number");

        Self {
            database_url,
            database_type,
            jwt_secret,
            port,
        }
    }
} 