extern crate actix_web;

use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Result};

pub fn index(_request: &HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/index.html")))
}
