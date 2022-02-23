use crate::errors::AppError;

use super::HtmlTemplate;
use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {}

pub async fn about_handler() -> Result<impl IntoResponse, AppError> {
    let template = AboutTemplate {};
    Ok(HtmlTemplate(template))
}
