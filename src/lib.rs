use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};
use quote::quote;

#[proc_macro_attribute]
pub fn trace(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_function = parse_macro_input!(input as ItemFn);
    let input_function_signature = &input_function.sig;
    let input_function_block = &input_function.block;

    let input_function_name = &input_function.sig.ident;

    let expanded = quote! {
        #input_function_signature {
            use crate::bindings::wasi;

            let __generate_random_16_hex = || {
                use rand::{Rng, RngExt};
                let mut rng_generator = rand::rng();
                let hex_generator_closure = || format!("{:X}", rng_generator.random_range(0..16));
                let hex_iterator = std::iter::repeat_with(hex_generator_closure).take(16);
                hex_iterator.collect()
            };

            let __host_ctx = wasi::otel::tracing::outer_span_context();
            let __span_id = __generate_random_16_hex();
            let __trace_id = if __host_ctx.trace_id.is_empty() {
                __generate_random_16_hex()
            } else {
                __host_ctx.trace_id.clone()
            };
            let __span_context = wasi::otel::tracing::SpanContext {
                trace_id: __trace_id,
                span_id: __span_id,
                trace_flags: wasi::otel::tracing::TraceFlags::SAMPLED,
                is_remote: false,
                trace_state: Vec::new(),
            };

            wasi::otel::tracing::on_start(&__span_context);
            let __start_time = wasi::clocks::wall_clock::now();

            let result = { #input_function_block };

            let __end_time = wasi::clocks::wall_clock::now();
            wasi::otel::tracing::on_end(&wasi::otel::tracing::SpanData {
                span_context: __span_context,
                parent_span_id: __host_ctx.span_id,
                name: String::from(stringify!(#input_function_name)),
                start_time: __start_time,
                end_time: __end_time,
                span_kind: wasi::otel::tracing::SpanKind::Internal,
                attributes: Vec::new(),
                events: Vec::new(),
                links: Vec::new(),
                status: wasi::otel::tracing::Status::Ok,
                instrumentation_scope: wasi::otel::types::InstrumentationScope {
                    name: String::from(env!("CARGO_PKG_NAME")),
                    version: Some(String::from(env!("CARGO_PKG_VERSION"))),
                    schema_url: None,
                    attributes: Vec::new(),
                },
                dropped_attributes: 0,
                dropped_events: 0,
                dropped_links: 0,
            });

            result
        }
    };

    TokenStream::from(expanded)
}
