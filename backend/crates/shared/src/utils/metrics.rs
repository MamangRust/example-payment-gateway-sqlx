use opentelemetry::{
    KeyValue, global,
    metrics::{Counter, Gauge, Histogram},
};
use std::fmt::{Display, Formatter};
use std::{
    fmt, fs,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use sysinfo::System;

fn get_thread_count(pid: usize) -> Option<i64> {
    let path = format!("/proc/{pid}/status");
    if let Ok(contents) = fs::read_to_string(path) {
        for line in contents.lines() {
            if line.starts_with("Threads:") && line.split_whitespace().nth(1).is_some() {
                return line
                    .split_whitespace()
                    .nth(1)
                    .and_then(|thread_count| thread_count.parse::<i64>().ok());
            }
        }
    }
    None
}

#[derive(Debug, Clone)]
pub struct SystemMetrics {
    memory_alloc_bytes: Gauge<i64>,
    memory_sys_bytes: Gauge<i64>,
    available_memory: Counter<u64>,
    thread_usage: Gauge<i64>,
    total_cpu_usage: Counter<f64>,
    _process_start_time: Gauge<u64>,
}

impl SystemMetrics {
    pub fn new() -> Self {
        let meter = global::meter("system_metrics");

        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let memory_alloc_bytes = meter
            .i64_gauge("process.memory.alloc.bytes")
            .with_description("Current memory allocation in bytes")
            .with_unit("bytes")
            .build();

        let memory_sys_bytes = meter
            .i64_gauge("process.memory.sys.bytes")
            .with_description("Total system memory in bytes")
            .with_unit("bytes")
            .build();

        let available_memory = meter
            .u64_counter("process.memory.available.total")
            .with_description("Total available memory")
            .with_unit("bytes")
            .build();

        let thread_usage = meter
            .i64_gauge("process.threads.total")
            .with_description("Thread total")
            .build();

        let total_cpu_usage = meter
            .f64_counter("system.cpu.usage.total")
            .with_description("Total CPU usage")
            .with_unit("percent")
            .build();

        let process_start_time = meter
            .u64_gauge("process.start_time.seconds")
            .with_description("Start time of the process since unix epoch in seconds")
            .with_unit("s")
            .build();

        process_start_time.record(start_time, &[]);

        Self {
            memory_alloc_bytes,
            memory_sys_bytes,
            available_memory,
            thread_usage,
            total_cpu_usage,
            _process_start_time: process_start_time,
        }
    }

    pub async fn update_metrics(&self) {
        let mut sys = System::new_all();
        sys.refresh_all();

        let pid = std::process::id() as usize;

        if let Some(process) = sys.process(sysinfo::Pid::from(pid)) {
            let current_memory = process.memory() as i64;
            self.memory_alloc_bytes.record(current_memory, &[]);
            self.memory_sys_bytes
                .record(process.virtual_memory() as i64, &[]);

            let available_memory = sys.available_memory() / 1_024;
            self.available_memory.add(available_memory, &[]);

            let total_cpu_usage = sys.global_cpu_usage() as f64;
            self.total_cpu_usage.add(total_cpu_usage, &[]);

            if let Some(thread_count) = get_thread_count(pid) {
                self.thread_usage.record(thread_count, &[]);
            }
        }
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Status::Success => "success",
            Status::Error => "error",
        };
        write!(f, "{}", s)
    }
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct Metrics {
    request_counter: Counter<u64>,
    request_duration: Histogram<f64>,
}

impl Metrics {
    pub fn new() -> Self {
        let meter = global::meter("http_metrics");

        let request_counter = meter
            .u64_counter("http.server.requests.total")
            .with_description("Total number of HTTP requests")
            .build();

        let request_duration = meter
            .f64_histogram("http.server.request.duration")
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
        Self::new()
    }
}

pub async fn run_metrics_collector(system_metrics: Arc<SystemMetrics>) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(15));
    loop {
        interval.tick().await;
        system_metrics.update_metrics().await;
    }
}
