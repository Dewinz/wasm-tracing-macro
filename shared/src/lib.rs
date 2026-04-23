use serde::{Deserialize, Serialize};

/// Span info stored in the data service.
#[derive(Serialize, Deserialize, Clone)]
pub struct SpanInfo {
    pub trace_id: String,
    pub span_id: String,
    pub parent_span_id: String,
    pub name: String,
    pub start_seconds: u64,
    pub start_nanoseconds: u32,
}

#[derive(Serialize, Deserialize)]
pub enum Action {
    /// Store a new span in the data service.
    Start(SpanInfo),
    /// Retrieve and remove the current span, returning its SpanInfo.
    End,
    /// Get the current active span context.
    GetContext,
}

#[derive(Serialize, Deserialize)]
pub enum ActionResponse {
    Started,
    Ended(Option<SpanInfo>),
    Context(Option<SpanInfo>),
}
