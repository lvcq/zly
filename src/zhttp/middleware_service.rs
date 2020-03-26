use futures_util::future;
use http::header::{HeaderValue, ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN, ORIGIN, ACCESS_CONTROL_ALLOW_HEADERS};
use hyper::{Body, Request, Response};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use tower_service::Service;

use super::middleware::ZRequest;
use super::router::{Router, RouterWorker};
use super::session_middleware::SessionMiddleware;
use crate::zpostgres::{DBWorker, PgPool};

pub struct MiddlewareService {
    session: Arc<Mutex<SessionMiddleware>>,
    router_worker: Arc<Mutex<RouterWorker>>,
    db_worker: Arc<Mutex<DBWorker>>,
}

impl MiddlewareService {
    pub fn new(
        session_ref: Arc<Mutex<SessionMiddleware>>,
        router_worker: Arc<Mutex<RouterWorker>>,
        db_worker: Arc<Mutex<DBWorker>>,
    ) -> Self {
        MiddlewareService {
            session: session_ref,
            router_worker,
            db_worker,
        }
    }

    fn cors(&self, zreq: &mut ZRequest, response: &mut Response<Body>) {
        let origin: HeaderValue = match zreq.headers().get(ORIGIN) {
            Some(hv) => hv.clone(),
            None => "*".parse().unwrap(),
        };
        response
            .headers_mut()
            .insert(ACCESS_CONTROL_ALLOW_ORIGIN, origin);
        response.headers_mut().insert(
            ACCESS_CONTROL_ALLOW_METHODS,
            "POST, GET, OPTIONS".parse().unwrap(),
        );
        response
            .headers_mut()
            .insert(ACCESS_CONTROL_ALLOW_CREDENTIALS, "true".parse().unwrap());
        response.headers_mut().insert(ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type,content-type".parse().unwrap());
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
        let mut zreq = ZRequest::new(req, self.db_worker.clone());
        self.cors(&mut zreq, &mut response);
        if zreq.method().as_str() != "OPTIONS" {
            let mut session_lock = self.session.lock().unwrap();
            session_lock.get_session(&mut zreq, &mut response);
            let worker_lock = self.router_worker.lock().unwrap();
            worker_lock.handler_request(&mut zreq, &mut response);
            session_lock.set_session(&mut zreq, &mut response);
            worker_lock.free_worker();
            let db_worker = self.db_worker.lock().unwrap();
            db_worker.free();
        }
        future::ok(response)
    }
}

pub struct MakeSvc {
    session: Arc<Mutex<SessionMiddleware>>,
    router: Router,
    pg_pool: PgPool,
}

impl MakeSvc {
    pub fn new(sess: SessionMiddleware, router: Router) -> Self {
        MakeSvc {
            session: Arc::new(Mutex::new(sess)),
            router,
            pg_pool: PgPool::new(),
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
        future::ok(MiddlewareService::new(
            self.session.clone(),
            self.router.get_free_worker(),
            self.pg_pool.get_free_worker(),
        ))
    }
}
