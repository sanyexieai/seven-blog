#[macro_use]
extern crate rocket;

#[derive(Debug)]
pub enum DbPool {
    Postgres(Pool<Postgres>),
    Sqlite(Pool<Sqlite>),
    MySql(Pool<MySql>),
}

pub mod config;
pub mod error;
pub mod handlers;
pub mod models;
pub mod services;
pub mod utils;

use config::{Config, DatabaseType};
use handlers::user_handler;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use services::user_service::UserService;
use sqlx::{Pool, Sqlite, Postgres, MySql};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        user_handler::register,
        user_handler::login,
    ),
    components(
        schemas(
            crate::models::user::CreateUserDto,
            crate::models::user::LoginDto,
            crate::models::user::TokenResponse,
        )
    ),
    tags(
        (name = "auth", description = "认证相关接口")
    )
)]
struct ApiDoc;

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    message: String,
}

#[get("/")]
fn index() -> Json<Message> {
    Json(Message {
        message: "Welcome to Seven Blog API".to_string(),
    })
}

#[launch]
async fn rocket() -> _ {
    let config = Config::from_env();
    
    // 创建数据库连接池
    let pool = match config.database_type {
        DatabaseType::Postgres => {
            DbPool::Postgres(
                sqlx::postgres::PgPoolOptions::new()
                    .max_connections(5)
                    .connect(&config.database_url)
                    .await
                    .expect("Failed to create PostgreSQL pool")
            )
        }
        DatabaseType::Sqlite => {
            // 确保 SQLite 数据库文件存在
            let db_path = config.database_url.replace("sqlite:", "");
            if !std::path::Path::new(&db_path).exists() {
                std::fs::File::create(&db_path).expect("Failed to create SQLite database file");
            }
            DbPool::Sqlite(
                sqlx::sqlite::SqlitePoolOptions::new()
                    .max_connections(5)
                    .connect(&config.database_url)
                    .await
                    .expect("Failed to create SQLite pool")
            )
        }
        DatabaseType::MySql => {
            DbPool::MySql(
                sqlx::mysql::MySqlPoolOptions::new()
                    .max_connections(5)
                    .connect(&config.database_url)
                    .await
                    .expect("Failed to create MySQL pool")
            )
        }
    };

    // 创建用户服务
    let user_service = UserService::new(pool, config.jwt_secret);

    // 创建 Rocket 实例
    let mut rocket = rocket::build()
        .mount("/api", routes![index])
        .mount("/api/auth", routes![user_handler::register, user_handler::login])
        .manage(user_service);

    // 只在开发环境添加 Swagger
    if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()) == "development" {
        rocket = rocket.mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>")
                .url("/api-docs/openapi.json", ApiDoc::openapi())
        );
    }

    rocket
}
