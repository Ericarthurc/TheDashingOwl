use crate::errors::AppError;

use super::meta::Meta;
use comrak::{markdown_to_html, ComrakOptions};

pub async fn markdown_parser(file_contents: &String) -> Result<String, AppError> {
    let mut options = ComrakOptions::default();
    options.extension.autolink = true;
    options.extension.table = true;
    options.extension.description_lists = true;
    options.extension.superscript = true;
    options.extension.strikethrough = true;
    options.extension.footnotes = true;
    options.extension.front_matter_delimiter = Some("---".to_owned());
    options.render.unsafe_ = true;

    Ok(markdown_to_html(&file_contents, &options))
}

pub async fn meta_parser(file_contents: &String, file_name: &str) -> Result<Meta, AppError> {
    Ok(Meta::new(&file_contents, file_name))
}
