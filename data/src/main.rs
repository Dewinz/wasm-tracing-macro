use std::{cell::Cell, rc::Rc};
use shared::Action;

#[wstd::main]
async fn main() -> wstd::io::Result<()> {
    let span_tree: Rc<Cell<String>> = Rc::new(Cell::default());
    wash_service_helpers::run_tcp_server(8080, async move |action: Action| {
        process_message(&span_tree , action).await
    })
    .await
}

async fn process_message(span_tree: &Cell<String>, action: Action) -> String {
    match action {
        Action::Add => add_span(counter),
        Action::Remove => update_counter_value(counter, direction),
    }
}

fn add_span(span_tree: &Cell<String>)

fn get_counter_value(counter: &Cell<String>) -> String {
    counter.get()
}

fn update_counter_value(counter: &Cell<String>) -> i32 {
}
