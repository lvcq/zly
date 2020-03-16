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

    fn find_target_cookie(&self, target_key: &str, cookie_str: &str) -> Option<String> {
        let cookie_vec: Vec<&str> = cookie_str.split("; ").collect();
        let len = cookie_vec.len();
        let mut index: usize = 0;
        while index < len {
            let cookie: Cookie = match cookie_vec[index].parse::<Cookie>() {
                Ok(coo) => coo,
                Err(_) => {
                    index = index + 1;
                    continue;
                }
            };
            if cookie.get_name() == target_key {
                return Some(String::from(cookie.get_value()));
            }
            index = index + 1;
        }
        None
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
                    let coo_value = self
                        .find_target_cookie("ZLY_SESSION_ID", cookie.unwrap().to_str().unwrap());
                    if coo_value.is_none() {
                        zreq.set_session_is_new(true);
                    } else {
                        zreq.set_session_is_new(false);
                        let r_key = coo_value.unwrap();
                        println!("session key: {}", &r_key);
                        let s_value = self.session.get_session_by_key(&r_key);
                        if s_value.is_none() {
                            zreq.set_session_is_new(true);
                        } else {
                            zreq.session.set_key(&r_key);
                            zreq.session.set_value(&s_value.unwrap());
                        }
                    }
                } else {
                    zreq.set_session_is_new(true);
                }
            }
            &HttpPhase::HandleResponse => {
                let mut key: String = String::from("");
                if zreq.session.is_new && zreq.session.value.is_some() {
                    let value = zreq.session.get_value().unwrap();
                    key = match self.session.store_session(&value) {
                        Ok(s_key) => s_key,
                        Err(_) => String::from(""),
                    };
                } else if !zreq.session.is_new
                    && zreq.session.value.is_some()
                    && zreq.session.key.is_some()
                {
                    key = match &zreq.session.key {
                        Some(s_key) => s_key.clone(),
                        None => String::from(""),
                    };
                }
                if !key.eq("") {
                    let cookie = Cookie::new("ZLY_SESSION_ID", &key)
                        .set_max_age(60 * 30)
                        .set_http_only(true);
                    let cookie_str = cookie.to_string();
                    println!("key:{}", &cookie_str);
                    response
                        .headers_mut()
                        .insert(SET_COOKIE, HeaderValue::from_str(&cookie_str).unwrap());
                }
            }
        };
    }
}
