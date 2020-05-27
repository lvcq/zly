use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub file_storage_path: String,
    pub image_cache_path: String,
}

pub fn get_app_config() -> AppConfig {
    dotenv().ok();
    let file_storage_path =
        env::var("ZLY_FILE_STORAGE_PATH").expect("FILE_STORAGE_PATH must be set.");
    let image_cache_path =
        env::var("ZLY_IMAGE_CACHE_DIR").expect("ZLY_IMAGE_CACHE_DIR must be set.");
    AppConfig {
        file_storage_path,
        image_cache_path,
    }
}
