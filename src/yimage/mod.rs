use crate::zhttp::ResponseCode;
use crate::zpostgres::models::file::ZlyFile;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use image::imageops::FilterType;
use image::GenericImageView;
use image::{DynamicImage, ImageFormat};
use std::fs::OpenOptions;
use std::io::BufReader;
use std::path::PathBuf;

pub async fn image_parse() {}

pub async fn load_iamge_by_id(
    image_id: &str,
    storage_root_path: &str,
    conn: &PgConnection,
) -> Result<(mime::Mime, String, PathBuf), ResponseCode> {
    let (hash, name, mime) = get_file_hash_by_id(image_id, conn)?;
    let _image_format: ImageFormat = match check_image_format(&mime) {
        Some(ifmt) => ifmt,
        None => return Err(ResponseCode::Code10007),
    };
    let mut file_path = PathBuf::from(storage_root_path);
    file_path.push(hash);
    Ok((mime, name, file_path))
}

fn get_file_hash_by_id(
    id: &str,
    conn: &PgConnection,
) -> Result<(String, String, mime::Mime), ResponseCode> {
    use crate::zpostgres::schema::zly_file::dsl::{file_id, zly_file};
    let result: QueryResult<Vec<ZlyFile>> = zly_file.filter(file_id.eq(id)).load::<ZlyFile>(conn);
    if result.is_err() {
        return Err(ResponseCode::Code10003);
    }

    let res_vec = result.unwrap();
    if res_vec.is_empty() {
        return Err(ResponseCode::Code10007);
    } else {
        Ok((
            res_vec[0].file_hash.clone(),
            res_vec[0].file_name.clone(),
            res_vec[0].file_mime.parse::<mime::Mime>().unwrap(),
        ))
    }
}

fn check_image_format(mime_ins: &mime::Mime) -> Option<ImageFormat> {
    match (mime_ins.type_(), mime_ins.subtype()) {
        (mime::IMAGE, mime::PNG) => Some(ImageFormat::Png),
        (mime::IMAGE, mime::JPEG) => Some(ImageFormat::Jpeg),
        _ => None,
    }
}

pub async fn load_image_and_resize(
    image_id: &str,
    storage_root_path: &str,
    cache_base: &str,
    width: usize,
    height: usize,
    conn: &PgConnection,
) -> Result<(mime::Mime, String, PathBuf), ResponseCode> {
    let (hash, name, mime) = get_file_hash_by_id(image_id, conn)?;
    let image_format: ImageFormat = match check_image_format(&mime) {
        Some(ifmt) => ifmt,
        None => return Err(ResponseCode::Code10007),
    };
    let (cache_exist, cache_path) =
        check_image_cache(cache_base, mime.subtype().as_str(), image_id, width, height);
    if !cache_exist {
        let mut file_path = PathBuf::from(storage_root_path);
        file_path.push(hash);
        let mut img = match read_image(file_path, image_format) {
            Some(img_ins) => img_ins,
            None => {
                return Err(ResponseCode::Code10003);
            }
        };
        img = img.resize_exact(width as u32, height as u32, FilterType::Gaussian);
        img.save(&cache_path).unwrap();
    }
    Ok((mime, name, cache_path))
}

pub async fn load_image_and_resize_only_width(
    image_id: &str,
    storage_root_path: &str,
    cache_base: &str,
    width: usize,
    conn: &PgConnection,
) -> Result<(mime::Mime, String, PathBuf), ResponseCode> {
    let (hash, name, mime) = get_file_hash_by_id(image_id, conn)?;
    let image_format: ImageFormat = match check_image_format(&mime) {
        Some(ifmt) => ifmt,
        None => return Err(ResponseCode::Code10007),
    };
    let mut file_path = PathBuf::from(storage_root_path);
    file_path.push(hash);
    let mut img = match read_image(file_path, image_format) {
        Some(img_ins) => img_ins,
        None => {
            return Err(ResponseCode::Code10003);
        }
    };
    let (iw, ih) = img.dimensions();
    let height: usize = (ih as usize) / (iw as usize) * width;
    let (cache_exist, cache_path) =
        check_image_cache(cache_base, mime.subtype().as_str(), image_id, width, height);
    if !cache_exist {
        img = img.resize_exact(width as u32, height as u32, FilterType::Gaussian);
        img.save(&cache_path).unwrap();
    }
    Ok((mime, name, cache_path))
}

fn read_image(img_path: PathBuf, img_format: ImageFormat) -> Option<DynamicImage> {
    let file = match OpenOptions::new().read(true).open(img_path) {
        Ok(img_file) => img_file,
        Err(_) => {
            return None;
        }
    };
    let buf = BufReader::new(file);
    match image::load(buf, img_format) {
        Ok(img) => Some(img),
        Err(_) => None,
    }
}

fn check_image_cache(
    cache_base: &str,
    mime_type: &str,
    id: &str,
    width: usize,
    height: usize,
) -> (bool, PathBuf) {
    let mut img_path = PathBuf::from(cache_base);
    img_path.push(mime_type);
    let dir_metadata = std::fs::metadata(&img_path);
    if dir_metadata.is_err() || !dir_metadata.unwrap().is_dir() {
        std::fs::DirBuilder::new()
            .recursive(true)
            .create(&img_path)
            .unwrap();
    }
    img_path.push(format!("{}_{}_{}.{}", id, width, height, mime_type));
    let file_metadata = std::fs::metadata(&img_path);
    return if file_metadata.is_ok() && file_metadata.unwrap().is_file() {
        (true, img_path)
    } else {
        (false, img_path)
    };
}
