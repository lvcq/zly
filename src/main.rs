use actix_cors::Cors;
use actix_redis::RedisSession;
use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use zly::router;
use zly::zconfig::get_app_config;
use zly::zpostgres::PgPool;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| {
        App::new()
            .data(get_app_config())
            .data(PgPool::new())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(
                Cors::new()
                    .supports_credentials()
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::ACCEPT,
                        http::header::HeaderName::from_static("upload-info"),
                    ])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .wrap(
                RedisSession::new("192.168.47.1:6379", &[0; 32])
                    .ttl(60 * 60 * 3)
                    .cookie_name("ZLY_SESSION"),
            )
            .service(
                web::scope("/zly")
                    .route("/validate-logon", web::get().to(router::validate_logon))
                    .route("/is-init", web::get().to(router::validate_init))
                    .route("/set-root-info", web::post().to(router::set_root_info))
                    .route("/user-login", web::post().to(router::user_login))
                    .route(
                        "/add-new-storage",
                        web::post().to(router::add_new_storage_handler),
                    )
                    .route(
                        "/user-storage-list",
                        web::get().to(router::get_user_storage),
                    )
                    .route(
                        "/check-file-exist",
                        web::post().to(router::check_file_exist_in_folder),
                    )
                    .route(
                        "/add-exist-file",
                        web::post().to(router::create_ref_with_exist_file_and_storage),
                    )
                    .route("/upload-file", web::post().to(router::file_upload_handler))
                    .route("/get-storage-files", web::get().to(router::query_files))
                    .route("/img/{id}", web::get().to(router::origin_image_handler))
                    .route("/img/{id}/", web::get().to(router::origin_image_handler))
                    .route(
                        "/img/{id}/{parameter}",
                        web::get().to(router::image_parse_with_size_or_format),
                    ),
            )
    })
    .workers(8)
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
