use hyper::{Body, Request, Response};

pub trait Middleware {
  fn http_handler(
    &mut self,
    zreq: &mut ZRequest,
    response: &mut Response<Body>,
    hp: &mut HttpPhase,
  ) {
  }
}

pub enum HttpPhase {
  HandleRequest,
  HandleResponse,
}

pub struct ZRequest {
  pub req: Request<Body>,
  pub session: ZSession,
}

impl ZRequest {
  pub fn new(req: Request<Body>) -> Self {
    ZRequest {
      req,
      session: ZSession::new(),
    }
  }
  pub fn set_session_is_new(&mut self, is_n: bool) {
    self.session.set_is_new(is_n);
  }
}

pub struct ZSession {
  pub value: Option<String>,
  pub key: Option<String>,
  pub is_new: bool,
}

impl ZSession {
  pub fn new() -> Self {
    ZSession {
      value: None,
      key: None,
      is_new: false,
    }
  }

  pub fn set_is_new(&mut self, is_n: bool) {
    self.is_new = is_n;
  }
  pub fn get_value(&self) -> Option<String> {
    match &self.value {
      Some(str_v) => Some(str_v.clone()),
      None => None,
    }
  }
}
