use askama::Template;
use axum::{
    body,
    handler::Handler,
    http::header::CONTENT_TYPE,
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use handlers::HtmlTemplate;
use std::net::SocketAddr;

use prometheus::{Encoder, TextEncoder};

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
        // .route("/metrics", get(metrics))
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
#[template(path = "not_found.html")]
struct NotFoundTemplate {}
async fn handler_404() -> impl IntoResponse {
    let template = NotFoundTemplate {};
    HtmlTemplate(template)
}

// Not ready for production
// async fn metrics() -> impl IntoResponse {
//     let encoder = TextEncoder::new();
//     let metric_families = prometheus::gather();
//     let mut buffer = vec![];
//     encoder.encode(&metric_families, &mut buffer).unwrap();
//     Response::builder()
//         .status(200)
//         .header(CONTENT_TYPE, encoder.format_type())
//         .body(body::boxed(body::Full::from(buffer)))
//         .unwrap()
// }
