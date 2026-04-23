use shared::{Action, ActionResponse, SpanInfo};

#[wstd::main]
async fn main() -> wstd::io::Result<()> {
    let active_span: std::rc::Rc<std::cell::RefCell<Option<SpanInfo>>> =
        std::rc::Rc::new(std::cell::RefCell::new(None));

    wash_service_helpers::run_tcp_server(1, async move |action: Action| {
        process_message(&active_span, action)
    })
    .await
}

fn process_message(
    active_span: &std::cell::RefCell<Option<SpanInfo>>,
    action: Action,
) -> ActionResponse {
    match action {
        Action::Start(span_info) => {
            *active_span.borrow_mut() = Some(span_info);
            ActionResponse::Started
        }
        Action::End => {
            let span = active_span.borrow_mut().take();
            ActionResponse::Ended(span)
        }
        Action::GetContext => {
            let span = active_span.borrow().clone();
            ActionResponse::Context(span)
        }
    }
}
