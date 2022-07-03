use askama::Template;
use axum::{
    handler::Handler,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use handlers::HtmlTemplate;
use std::net::SocketAddr;

use crate::handlers::{
    about::about_handler,
    blog::{blog_handler, blog_index_handler},
    category::category_handler,
    series::{series_handler, series_index_handler},
};

use tower_http::services::{fs::ServeDir, ServeFile};

mod errors;
mod handlers;
mod parsers;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .fallback(handler_404.into_service())
        .route(
            "/sw.js",
            get_service(ServeFile::new("./public/js/sw.js")).handle_error(
                |error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                },
            ),
        )
        .route(
            "/robots.txt",
            get_service(ServeFile::new("./public/robots.txt")).handle_error(
                |error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                },
            ),
        )
        .route("/", get(blog_index_handler))
        .route("/blog/:blog", get(blog_handler))
        .route("/category/:category", get(category_handler))
        .route("/series", get(series_index_handler))
        .route("/series/:series", get(series_handler))
        .route("/about", get(about_handler))
        .nest(
            "/public",
            get_service(ServeDir::new("./public/")).handle_error(
                |error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                },
            ),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Server: {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Template)]
#[template(path = "not_found.html.j2")]
struct NotFoundTemplate {}
async fn handler_404() -> impl IntoResponse {
    let template = NotFoundTemplate {};
    HtmlTemplate(template)
}
