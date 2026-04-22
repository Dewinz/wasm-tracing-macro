use shared::Action;
use std::{cell::Cell, rc::Rc};

#[wstd::main]
async fn main() -> wstd::io::Result<()> {
    let span_tree: Rc<Cell<String>> = Rc::new(Cell::default());
    wash_service_helpers::run_tcp_server(1, async move |action: Action| {
        process_message(&span_tree, action).await
    })
    .await
}

async fn process_message(span_tree: &Cell<String>, action: Action) -> String {
    match action {
        Action::Add => add_span(span_tree),
        Action::Remove => end_span(span_tree),
    }
}

fn add_span(_span_tree: &Cell<String>) -> String {
    String::default()
}

fn end_span(_span_tree: &Cell<String>) -> String {
    String::default()
}
