use std::env;

use dotenv::dotenv;
use once_cell::sync::Lazy;

pub struct Config {
    pub image_uploads_dir: String,
    pub request_body_limit: usize,
    pub db_url: String,
    pub server_addr: String,
    pub frontend_url: String,
    pub default_list_limit: usize,
    pub max_list_limit: usize,
}

pub static APP_CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenv().ok();
    let image_uploads_dir =
        env::var("IMAGE_UPLOADS_DIR").expect("IMAGE_UPLOADS_DIR must be set in .env");
    let request_body_limit = env::var("REQUEST_BODY_LIMIT")
        .map(|val| {
            val.parse::<usize>()
                .expect("Failed to parse REQUEST_BODY_LIMIT")
        })
        .unwrap_or(1024 * 1024 * 5); // Default is 5Mib
    let db_url = env::var("DB_URL").expect("DB_URL must be set in .env");
    let server_addr = env::var("SERVER_ADDR").unwrap_or("0.0.0.0:3000".into());
    let frontend_url = env::var("FRONTEND_URL").expect("FRONTEND_URL must be set in .env");
    let default_list_limit = env::var("DEFAULT_LIST_LIMIT")
        .map(|val| {
            val.parse::<usize>()
                .expect("Failed to parse DEFAULT_LIST_LIMIT")
        })
        .unwrap_or(32);
    let max_list_limit = env::var("MAX_LIST_LIMIT")
        .map(|val| {
            val.parse::<usize>()
                .expect("Failed to parse MAX_LIST_LIMIT")
        })
        .unwrap_or(128);

    Config {
        image_uploads_dir,
        request_body_limit,
        db_url,
        server_addr,
        frontend_url,
        default_list_limit,
        max_list_limit,
    }
});
