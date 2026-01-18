use crate::primitives::http::request::Request;
use crate::primitives::http::response::Response;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, OnceLock};

pub mod init;
pub use init::init_routes;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;
pub type ControllerHandler =
    Box<dyn for<'a> Fn(&'a mut Request, &'a RouteParams) -> BoxFuture<'a, Response> + Send + Sync>;
pub type MiddlewareHandler = Box<
    dyn for<'a> Fn(
            &'a mut Request,
            &'a RouteParams,
            &'a mut Vec<Handler>,
        ) -> BoxFuture<'a, Response>
        + Send
        + Sync,
>;

#[allow(dead_code)]
pub enum HandlerKind {
    Middleware(MiddlewareHandler),
    Controller(ControllerHandler),
}

pub type Handler = Arc<HandlerKind>;

#[derive(Debug, Default)]
pub struct RouteParams {
    params: HashMap<String, String>,
}

impl RouteParams {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.params.get(key).map(|s| s.as_str())
    }
}

pub struct Route {
    pub method: &'static str,
    pub path: &'static [&'static str],
    pub handlers: Vec<Handler>,
}

impl Route {
    pub fn new(
        method: &'static str,
        path: &'static [&'static str],
        handlers: Vec<Handler>,
    ) -> Self {
        Self {
            method,
            path,
            handlers,
        }
    }
}

#[macro_export]
macro_rules! route {
    ($handler:path) => {
        std::sync::Arc::new($crate::routing::HandlerKind::Controller(Box::new(
            |req, params| Box::pin($handler(req, params)),
        )))
    };
}

#[macro_export]
macro_rules! middleware {
    ($handler:path) => {
        std::sync::Arc::new($crate::routing::HandlerKind::Middleware(Box::new(
            |req, params, handlers| Box::pin($handler(req, params, handlers)),
        )))
    };
}

static ROUTES: OnceLock<Vec<Route>> = OnceLock::new();

pub fn init(routes: Vec<Route>) {
    let _ = ROUTES.set(routes);
}

pub fn routes() -> &'static [Route] {
    ROUTES.get().map(|r| r.as_slice()).unwrap_or(&[])
}

pub async fn route(request: &mut Request) -> Response {
    let path = request.url.split('?').next().unwrap_or("");

    let segments: Vec<&str> = path
        .trim_matches('/')
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();

    let routes = routes();
    let mut path_matched = false;

    for route_def in routes {
        let params = match path_match_params(route_def.path, &segments) {
            Some(params) => params,
            None => continue,
        };
        path_matched = true;
        if route_def.method == request.method {
            let mut handlers = route_def.handlers.clone();
            handlers.reverse();
            return next_handler(request, &params, &mut handlers).await;
        }
    }

    if path_matched {
        return method_not_allowed();
    }

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/plain".to_string());
    Response {
        status_code: 404,
        headers,
        body: "Not Found".to_string(),
    }
}

pub async fn next_handler(
    request: &mut Request,
    params: &RouteParams,
    handlers: &mut Vec<Handler>,
) -> Response {
    if let Some(handler) = handlers.pop() {
        match &*handler {
            HandlerKind::Middleware(middleware) => middleware(request, params, handlers).await,
            HandlerKind::Controller(controller) => controller(request, params).await,
        }
    } else {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());
        Response {
            status_code: 500,
            headers,
            body: "Middleware chain ended without controller".to_string(),
        }
    }
}

fn path_match_params(pattern: &[&str], segments: &[&str]) -> Option<RouteParams> {
    if pattern.len() != segments.len() {
        return None;
    }

    let mut params = HashMap::new();
    for (p, s) in pattern.iter().zip(segments.iter()) {
        if let Some(name) = p.strip_prefix(':') {
            params.insert(name.to_string(), (*s).to_string());
            continue;
        }
        if p != s {
            return None;
        }
    }

    Some(RouteParams { params })
}

fn method_not_allowed() -> Response {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/plain".to_string());
    Response {
        status_code: 405,
        headers,
        body: "Method Not Allowed".to_string(),
    }
}
