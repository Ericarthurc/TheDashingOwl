use std::collections::HashMap;

use askama::Template;

use crate::{
    errors::AppError,
    parsers::{get_meta_by_category_vec, meta::Meta},
};

use super::HtmlTemplate;
use axum::{extract::Path, response::IntoResponse};

#[derive(Template)]
#[template(path = "category.html.j2")]
struct CategoryTemplate {
    category: String,
    category_index: Vec<Meta>,
}
pub async fn category_handler(
    Path(params): Path<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    let category = params.get("category").unwrap();

    let category_meta = get_meta_by_category_vec(category).await?;

    let template = CategoryTemplate {
        category: category.clone(),
        category_index: category_meta,
    };
    Ok(HtmlTemplate(template))
}
