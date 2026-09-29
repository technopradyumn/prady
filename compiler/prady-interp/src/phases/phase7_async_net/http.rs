// Phase 7 — HTTP Server & Client Models & Routing
// Provides request/response encapsulation and route dispatching for Prady server applications.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Options,
}

impl std::str::FromStr for HttpMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "PUT" => Ok(HttpMethod::Put),
            "DELETE" => Ok(HttpMethod::Delete),
            "PATCH" => Ok(HttpMethod::Patch),
            "OPTIONS" => Ok(HttpMethod::Options),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub query_params: HashMap<String, String>,
}

impl HttpRequest {
    pub fn new(method: HttpMethod, path: impl Into<String>) -> Self {
        Self {
            method,
            path: path.into(),
            headers: HashMap::new(),
            body: Vec::new(),
            query_params: HashMap::new(),
        }
    }

    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).to_string()
    }
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn ok(body: impl Into<Vec<u8>>) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());
        Self {
            status: 200,
            headers,
            body: body.into(),
        }
    }

    pub fn json(body_str: impl Into<String>) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        Self {
            status: 200,
            headers,
            body: body_str.into().into_bytes(),
        }
    }

    pub fn not_found(message: &str) -> Self {
        Self {
            status: 404,
            headers: HashMap::new(),
            body: message.as_bytes().to_vec(),
        }
    }

    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).to_string()
    }
}

pub type RouteHandler = fn(&HttpRequest) -> HttpResponse;

pub struct HttpRouter {
    routes: Vec<(HttpMethod, String, RouteHandler)>,
}

impl HttpRouter {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn get(&mut self, path: impl Into<String>, handler: RouteHandler) {
        self.routes.push((HttpMethod::Get, path.into(), handler));
    }

    pub fn post(&mut self, path: impl Into<String>, handler: RouteHandler) {
        self.routes.push((HttpMethod::Post, path.into(), handler));
    }

    pub fn handle(&self, req: &HttpRequest) -> HttpResponse {
        for (method, path, handler) in &self.routes {
            if method == &req.method && path == &req.path {
                return handler(req);
            }
        }
        HttpResponse::not_found("404 Route Not Found")
    }
}

impl Default for HttpRouter {
    fn default() -> Self {
        Self::new()
    }
}
