use opentelemetry::Context;
use opentelemetry::trace::{TraceContextExt, TraceId};

pub fn get_trace_id() -> Option<String> {
    let binding = Context::current();
    let span = binding.span();
    let span_ctx = span.span_context();
    let trace_id = span_ctx.trace_id();

    if trace_id != TraceId::INVALID {
        Some(trace_id.to_string())
    } else {
        None
    }
}
