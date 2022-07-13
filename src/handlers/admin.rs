use askama::Template;
use axum::{extract, response::IntoResponse, response::Response};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

use super::HtmlTemplate;
use crate::errors::AppError;

#[derive(Template)]
#[template(path = "admin.html.j2")]
struct AdminTemplate {}

pub async fn admin_handler() -> Result<impl IntoResponse, AppError> {
    let template = AdminTemplate {};
    Ok(HtmlTemplate(template))
}

#[derive(Template)]
#[template(path = "admin_login.html.j2")]
struct AdminLoginTemplate {}

pub async fn admin_login_handler() -> Result<Response, AppError> {
    let template = AdminLoginTemplate {};
    Ok(HtmlTemplate(template).into_response())
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct LoginInput {
    username: String,
    password: String,
}

pub async fn admin_login_consumer_handler(
    extract::Json(login_payload): extract::Json<LoginInput>,
    cookies: Cookies,
) -> Result<Response, AppError> {
    println!("{:?}", login_payload);
    Ok((StatusCode::FORBIDDEN).into_response())
}
