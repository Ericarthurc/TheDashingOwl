use askama::Template;
use axum::{
    body::{self, Full},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use crate::errors::AppError;

pub mod blog;
pub mod series;

pub struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(body::boxed(Full::from(format!(
                    "Failed to render template. Error: {}",
                    err
                ))))
                .unwrap(),
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

pub async fn index_handler() -> Result<impl IntoResponse, AppError> {
    let template = IndexTemplate {};
    Ok(HtmlTemplate(template))
}
