use rand::Rng;
use shared::{Action, ActionResponse, SpanInfo};
use wash_service_helpers::send_message;

mod bindings {
    wit_bindgen::generate!({
        generate_all,
    });

    use crate::Component;
    export!(Component);
}

use bindings::exports::dewinz::otel::tracing::Guest;
use bindings::wasi::otel::tracing;

struct Component;

impl Guest for Component {
    fn on_start() {
        wstd::runtime::block_on(async {
            // Get the host's current span context to use as parent.
            let host_ctx = tracing::outer_span_context();

            // Generate a new span ID for this span.
            let span_id = generate_hex_id(16);

            // Use the host's trace ID if available, otherwise generate one.
            let trace_id = if host_ctx.trace_id.is_empty() {
                generate_hex_id(32)
            } else {
                host_ctx.trace_id
            };

            let now = bindings::wasi::clocks::wall_clock::now();

            let span_info = SpanInfo {
                trace_id,
                span_id,
                parent_span_id: host_ctx.span_id,
                name: String::from("wasm-span"),
                start_seconds: now.seconds,
                start_nanoseconds: now.nanoseconds,
            };

            send_message::<ActionResponse, _>(1, Action::Start(span_info))
                .await
                .expect("Failed to store span in data service");
        });
    }

    fn on_end() {
        wstd::runtime::block_on(async {
            let response = send_message::<ActionResponse, _>(1, Action::End)
                .await
                .expect("Failed to retrieve span from data service");

            if let ActionResponse::Ended(Some(span_info)) = response {
                let now = bindings::wasi::clocks::wall_clock::now();

                // Forward the complete span data to the host for OTLP export.
                tracing::on_end(&tracing::SpanData {
                    span_context: tracing::SpanContext {
                        trace_id: span_info.trace_id,
                        span_id: span_info.span_id,
                        trace_flags: tracing::TraceFlags::SAMPLED,
                        is_remote: false,
                        trace_state: Vec::new(),
                    },
                    parent_span_id: span_info.parent_span_id,
                    span_kind: tracing::SpanKind::Internal,
                    name: span_info.name,
                    start_time: tracing::Datetime {
                        seconds: span_info.start_seconds,
                        nanoseconds: span_info.start_nanoseconds,
                    },
                    end_time: tracing::Datetime {
                        seconds: now.seconds,
                        nanoseconds: now.nanoseconds,
                    },
                    attributes: Vec::new(),
                    events: Vec::new(),
                    links: Vec::new(),
                    status: tracing::Status::Ok,
                    instrumentation_scope: tracing::InstrumentationScope {
                        name: String::from("dewinz-otel"),
                        version: Some(String::from("0.1.0")),
                        schema_url: None,
                        attributes: Vec::new(),
                    },
                    dropped_attributes: 0,
                    dropped_events: 0,
                    dropped_links: 0,
                });
            }
        });
    }

    fn get_span_context() -> tracing::SpanContext {
        wstd::runtime::block_on(async {
            let response = send_message::<ActionResponse, _>(1, Action::GetContext)
                .await
                .expect("Failed to get span context from data service");

            match response {
                ActionResponse::Context(Some(span_info)) => tracing::SpanContext {
                    trace_id: span_info.trace_id,
                    span_id: span_info.span_id,
                    trace_flags: tracing::TraceFlags::SAMPLED,
                    is_remote: false,
                    trace_state: Vec::new(),
                },
                _ => tracing::outer_span_context(),
            }
        })
    }
}

/// Generate a random hex string of the given length.
fn generate_hex_id(len: usize) -> String {
    let mut rng = rand::rng();
    let hex_char = || format!("{:x}", rng.random_range(0..16u8));
    std::iter::repeat_with(hex_char).take(len).collect()
}
