use axum::{
    body::Body,
    http::{Request, Response},
    extract::ConnectInfo,
};
use std::{net::SocketAddr, task::{Context, Poll}};
use tower::{Layer, Service};
use tracing::info;

#[derive(Clone)]
pub struct LogIpLayer;

impl<S> Layer<S> for LogIpLayer {
    type Service = LogIpMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LogIpMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct LogIpMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for LogIpMiddleware<S>
where
    S: Service<Request<Body>, Response = Response<Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let ip = req
            .extensions()
            .get::<ConnectInfo<SocketAddr>>()
            .map(|info| info.0.ip().to_string())
            .unwrap_or_else(|| "unknown".into());

        let path = req.uri().path().to_string();

        info!("[{}] requesting route: \"{}\"", ip, path);

        self.inner.call(req)
    }
}
