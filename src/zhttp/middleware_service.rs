use futures_util::future;
use hyper::{Body, Request, Response};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use tower_service::Service;

use super::middleware::{HttpPhase, Middleware, ZRequest};
use super::session_middleware::SessionMiddleware;

pub struct MiddlewareService {
    session: Arc<Mutex<SessionMiddleware>>,
}

impl MiddlewareService {
    pub fn new(session_ref: Arc<Mutex<SessionMiddleware>>) -> Self {
        MiddlewareService {
            session: session_ref,
        }
    }
}

impl Service<Request<Body>> for MiddlewareService {
    type Response = Response<Body>;
    type Error = std::io::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let mut response = Response::new(Body::empty());
        let mut zreq = ZRequest::new(req);
        let mut hp: HttpPhase = HttpPhase::HandleRequest;
        let mut sess_t = self.session.lock().unwrap();
        sess_t.http_handler(&mut zreq, &mut response, &mut hp);
        hp = HttpPhase::HandleResponse;
        zreq.session.value = Some(String::from("test-session"));
        sess_t.http_handler(&mut zreq, &mut response, &mut hp);
        *response.body_mut() = Body::from("any");
        future::ok(response)
    }
}

pub struct MakeSvc {
    session: Arc<Mutex<SessionMiddleware>>,
}

impl MakeSvc {
    pub fn new(sess: SessionMiddleware) -> Self {
        MakeSvc {
            session: Arc::new(Mutex::new(sess)),
        }
    }
}

impl<T> Service<T> for MakeSvc {
    type Response = MiddlewareService;
    type Error = std::io::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, _: T) -> Self::Future {
        println!("新连接");
        future::ok(MiddlewareService::new(self.session.clone()))
    }
}
