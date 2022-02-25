use axum::{
    http::{Response, StatusCode},
    routing::{get, get_service},
    Router,
};
use std::net::SocketAddr;

use crate::handlers::{
    about::about_handler,
    blog::{blog_handler, blog_index_handler},
    series::{series_handler, series_index_handler},
};

use http_body::{Body as _, Full};
use std::io;
use tower::ServiceBuilder;
use tower_http::services::fs::{ServeDir, ServeFileSystemResponseBody};

mod errors;
mod handlers;
mod parsers;

#[tokio::main]
async fn main() {
    let series_routes = Router::new()
        .route("/", get(series_index_handler))
        .route("/:series", get(series_handler));

    let handler_404 = ServiceBuilder::new().and_then(
        |response: Response<ServeFileSystemResponseBody>| async move {
            let response = if response.status() == StatusCode::NOT_FOUND {
                let body = Full::from("Not Found").map_err(|err| match err {}).boxed();
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(body)
                    .unwrap()
            } else {
                response.map(|body| body.boxed())
            };

            Ok::<_, io::Error>(response)
        },
    );

    let app = Router::new()
        .fallback(
            get_service(handler_404.service(ServeDir::new("./public"))).handle_error(
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
        .nest("/series", series_routes)
        .route("/about", get(about_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Server: {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
