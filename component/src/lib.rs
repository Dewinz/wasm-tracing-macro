mod bindings {
    wit_bindgen::generate!({ generate_all });

    use crate::Component;
    export!(Component);
}

use bindings::exports::dewinz::component::component::Guest;
use otel_tracing_wasm_macro::trace;

struct Component;

impl Guest for Component {
    #[trace]
    fn first() {}

    #[trace]
    fn second(_something: String) {}

    #[trace]
    fn third() -> Result<String, String> {
        Ok(String::default())
    }
}
