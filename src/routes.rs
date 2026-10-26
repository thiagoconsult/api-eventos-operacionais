pub fn route(method: &str, path: &str) -> HttpResponse {
    match (method, path) {
        (method, path) if method == "GET" && path == "/health" => {
            HttpResponse::new(HttpStatus::Ok, "application/json", "{\"status\":\"ok\"}")
        }
        (method, _) if method != "GET" => HttpResponse::new(
            HttpStatus::MethodNotAllowed,
            "application/json",
            "{\"status\":\"Method Not Allowed\"}",
        ),
        _ => HttpResponse::new(
            HttpStatus::NotFound,
            "application/json",
            "{\"status\":\"Not Found\"}",
        ),
    }
}

#[derive(Debug)]
pub enum HttpStatus {
    MethodNotAllowed,
    NotFound,
    Ok,
}
impl HttpStatus {
    fn as_str(&self) -> &str {
        match self {
            HttpStatus::MethodNotAllowed => "HTTP/1.1 405 METHOD NOT ALLOWED",
            HttpStatus::NotFound => "HTTP/1.1 404 NOT FOUND",
            HttpStatus::Ok => "HTTP/1.1 200 OK",
        }
    }
}
#[derive(Debug)]
pub struct HttpResponse {
    status_http: HttpStatus,
    content_type: String,
    body: String,
}
impl HttpResponse {
    pub fn new(status_http: HttpStatus, content_type: &str, body: &str) -> Self {
        Self {
            status_http,
            content_type: content_type.to_owned(),
            body: body.to_owned(),
        }
    }
    pub fn to_http(&self) -> String {
        let status_http = &self.status_http.as_str();
        let content_type = &self.content_type;
        let body = &self.body;
        let body_size: usize = body.len();

        format!(
            "{status_http}\r\nContent-type: {content_type}\r\nContent-Length: {body_size}\r\nConnection: close\r\n\r\n{body}"
        )
    }
}
