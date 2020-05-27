use crate::yimage;
use crate::zconfig::AppConfig;
use crate::zhttp::generate_response_with_response_code;
use crate::zhttp::response_code::ResponseCode;
use crate::zpostgres::{DBWorker, PgPool};
use actix_files::NamedFile;
use actix_http::http::header::{ContentDisposition, DispositionParam, DispositionType};
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use image::ImageFormat;
use std::path::PathBuf;
use std::rc::Rc;

enum ImageParseParameter {
    Error,
    Empty,
    ImageSizeWidthAndHeight((usize, usize)),
    ImageSizeWidth(usize),
    ImageFormat(ImageFormat),
}

pub async fn image_parse_handler(
    path: web::Path<(String, PathBuf)>,
) -> Result<HttpResponse, actix_web::Error> {
    let id: String = path.0.parse().unwrap();

    Ok(HttpResponse::Ok().body(format!("img id: {}", id)))
}

pub async fn origin_image_handler(
    req: HttpRequest,
    path: web::Path<(String,)>,
    db: web::Data<PgPool>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse, actix_web::Error> {
    let id: String = path.0.parse().unwrap();
    if id.trim().is_empty() {
        return Ok(generate_response_with_response_code(
            ResponseCode::Code10008,
            true,
        ));
    }
    let db_worker = db.get_free_worker().unwrap();
    let res = find_image_by_id(&id, &req, db_worker.clone(), &app_config.file_storage_path).await;
    db.free(db_worker.index);
    Ok(res)
}

fn generate_cd(filename: String) -> ContentDisposition {
    ContentDisposition {
        disposition: DispositionType::Inline,
        parameters: vec![DispositionParam::Filename(filename)],
    }
}

pub async fn image_parse_with_size_or_format(
    path: web::Path<(String, String)>,
    req: HttpRequest,
    db: web::Data<PgPool>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse, actix_web::Error> {
    let id: String = path.0.parse().unwrap();
    let params: String = path.1.parse().unwrap();
    let db_worker = db.get_free_worker().unwrap();
    let res: HttpResponse;
    match check_parameter(&params) {
        ImageParseParameter::Error => {
            res = generate_response_with_response_code(ResponseCode::Code10009, true);
        }
        ImageParseParameter::Empty => {
            res =
                find_image_by_id(&id, &req, db_worker.clone(), &app_config.file_storage_path).await;
        }
        ImageParseParameter::ImageSizeWidth(width) => {
            res = resize_image_only_width(
                &id,
                &app_config.file_storage_path,
                &app_config.image_cache_path,
                db_worker.clone(),
                width,
                &req,
            )
            .await;
        }
        ImageParseParameter::ImageSizeWidthAndHeight((width, height)) => {
            res = resize_image(
                &id,
                &app_config.file_storage_path,
                &app_config.image_cache_path,
                db_worker.clone(),
                width,
                height,
                &req,
            )
            .await;
        }
        _ => {
            res = generate_response_with_response_code(ResponseCode::Code10009, true);
        }
    }
    db.free(db_worker.index);
    Ok(res)
}

async fn find_image_by_id(
    id: &str,
    req: &HttpRequest,
    db_worker: Rc<DBWorker>,
    data_root_path: &str,
) -> HttpResponse {
    let (mime_type, img_name, img_path) =
        match yimage::load_iamge_by_id(id, data_root_path, &db_worker.connection).await {
            Ok((m_str, f_name, i_path)) => (m_str, f_name, i_path),
            Err(rsc) => {
                return generate_response_with_response_code(rsc, true);
            }
        };
    match NamedFile::open(img_path) {
        Ok(f_stream) => match f_stream
            .set_content_type(mime_type)
            .set_content_disposition(generate_cd(img_name))
            .into_response(req)
        {
            Ok(res) => {
                return res;
            }
            Err(_) => {
                return generate_response_with_response_code(ResponseCode::Code10003, true);
            }
        },
        Err(_) => {
            return generate_response_with_response_code(ResponseCode::Code10003, true);
        }
    }
}

fn check_parameter(parameter: &str) -> ImageParseParameter {
    if parameter.trim().is_empty() {
        return ImageParseParameter::Empty;
    }
    let low_parameter = parameter.to_lowercase();
    if low_parameter.eq(&"png".to_string()) {
        return ImageParseParameter::ImageFormat(ImageFormat::Png);
    }
    if low_parameter.eq(&"jpeg".to_string()) || low_parameter.eq(&"jpg".to_string()) {
        return ImageParseParameter::ImageFormat(ImageFormat::Jpeg);
    }

    if low_parameter.eq(&"gif".to_string()) {
        return ImageParseParameter::ImageFormat(ImageFormat::Gif);
    }
    if low_parameter.eq(&"webp".to_string()) {
        return ImageParseParameter::ImageFormat(ImageFormat::WebP);
    }

    return check_parameter_is_size(parameter);
}

fn check_parameter_is_size(param: &str) -> ImageParseParameter {
    let mut split_index: usize = 0;
    let mut split_count: usize = 0;
    let mut chars = param.chars();
    let mut index: usize = 0;
    while let Some(current) = chars.next() {
        if current == 'x' {
            split_index = index;
            split_count = split_count + 1;
            if split_count > 1 {
                return ImageParseParameter::Error;
            }
        } else if !current.is_ascii_digit() {
            return ImageParseParameter::Error;
        }

        index = index + 1;
    }

    if split_count == 1 && (split_index == 0 || split_index == param.len() - 1) {
        return ImageParseParameter::Error;
    }

    if split_count == 0 {
        return ImageParseParameter::ImageSizeWidth(param.parse::<usize>().unwrap());
    } else {
        let width: usize = param[0..split_index].parse().unwrap();
        let height: usize = param[split_index + 1..].parse().unwrap();
        return ImageParseParameter::ImageSizeWidthAndHeight((width, height));
    }
}

async fn resize_image(
    id: &str,
    base_path: &str,
    cache_path: &str,
    db_worker: Rc<DBWorker>,
    width: usize,
    height: usize,
    req: &HttpRequest,
) -> HttpResponse {
    let (img_mime, img_name, img_cache) = match yimage::load_image_and_resize(
        id,
        base_path,
        cache_path,
        width,
        height,
        &db_worker.connection,
    )
    .await
    {
        Ok((i_mime, i_name, cache)) => (i_mime, i_name, cache),
        Err(rsc) => {
            return generate_response_with_response_code(rsc, true);
        }
    };
    read_image_to_response(img_mime, img_name, img_cache, req)
}

async fn resize_image_only_width(
    id: &str,
    base_path: &str,
    cache_path: &str,
    db_worker: Rc<DBWorker>,
    width: usize,
    req: &HttpRequest,
) -> HttpResponse {
    let (img_mime, img_name, img_cache) = match yimage::load_image_and_resize_only_width(
        id,
        base_path,
        cache_path,
        width,
        &db_worker.connection,
    )
    .await
    {
        Ok((i_mime, i_name, cache)) => (i_mime, i_name, cache),
        Err(rsc) => {
            return generate_response_with_response_code(rsc, true);
        }
    };
    read_image_to_response(img_mime, img_name, img_cache, req)
}

fn read_image_to_response(
    img_mime: mime::Mime,
    img_name: String,
    img_path: PathBuf,
    req: &HttpRequest,
) -> HttpResponse {
    match NamedFile::open(img_path) {
        Ok(f_stream) => match f_stream
            .set_content_type(img_mime)
            .set_content_disposition(generate_cd(img_name))
            .into_response(req)
        {
            Ok(res) => {
                return res;
            }
            Err(_) => {
                return generate_response_with_response_code(ResponseCode::Code10003, true);
            }
        },
        Err(_) => {
            return generate_response_with_response_code(ResponseCode::Code10003, true);
        }
    }
}
