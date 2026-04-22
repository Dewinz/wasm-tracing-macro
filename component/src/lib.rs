use wash_service_helpers::send_message;
use serde::{Deserialize, Serialize};

mod bindings {
    wit_bindgen::generate!({
        generate_all,
    });

    use crate::Tracing;
    export!(Tracing);
}

use bindings::exports::wasi::otel::tracing;

struct Tracing;

#[derive(Deserialize, Serialize, Default)]
struct Something {
    value: String
}

impl tracing::Guest for Tracing {
    fn on_start(_context: tracing::SpanContext) {
        wstd::runtime::block_on(async {
            send_message::<(), _>(0, Something::default()).await.unwrap();
        });
    }

    fn on_end(_span: tracing::SpanData) {
        wstd::runtime::block_on(async {
            send_message::<(), _>(0, Something::default()).await.unwrap();
        });
    }

    fn outer_span_context() -> tracing::SpanContext {
        wstd::runtime::block_on(async {
            send_message::<(), _>(0, Something::default()).await.unwrap();
        });

        tracing::SpanContext {
            is_remote: false,
            span_id: String::from(""),
            trace_flags: tracing::TraceFlags::empty(),
            trace_id: String::from(""),
            trace_state: Vec::new(),
        }
    }
}
