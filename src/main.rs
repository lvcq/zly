
use zly::zhttp::session::SessionConfig;
use zly::zhttp::ZHttp;
use zly::zredis::RedisConfig;
use zly::router;

fn main() {
    start_server();
}

fn start_server() {
    let redis_config = RedisConfig {
        host: String::from("192.168.47.128"),
        port: 6379,
        database: 1,
        auth: String::from("ck123456"),
    };
    let session_config = SessionConfig {
        redis_config,
        prefix: None,
        secret: None,
    };
    let zly_router=router::init_router();
    let http_server = ZHttp::new(8000).session_redis(session_config);
    http_server.start_server(zly_router);
}
