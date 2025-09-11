use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_sdk::logs::SdkLoggerProvider;
use tracing_appender::{
    non_blocking,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub struct Logger;

impl Logger {
    pub fn new(component: &str, is_dev: bool) -> Self {
        let log_dir = if is_dev { "./logs" } else { "/var/log/app" };
        let file_name = format!("rust_app_{component}.log");
        let file_appender = RollingFileAppender::new(Rotation::DAILY, log_dir, file_name);
        let (file_writer, guard) = non_blocking(file_appender);

        let file_layer = fmt::layer()
            .with_writer(file_writer)
            .with_ansi(false)
            .json()
            .with_filter(EnvFilter::new("info"));

        let console_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("off"));

        let console_layer = fmt::layer()
            .pretty()
            .with_thread_names(true)
            .with_ansi(true)
            .with_filter(console_filter);

        let registry = tracing_subscriber::registry()
            .with(file_layer)
            .with(console_layer);

        let sdk_logger_provider = if !is_dev {
            Some(SdkLoggerProvider::builder().build())
        } else {
            None
        };

        if let Some(provider) = &sdk_logger_provider {
            let otel_filter = EnvFilter::new("info")
                .add_directive("hyper=off".parse().unwrap())
                .add_directive("opentelemetry=off".parse().unwrap())
                .add_directive("tonic=off".parse().unwrap())
                .add_directive("h2=off".parse().unwrap())
                .add_directive("reqwest=off".parse().unwrap());

            registry
                .with(OpenTelemetryTracingBridge::new(provider).with_filter(otel_filter))
                .init();
        } else {
            registry.init();
        }

        std::mem::forget(guard);

        Self
    }
}
