use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

pub fn tracing() {
    let fmt_layer = fmt::layer()
        .without_time()
        .with_line_number(true)
        .with_level(true)
        .with_target(true);

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("trace"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}
