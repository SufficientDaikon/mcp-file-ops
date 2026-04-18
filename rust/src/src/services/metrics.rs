use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

pub struct Metrics {
    pub requests_total: Arc<AtomicU64>,
    pub requests_success: Arc<AtomicU64>,
    pub requests_error: Arc<AtomicU64>,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            requests_total: Arc::new(AtomicU64::new(0)),
            requests_success: Arc::new(AtomicU64::new(0)),
            requests_error: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn record_request_success(&self) {
        self.requests_total.fetch_add(1, Ordering::Relaxed);
        self.requests_success.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_request_error(&self) {
        self.requests_total.fetch_add(1, Ordering::Relaxed);
        self.requests_error.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_latency(&self, _method: &str, _ms: u64) {
        // Implement latency tracking if needed
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}
