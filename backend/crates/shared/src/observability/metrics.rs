use opentelemetry::{
    KeyValue, global,
    metrics::{Counter, Gauge, Histogram, Meter},
};
use std::{
    fmt::{Display, Formatter, Result},
    time::{SystemTime, UNIX_EPOCH},
};
use sysinfo::{ProcessesToUpdate, System};

#[derive(Debug)]
pub struct SystemMetrics {
    system: System,
    pid: sysinfo::Pid,

    memory_used_mb: Gauge<f64>,
    memory_available_mb: Gauge<f64>,
    memory_usage_percent: Gauge<f64>,
    cpu_usage_percent: Gauge<f64>,
    thread_count: Gauge<i64>,
    process_uptime_seconds: Gauge<u64>,

    process_start_time: u64,
}

impl SystemMetrics {
    pub fn new() -> Self {
        let mut system = System::new();
        system.refresh_all();

        let pid = sysinfo::Pid::from(std::process::id() as usize);

        let meter = global::meter("system_metrics");

        let process_start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            system,
            pid,

            memory_used_mb: meter
                .f64_gauge("process.memory.used_mb")
                .with_description("Process memory usage in MB")
                .with_unit("MB")
                .build(),

            memory_available_mb: meter
                .f64_gauge("system.memory.available_mb")
                .with_unit("MB")
                .build(),

            memory_usage_percent: meter
                .f64_gauge("process.memory.usage_percent")
                .with_unit("%")
                .build(),

            cpu_usage_percent: meter
                .f64_gauge("system.cpu.usage_percent")
                .with_unit("%")
                .build(),

            thread_count: meter.i64_gauge("process.threads.count").build(),

            process_uptime_seconds: meter
                .u64_gauge("process.uptime_seconds")
                .with_unit("s")
                .build(),

            process_start_time,
        }
    }

    pub fn update_metrics(&mut self) {
        self.system
            .refresh_processes(ProcessesToUpdate::Some(&[self.pid]), true);
        self.system.refresh_memory();
        self.system.refresh_cpu_usage();

        if let Some(process) = self.system.process(self.pid) {
            let memory_used_mb = process.memory() as f64 / 1_048_576.0;
            self.memory_used_mb.record(memory_used_mb, &[]);

            let available_mb = self.system.available_memory() as f64 / 1_048_576.0;
            self.memory_available_mb.record(available_mb, &[]);

            let total_memory = self.system.total_memory();
            if total_memory > 0 {
                let usage_percent = (process.memory() as f64 / total_memory as f64) * 100.0;
                self.memory_usage_percent.record(usage_percent, &[]);
            }

            self.cpu_usage_percent
                .record(self.system.global_cpu_usage() as f64, &[]);

            self.thread_count
                .record(process.tasks().map(|t| t.len() as i64).unwrap_or(0), &[]);

            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            self.process_uptime_seconds
                .record(now.saturating_sub(self.process_start_time), &[]);
        }
    }
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Status {
    Success,
    Error,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = match self {
            Status::Success => "success",
            Status::Error => "error",
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug)]
pub struct Metrics {
    request_counter: Counter<u64>,
    request_duration: Histogram<f64>,
}

impl Metrics {
    pub fn new(meter: Meter) -> Self {
        let request_counter = meter
            .u64_counter("requests_total")
            .with_description("Total number of HTTP requests")
            .build();

        let request_duration = meter
            .f64_histogram("request_duration_seconds")
            .with_description("HTTP request duration in seconds")
            .with_unit("s")
            .build();

        Self {
            request_counter,
            request_duration,
        }
    }

    pub fn record(&self, method: Method, status: Status, duration_secs: f64) {
        let attributes = &[
            KeyValue::new("http.method", method.to_string()),
            KeyValue::new("http.status", status.to_string()),
        ];

        self.request_counter.add(1, attributes);
        self.request_duration.record(duration_secs, attributes);
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new(global::meter("http_status"))
    }
}

pub async fn run_metrics_collector() {
    let mut metrics = SystemMetrics::new();
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(15));

    loop {
        interval.tick().await;
        metrics.update_metrics();
    }
}
