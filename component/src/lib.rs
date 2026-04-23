mod bindings {
    wit_bindgen::generate!({ generate_all });

    use crate::Component;
    export!(Component);
}

use bindings::exports::dewinz::component::component::Guest;

struct Component;

impl Guest for Component {
    fn first() {
    }

    fn second(_something: String) {
    }

    fn third() -> Result<String, String> {
        Ok(String::default())
    }
}
