use askama::Template;
use axum::{
    body::{self, Full},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

pub mod about;
pub mod admin;
pub mod blog;
pub mod category;
pub mod series;

pub struct HtmlTemplate<T>(pub T);

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
