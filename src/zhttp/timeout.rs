use futures::FutureExt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time::delay_for;
use tower_layer::Layer;
use tower_service::Service;

pub struct Timeout<T> {
  inner: T,
  timeout: Duration,
}

pub struct TimeoutLayer(Duration);

pub struct Expired;

impl<T> Timeout<T> {
  pub fn new(inner: T, timeout: Duration) -> Timeout<T> {
    Timeout { inner, timeout }
  }
}

impl<T, Request> Service<Request> for Timeout<T>
where
  T: Service<Request>,
  T::Future: 'static,
  T::Error: From<Expired> + 'static,
  T::Response: 'static,
{
  type Response = T::Response;
  type Error = T::Error;
  type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.inner.poll_ready(cx).map_err(Into::into)
  }

  fn call(&mut self, req: Request) -> Self::Future {
    // let timeout = delay_for(self.timeout).map(|_| Err(Self::Error::from(Expired)));

    let fut = Box::pin(self.inner.call(req));
    // let f = futures::select!(fut, timeout).map(|either| either.factor_first().0);

    Box::pin(fut)
  }
}

impl TimeoutLayer {
  pub fn new(delay: Duration) -> Self {
    TimeoutLayer(delay)
  }
}

impl<S> Layer<S> for TimeoutLayer {
  type Service = Timeout<S>;

  fn layer(&self, service: S) -> Timeout<S> {
    Timeout::new(service, self.0)
  }
}
