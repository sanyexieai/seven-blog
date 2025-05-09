use rocket::{post, serde::json::Json, State};
use utoipa::OpenApi;
use crate::{
    error::AppError,
    models::user::{CreateUserDto, LoginDto, TokenResponse},
    services::user_service::UserService,
};

// 用户注册
#[utoipa::path(
    post,
    path = "/register",
    request_body = CreateUserDto,
    responses(
        (status = 200, description = "注册成功", body = TokenResponse),
        (status = 400, description = "请求参数错误"),
        (status = 409, description = "用户已存在")
    ),
    tag = "auth"
)]
#[post("/register", format = "json", data = "<dto>")]
pub async fn register(
    user_service: &State<UserService>,
    dto: Json<CreateUserDto>,
) -> Result<Json<TokenResponse>, AppError> {
    let token = user_service.register(dto.into_inner()).await?;
    Ok(Json(TokenResponse { token }))
}

/// 用户登录
#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginDto,
    responses(
        (status = 200, description = "登录成功", body = TokenResponse),
        (status = 400, description = "请求参数错误"),
        (status = 401, description = "用户名或密码错误"),
        (status = 404, description = "用户不存在")
    ),
    tag = "auth"
)]
#[post("/login", format = "json", data = "<dto>")]
pub async fn login(
    user_service: &State<UserService>,
    dto: Json<LoginDto>,
) -> Result<Json<TokenResponse>, AppError> {
    let token = user_service
        .login(&dto.username, &dto.password)
        .await?;
    Ok(Json(TokenResponse { token }))
} 