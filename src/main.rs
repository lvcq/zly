extern crate futures;
extern crate futures_util;
extern crate hyper;
extern crate tokio;
extern crate zformdata;

use zly::zhttp;

fn main() {
    zhttp::start_server();
}
