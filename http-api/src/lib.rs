mod bindings {
    wit_bindgen::generate!({ generate_all });

    use crate::Component;
    export!(Component);
}

use bindings::dewinz::component::component;
use bindings::exports::wasi::http::incoming_handler::Guest;
use bindings::wasi::http::types::{
    Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};
use otel_tracing_wasm_macro::trace;

struct Component;

impl Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let path = request.path_with_query().unwrap_or_default();
        let (status, body_text) = match path.as_str() {
            "/" => handle_home(),
            _ => (404, String::from("Not found")),
        };

        let response = OutgoingResponse::new(Fields::new());
        response.set_status_code(status).unwrap();
        let body = response.body().unwrap();
        ResponseOutparam::set(response_out, Ok(response));
        let stream = body.write().unwrap();
        stream
            .blocking_write_and_flush(body_text.as_bytes())
            .unwrap();
        drop(stream);
        OutgoingBody::finish(body, None).unwrap();
    }
}

#[trace]
fn handle_home() -> (u16, String) {
    component::first();
    component::second(&String::default());
    another_function();
    (200, String::from("Hi"))
}

#[trace]
fn another_function() {
    let _ = component::third();
}
