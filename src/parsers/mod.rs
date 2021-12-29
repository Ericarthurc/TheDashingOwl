use async_fs;
use chrono::NaiveDate;
use futures_lite::stream::StreamExt;

use crate::errors::AppError;

use self::meta::Meta;
use self::parsers::{markdown_parser, meta_parser};

pub mod meta;
pub mod parsers;

pub async fn get_file(file_name: &str) -> Result<String, AppError> {
    let file_content =
        async_fs::read_to_string(format!("./markdown/{}.markdown", file_name)).await?;

    Ok(file_content)
}

pub async fn get_blog_index_vec() -> Result<Vec<Meta>, AppError> {
    let mut meta_vec: Vec<Meta> = vec![];

    let mut files = async_fs::read_dir("./markdown").await?;

    while let Some(file) = files.next().await {
        meta_vec.push(
            meta_parser(
                file.unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .to_string()
                    .split(".markdown")
                    .collect::<Vec<&str>>()[0],
            )
            .await?,
        );
    }

    meta_vec.sort_by(|a, b| {
        let a_date = NaiveDate::parse_from_str(&a.date, "%B %d, %Y").unwrap();
        let b_date = NaiveDate::parse_from_str(&b.date, "%B %d, %Y").unwrap();
        b_date.cmp(&a_date)
    });

    Ok(meta_vec)
}

pub async fn get_series_index_vec() -> Result<Vec<Meta>, AppError> {
    let mut meta_vec: Vec<Meta> = vec![];
    Ok(meta_vec)
}

pub async fn get_meta_by_series_vec() -> Result<Vec<Meta>, AppError> {
    let mut meta_vec: Vec<Meta> = vec![];
    Ok(meta_vec)
}

pub async fn get_meta_and_markdown(file_name: &str) -> Result<(Meta, String), AppError> {
    let meta = meta_parser(file_name).await?;
    let mark = markdown_parser(file_name).await?;
    Ok((meta, mark))
}
