use hyper::Server;
use std::net::SocketAddr;

pub mod cookie;
pub mod middleware;
pub mod middleware_service;
pub mod router;
pub mod session;
pub mod session_middleware;

use middleware_service::MakeSvc;
use router::Router;
use session::SessionConfig;
use session_middleware::SessionMiddleware;

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
  pub async fn start_server(&self, router: Router) {
    let addr = SocketAddr::from(([192, 168, 164, 129], self.port));
    let sess_mi = SessionMiddleware::new(self.session_config.as_ref().unwrap().clone());
    let server = Server::bind(&addr).serve(MakeSvc::new(sess_mi, router));
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
