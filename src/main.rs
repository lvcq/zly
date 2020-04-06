use zly::router;
use actix_web::{HttpServer, App, web, middleware::Logger};
use actix_redis::RedisSession;
use env_logger::Env;
use zly::zpostgres::PgPool;
use actix_cors::Cors;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| {
        App::new()
            .data(PgPool::new())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(
                Cors::new()
                    .supports_credentials()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish()
            )
            .wrap(
                RedisSession::new("192.168.47.1:6379", &[0; 32])
                    .ttl(60 * 30)
                    .cookie_name("ZLY_SESSION")
            )
            .service(
                web::scope("/zly")
                    .route("/validate-logon", web::get().to(router::validate_logon))
                    .route("/is-init", web::get().to(router::validate_init))
                    .route("/set-root-info", web::post().to(router::set_root_info))
                    .route("/user-login", web::post().to(router::user_login))
            )
    })
        .workers(8)
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
