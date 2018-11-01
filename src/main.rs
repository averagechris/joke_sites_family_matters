extern crate actix_web;

use std::env;

use actix_web::http::StatusCode;
use actix_web::{fs, server, App, HttpRequest, HttpResponse, Result};

fn index(_request: &HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}

fn main() {
    let dev_ipaddress = "127.0.0.1:8088";
    let prod_ipaddress = "0.0.0.0:80";
    let application_server = server::new(|| {
        App::new().resource("/", |r| r.f(index)).handler(
            "/static",
            fs::StaticFiles::new("../static").unwrap().show_files_listing(),
        )
    });
    let application_server = match env::var_os("FAMILY_MATTERS_DEV") {
        Some(_) => application_server.bind(dev_ipaddress),
        _ => application_server.bind(prod_ipaddress),
    };

    application_server.unwrap().run();
}
