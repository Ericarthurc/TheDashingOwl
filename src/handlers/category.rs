use std::collections::HashMap;

use askama::Template;

use crate::{errors::AppError, parsers::meta::Meta};

use super::HtmlTemplate;
use axum::{extract::Path, response::IntoResponse};

#[derive(Template)]
#[template(path = "category.html")]
struct CategoryTemplate {
    category: String,
    category_index: Vec<Meta>,
}
pub async fn category_handler(
    Path(params): Path<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    let category = params.get("category").unwrap();

    Ok(())
}
