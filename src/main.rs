extern crate futures;
extern crate futures_util;
extern crate hyper;
extern crate tokio;
extern crate zformdata;
extern crate redis;
extern crate crypto;
extern crate uuid;

use zly::zhttp::ZHttp;
use zly::zredis::RedisConfig;
use zly::zhttp::session::SessionConfig;

fn main() {
    start_server();
}

fn start_server(){
    let redis_config=RedisConfig{
        host: String::from("127.0.0.1"),
        port: 6379,
        database:1,
        auth:String::from("ck123456")
    };
    let session_config=SessionConfig{
        redis_config,
        prefix:None,
        secret:None,
    };
    let mut http_server = ZHttp::new(8000).session_redis(session_config);
    http_server.start_server();
}
