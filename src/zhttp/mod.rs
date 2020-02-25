use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use zformdata::FormValue;

pub mod middleware;
pub mod middleware_service;
pub mod session;
pub mod session_middleware;
pub mod timeout;
pub mod cookie;

use middleware_service::MakeSvc;
use session::{Session, SessionConfig};
use session_middleware::SessionMiddleware;
use timeout::TimeoutLayer;
async fn handle_connect(req: Request<Body>) -> Result<Response<Body>, Infallible> {
  let mut response = Response::new(Body::empty());

  match (req.method(), req.uri().path()) {
    (&Method::GET, "/") => {
      *response.body_mut() = Body::from("Try POSTing data to /echo");
    }
    (&Method::GET, "/validate-logon") => {
      *response.body_mut() = Body::from("next");
    }
    (&Method::POST, "/file/upload") => {
      let fv: FormValue = zformdata::read_formdata(req).await;
      println!("form value:{:?}", fv);
      *response.body_mut() = Body::from("111");
    }
    _ => {
      *response.status_mut() = StatusCode::NOT_FOUND;
    }
  };

  Ok(response)
}

pub struct ZHttp {
  port: u16,
  session_config: Option<SessionConfig>,
}

impl ZHttp {
  pub fn new(port: u16) -> ZHttp {
    ZHttp {
      port,
      session_config: None,
    }
  }

  pub fn session_redis(mut self, config: SessionConfig) -> Self {
    self.session_config = Some(config);
    return self;
  }

  #[tokio::main]
  pub async fn start_server(&self) {
    let addr = SocketAddr::from(([192, 168, 164, 129], self.port));
    let sess_mi = SessionMiddleware::new(self.session_config.as_ref().unwrap().clone());
    let server = Server::bind(&addr).serve(MakeSvc::new(sess_mi));
    // 程序关闭处理信号
    let graceful = server.with_graceful_shutdown(shutdown_signal());
    if let Err(e) = graceful.await {
      println!("server error: {}", e);
    }
  }
}

async fn shutdown_signal() {
  // Wait for the CTRL+C signal
  tokio::signal::ctrl_c()
    .await
    .expect("failed to install CTRL+C signal handler");
}
