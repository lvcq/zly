use zformdata::FormValue;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn handle_connect(req: Request<Body>) -> Result<Response<Body>, Infallible> {
  let mut response = Response::new(Body::empty());

  match (req.method(), req.uri().path()) {
    (&Method::GET, "/") => {
      *response.body_mut() = Body::from("Try POSTing data to /echo");
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

async fn shutdown_signal() {
  // Wait for the CTRL+C signal
  tokio::signal::ctrl_c()
    .await
    .expect("failed to install CTRL+C signal handler");
}

#[tokio::main]
pub async fn start_server() {
  // We'll bind to 127.0.0.1:3000
  let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

  let make_svc = make_service_fn(|_conn| {
    async {
      // service_fn converts our function into a `Service`
      Ok::<_, Infallible>(service_fn(handle_connect))
    }
  });
  let server = Server::bind(&addr).serve(make_svc);

  // And now add a graceful shutdown signal...
  let graceful = server.with_graceful_shutdown(shutdown_signal());

  // Run this server for... forever!
  if let Err(e) = graceful.await {
    eprintln!("server error: {}", e);
  }
}
