use utoipa::OpenApi;
use crate::{
    handlers::user_handler,
    models::user::{User, UserInfo, CreateUserDto, LoginDto, TokenResponse},
    error::AppError,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        user_handler::register,
        user_handler::login,
    ),
    components(
        schemas(
            User,
            UserInfo,
            CreateUserDto,
            LoginDto,
            TokenResponse,
            AppError,
        )
    ),
    tags(
        (name = "auth", description = "认证相关接口")
    )
)]
pub struct ApiDoc; 