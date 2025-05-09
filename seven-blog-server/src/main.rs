use rocket::{get, routes, Build, Rocket};
use sea_orm::{Database, DatabaseConnection};
use services::user_service::UserService;
use utoipa_swagger_ui::SwaggerUi;
use sea_orm_migration::MigratorTrait;
use utoipa::OpenApi;

mod api;
mod config;
mod entity;
mod error;
mod handlers;
mod models;
mod services;
mod migration;
mod utils;

use handlers::user_handler;


#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let config = config::Config::from_env()?;
    let db_url = config.database_url();
    println!("数据库连接 URL: {}", db_url);

    // 创建数据库连接
    let db = Database::connect(&db_url).await?;
    println!("数据库连接成功");

    // 运行数据库迁移
    println!("开始执行数据库迁移...");
    match migration::Migrator::up(&db, None).await {
        Ok(_) => println!("数据库迁移成功"),
        Err(e) => {
            println!("数据库迁移失败: {:?}", e);
            return Err(e.into());
        }
    }

    // 创建用户服务
    let user_service = UserService::new(db, config.jwt_secret);

    // 创建 Rocket 实例
    let mut rocket = rocket::build()
        .mount("/", routes![user_handler::register, user_handler::login])
        .manage(user_service);

    // 只在开发环境添加 Swagger
    if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()) == "development" {
        rocket = rocket.mount(
            "/swagger",
            SwaggerUi::new("/swagger-ui/<_..>")
                .url("/api-docs/openapi.json", api::ApiDoc::openapi())
        );
    }

    rocket.launch().await?;

    Ok(())
}
