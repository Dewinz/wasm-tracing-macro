use serde::{Deserialize, Serialize};
use wash_service_helpers::send_message;
use shared::Action;

mod bindings {
    wit_bindgen::generate!({
        generate_all,
    });

    use crate::Tracing;
    export!(Tracing);
}

use bindings::exports::wasi::otel::tracing;

struct Tracing;

impl tracing::Guest for Tracing {
    fn on_start(_context: tracing::SpanContext) {
        wstd::runtime::block_on(async {
            let value = send_message::<String, _>(0, Action::Add)
                .await
                .unwrap();
            println!("{}", value);
        });
    }

    fn on_end(_span: tracing::SpanData) {
        wstd::runtime::block_on(async {
            let value = send_message::<String, _>(0, Action::Remove)
                .await
                .unwrap();
            println!("{}", value);
        });
    }

    /// This is supposed to facilitate the connection of the wasmcloud-runtime traces to the traces
    /// we define in our WASM component. This is currently not possible as we'd need a function to
    /// call from wasmCloud to get the SpanContext. For now it returns a default value for fault
    /// tolerance.
    fn outer_span_context() -> tracing::SpanContext {
        eprintln!("wasi:otel/tracing.outer-span-context function was called, but is not defined in WASM component");

        tracing::SpanContext {
            is_remote: false,
            span_id: String::from(""),
            trace_flags: tracing::TraceFlags::empty(),
            trace_id: String::from(""),
            trace_state: Vec::new(),
        }
    }
}
