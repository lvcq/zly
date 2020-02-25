use crate::zhttp::router::{Router,RouterItem};
use crate::zhttp::middleware::{ZRequest,HttpPhase};
use hyper::{Response,Body};
use http::Method;

pub fn init_router()->Router{
    let validate_logon_item=RouterItem{
        method:Some(vec![Method::GET]),
        handler:Box::new(validate_logon)
    };
    Router::new().add_router("validate-logon",validate_logon_item)
}

fn validate_logon(zreq:&mut ZRequest,response:&mut Response<Body>,hp:&mut HttpPhase){
   *response.body_mut()=Body::from("validate");
}
