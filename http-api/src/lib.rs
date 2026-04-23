use wstd::http::{Body, Request, Response};

mod bindings {
    wit_bindgen::generate!({ generate_all });
}

use bindings::dewinz::otel::tracing;

type HttpResult = Result<Response<Body>, wstd::http::Error>;

#[wstd::http_server]
async fn main(req: Request<Body>) -> HttpResult {
    match req.uri().path() {
        "/" => home(req).await,
        _ => not_found(req).await,
    }
}

async fn home(_req: Request<Body>) -> HttpResult {
    tracing::on_start();

    // ... do work ...

    tracing::on_end();

    Ok(Response::new(String::from("Hi").into()))
}

async fn not_found(_req: Request<Body>) -> HttpResult {
    Ok(Response::new(String::from("Not found").into()))
}
