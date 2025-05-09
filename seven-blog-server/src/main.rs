use rocket::{get, routes, Build, Rocket};
use sea_orm::{Database, DatabaseConnection};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use sea_orm_migration::MigratorTrait;

mod config;
mod entity;
mod error;
mod handlers;
mod models;
mod services;
mod migration;
mod utils;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::user_handler::register,
        handlers::user_handler::login,
    ),
    components(
        schemas(
            models::user::User,
            models::user::UserInfo,
            models::user::CreateUserDto,
            models::user::LoginDto,
            models::user::TokenResponse,
            error::AppError,
        )
    ),
    tags(
        (name = "users", description = "用户管理接口")
    )
)]
struct ApiDoc;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let config = config::Config::from_env()?;
    let db_url = config.database_url();

    // 创建数据库连接
    let db = Database::connect(&db_url).await?;

    // 运行数据库迁移
    migration::Migrator::up(&db, None).await?;

    // 创建用户服务
    // let user_service = UserService::new(pool, config.jwt_secret);

    // 创建 Rocket 实例
    let mut rocket = rocket::build()
        .mount("/api", routes![index]);
        // .mount("/api/auth", routes![user_handler::register, user_handler::login])
        // .manage(user_service);

    // 只在开发环境添加 Swagger
    if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()) == "development" {
        rocket = rocket.mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>")
                .url("/api-docs/openapi.json", ApiDoc::openapi())
        );
    }
    rocket.launch().await?;

    Ok(())
}
