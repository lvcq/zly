use hyper::service::Service;
use hyper::{Request,Response,Body};
use std::pin::Pin;
use std::future::Future;
use std::task::{Poll,Context};

pub struct MiddlewareService
{
    req_middlewares:Vec<RequestMiddleware>
}

impl Service<Request<Body>> for MiddlewareService
{
    type Response =Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output= Result<Self::Response,Self::Error>>>>;

    fn poll_ready(&mut self,cx:&mut Context<'_>)->Poll<Result<(),Self::Error>>{
        Poll::Ready(Ok(()))
    }

    fn call(&mut self,request:Request<Body>)->Self::Future{
    
    }
}

pub type RequestMiddleware=Box<dyn Fn()+ 'static>;
