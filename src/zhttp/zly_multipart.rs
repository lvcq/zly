use actix_multipart::{Field, Multipart};
use actix_web::http::header::ContentDisposition;
use actix_web::web;
use chrono::Utc;
use futures::StreamExt;
use futures::TryStreamExt;
use std::collections::HashMap;
use std::io::Write;
use std::{env, fs};

#[derive(Debug)]
pub struct FormDataValue {
    pub fields: HashMap<String, String>,
    pub files: HashMap<String, MultiFile>,
}

#[derive(Debug)]
pub struct MultiFile {
    pub filename: String,
    pub temp_path: String,
    pub mime: String,
    pub size: i64,
}

/// ### 读取`Multipart`数据流中的文件以及字段
pub async fn transfer_multipart(mut payload: Multipart) -> Result<FormDataValue, actix_web::Error> {
    let mut fdv = FormDataValue::new();
    while let Ok(Some(field)) = payload.try_next().await {
        let mut field: Field = field;
        let content_type: ContentDisposition = field.content_disposition().unwrap();
        let key_name = content_type.get_name().unwrap();
        match content_type.get_filename() {
            Some(filename) => {
                let file_type: mime::Mime = field.content_type().clone();
                fdv = parse_file(fdv, &mut field, filename, key_name, &file_type).await?;
            }
            None => {
                fdv = parse_field(fdv, &mut field, key_name).await?;
            }
        }
    }
    Ok(fdv)
}

/// 处理文件
async fn parse_file(
    mut fdv: FormDataValue,
    field: &mut Field,
    filename: &str,
    key: &str,
    mime_text: &mime::Mime,
) -> Result<FormDataValue, actix_web::Error> {
    let mut temp_file = env::temp_dir();
    let timestamp = Utc::now().timestamp_millis();
    temp_file.push(format!("{}-{}", timestamp, &filename));

    let temp_file_str = temp_file.to_str().unwrap().to_string();
    // File::create is blocking operation, use thread-pool
    let mut f = web::block(|| std::fs::File::create(temp_file))
        .await
        .unwrap();
    // Field in turn is stream of *Bytes* object
    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        // hasher.input(&data.to_vec()[..]);
        // filesystem operations are blocking, we have to use thread-pool
        f = web::block(move || f.write_all(&data).map(|_| f)).await?;
    }
    fdv.files.insert(
        String::from(key),
        MultiFile {
            filename: filename.to_string(),
            temp_path: temp_file_str.clone(),
            mime: mime_text.to_string(),
            size: fs::metadata(temp_file_str).unwrap().len() as i64,
        },
    );
    Ok(fdv)
}

/// 转换formdata为key:value
async fn parse_field(
    mut fdv: FormDataValue,
    field: &mut Field,
    key: &str,
) -> Result<FormDataValue, actix_web::Error> {
    let mut value_vec: Vec<u8> = Vec::new();
    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap().to_vec();
        value_vec.extend_from_slice(&data);
    }
    fdv.fields.insert(
        String::from(key),
        String::from_utf8_lossy(&value_vec).to_string(),
    );
    Ok(fdv)
}

impl FormDataValue {
    pub fn new() -> Self {
        FormDataValue {
            fields: HashMap::new(),
            files: HashMap::new(),
        }
    }
}

impl Drop for MultiFile {
    fn drop(&mut self) {
        match std::fs::remove_file(&self.temp_path) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}
