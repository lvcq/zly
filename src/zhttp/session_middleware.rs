use super::cookie::Cookie;
use super::middleware::{HttpPhase, Middleware, ZRequest};
use super::session::{Session, SessionConfig};
use http::header::HeaderValue;
use http::header::{COOKIE, SET_COOKIE};
use hyper::{Body, Response};

pub struct SessionMiddleware {
  pub session: Session,
}

impl SessionMiddleware {
  pub fn new(config: SessionConfig) -> Self {
    SessionMiddleware {
      session: Session::new(config),
    }
  }
}

impl Middleware for SessionMiddleware {
  fn http_handler(
    &mut self,
    zreq: &mut ZRequest,
    response: &mut Response<Body>,
    hp: &mut HttpPhase,
  ) {
    match &hp {
      &HttpPhase::HandleRequest => {
        let cookie = zreq.req.headers().get(COOKIE);
        if cookie.is_some() {
          println!("cookie:{:?}", cookie.unwrap());
        } else {
          zreq.set_session_is_new(true);
        }
      }
      &HttpPhase::HandleResponse => {
        if zreq.session.is_new && zreq.session.value.is_some() {
          let value = zreq.session.get_value().unwrap();
          let key = match self.session.store_session(&value) {
            Ok(s_key) => s_key,
            Err(_) => String::from(""),
          };
          let cookie = Cookie::new("ZLY_SESSION_ID", &key)
            .set_max_age(60 * 30)
            .set_http_only(true);
            let cookie_str=cookie.to_string();
          println!("key:{}", &cookie_str);
          response.headers_mut().insert(
            SET_COOKIE,
            HeaderValue::from_str(&cookie_str).unwrap(),
          );
        }
      }
    };
  }
}
