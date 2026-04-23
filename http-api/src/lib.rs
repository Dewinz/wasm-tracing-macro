use wstd::http::{Body, Request, Response};
use otel_tracing_wasm_macro::trace;

mod bindings {
    wit_bindgen::generate!({ generate_all });
}

use bindings::dewinz::component::component;

type HttpResult = Result<Response<Body>, wstd::http::Error>;

#[wstd::http_server]
async fn main(req: Request<Body>) -> HttpResult {
    match req.uri().path() {
        "/" => home(req).await,
        _ => not_found(req).await,
    }
}

#[trace]
async fn home(_req: Request<Body>) -> HttpResult {
    component::first();

    component::second(&String::default());

    another_function();

    Ok(Response::new(String::from("Hi").into()))
}

fn another_function() {
    let something = component::third();
    println!("{:?}", something);
}

async fn not_found(_req: Request<Body>) -> HttpResult {
    Ok(Response::new(String::from("Not found").into()))
}
