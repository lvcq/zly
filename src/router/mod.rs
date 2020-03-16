use crate::zhttp::router::{Router,RouterItem};
use crate::zhttp::middleware::{ZRequest,HttpPhase};
use hyper::{Response,Body};
use http::Method;
use std::collections::HashMap;

pub fn init_router()->Router{
    Router::new(Box::new(create_router_map))
}


fn create_router_map()->HashMap<String,RouterItem>{
    let mut router_map:HashMap<String,RouterItem> = HashMap::new();
    let validate_logon_item = RouterItem{
        method:Some(vec![Method::GET]),
        handler:Box::new(validate_logon)
    };
    router_map.insert(String::from("/zly/validate-logon"), validate_logon_item);
    router_map
}

fn validate_logon(zreq:&mut ZRequest,response:&mut Response<Body>,_hp:&mut HttpPhase){
    zreq.session.set_value("validate");
   *response.body_mut()=Body::from("validate");
}
