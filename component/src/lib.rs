use shared::Action;
use wash_service_helpers::send_message;

mod bindings {
    wit_bindgen::generate!({
        generate_all,
    });

    use crate::Tracing;
    export!(Tracing);
}

use bindings::exports::dewinz::otel::tracing::Guest;
use bindings::wasi::otel::tracing;

struct Tracing;

impl Guest for Tracing {
    fn on_start(_context: tracing::SpanContext) {
        wstd::runtime::block_on(async {
            let value = send_message::<String, _>(1, Action::Add)
                .await
                .expect("Couldn't invoke service for wasi:otel/tracing.on_start");
            eprintln!("{:?}", value);
        });
    }

    fn on_end(span: tracing::SpanData) {
        tracing::on_end(&span)
    }

    fn outer_span_context() -> tracing::SpanContext {
        tracing::outer_span_context()
    }
}
