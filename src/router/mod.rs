use crate::zhttp::router::{Router, RouterItem};
use crate::zhttp::middleware::{ZRequest, HttpPhase};
use hyper::{Response, Body};
use http::Method;
use std::collections::HashMap;
use crate::lsystem;
use crate::zhttp::HttpResult;

pub fn init_router() -> Router {
    Router::new(Box::new(create_router_map))
}


fn create_router_map() -> HashMap<String, RouterItem> {
    let mut router_map: HashMap<String, RouterItem> = HashMap::new();
    let validate_logon_item = RouterItem {
        method: Some(vec![Method::GET]),
        handler: Box::new(validate_logon),
    };
    let validate_init_item =RouterItem{
        method:Some(vec![Method::GET]),
        handler:Box::new(validate_init)
    };
    router_map.insert(String::from("/zly/validate-logon"), validate_logon_item);
    router_map.insert(String::from("/zly/is_init"),validate_init_item);
    router_map
}

fn validate_logon(zreq: &mut ZRequest, response: &mut Response<Body>, _hp: &mut HttpPhase) {
    zreq.session.set_value("validate");
    *response.body_mut() = Body::from("validate");
}


fn validate_init(zreq: &mut ZRequest, response: &mut Response<Body>, _hp: &mut HttpPhase) {
    let is_init= lsystem::is_init(&zreq.db_worker.lock().unwrap().connection);
    let res:HttpResult<bool>=HttpResult {
        success: true,
        message: None,
        data: is_init
    };
    *response.body_mut() =Body::from(serde_json::to_string(&res).unwrap())

}