use wstd::http::{Body, Request, Response, StatusCode};

mod bindings {
    wit_bindgen::generate!({ generate_all });
}

use bindings::wasi::otel::tracing;

type HttpResult = Result<Response<Body>, wstd::http::Error>;

#[wstd::http_server]
async fn main(req: Request<Body>) -> HttpResult {
    match req.uri().path_and_query().unwrap().as_str() {
        "/" => home(req).await,
        _ => not_found(req).await,
    }
}

async fn home(_req: Request<Body>) -> HttpResult {
    let span_context = tracing::SpanContext {
        is_remote: false,
        span_id: String::default(),
        trace_flags: tracing::TraceFlags::empty(),
        trace_id: String::default(),
        trace_state: Vec::new(),
    };
    tracing::on_start(&span_context);

    let something = tracing::outer_span_context();
    println!("{:?}", something);

    tracing::on_end(&tracing::SpanData {
        span_context,
        parent_span_id: String::default(),
        span_kind: tracing::SpanKind::Internal,
        name: String::default(),
        start_time: tracing::Datetime {
            nanoseconds: 0,
            seconds: 0,
        },
        end_time: tracing::Datetime {
            nanoseconds: 0,
            seconds: 0,
        },
        attributes: Vec::new(),
        events: Vec::new(),
        links: Vec::new(),
        status: tracing::Status::Unset,
        instrumentation_scope: tracing::InstrumentationScope {
            attributes: Vec::new(),
            name: String::default(),
            schema_url: None,
            version: None,
        },
        dropped_attributes: 0,
        dropped_events: 0,
        dropped_links: 0,
    });

    Ok(Response::new(format!("Hi").into()))
}

async fn not_found(_req: Request<Body>) -> HttpResult {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("Not found\n".into())
        .unwrap())
}
