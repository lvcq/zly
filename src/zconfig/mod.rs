use std::env;
use dotenv::dotenv;

#[derive(Clone)]
pub struct AppConfig {
    pub file_storage_path: String
}

pub fn get_app_config() -> AppConfig {
    dotenv().ok();
    let file_storage_path = env::var("ZLY_FILE_STORAGE_PATH")
        .expect("FILE_STORAGE_PATH must be set.");
    AppConfig {
        file_storage_path
    }
}