use zformdata::FormValue;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;

pub mod session;
pub mod middleware;

use session::{Session,SessionConfig};
use middleware::MiddlewareService;
async fn handle_connect(req: Request<Body>) -> Result<Response<Body>, Infallible> {
  let mut response = Response::new(Body::empty());

  match (req.method(), req.uri().path()) {
    (&Method::GET, "/") => {
      *response.body_mut() = Body::from("Try POSTing data to /echo");
    }
    (&Method::GET, "/validate-logon")=>{
       *response.body_mut()=Body::from("next"); 
    }
    (&Method::POST, "/file/upload") => {
      let fv:FormValue = zformdata::read_formdata(req).await;
      println!("form value:{:?}", fv);
      *response.body_mut() = Body::from("111");
    }
    _ => {
      *response.status_mut() = StatusCode::NOT_FOUND;
    }
  };

  Ok(response)
}



pub struct ZHttp{
    port:u16,
    session: Option<Session>
}

impl ZHttp{
    pub fn new(port:u16)->ZHttp{
        ZHttp{
            port,
            session:None,
        }
    }

    pub fn session_redis(mut self,config:SessionConfig)->Self{
        let zly_session = Session::new(config);
        self.session = Some(zly_session);
        return self;
    }

    #[tokio::main]
    pub async fn start_server(&self){
        let addr = SocketAddr::from(([127,0,0,1],self.port));
        let make_svc = make_service_fn(|_conn| {
            async {
                Ok::<_, Infallible>(service_fn(|req| async {
                    let response=handle_connect(req).await?;
                    Ok::<Response<Body>,Infallible>(response)
                }))
            }
        });
        let server =Server::bind(&addr).serve(make_svc);
        // 程序关闭处理信号
        let graceful = server.with_graceful_shutdown(shutdown_signal());
        if let Err(e) = graceful.await {
            println!("server error: {}",e);
        }
    }
}

async fn shutdown_signal() {
  // Wait for the CTRL+C signal
  tokio::signal::ctrl_c()
    .await
    .expect("failed to install CTRL+C signal handler");
}

