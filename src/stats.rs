use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use serde::Serialize;

#[derive(Clone)]
pub struct RequestStats {
    total: Arc<AtomicU64>,
    timestamps: Arc<Mutex<VecDeque<Instant>>>,
}

#[derive(Serialize)]
pub struct StatsResponse {
    pub total_requests: u64,
    pub requests_last_second: usize,
    pub requests_last_minute: usize,
    pub requests_last_hour: usize,
}

impl RequestStats {
    pub fn new() -> Self {
        Self {
            total: Arc::new(AtomicU64::new(0)),
            timestamps: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub async fn record(&self) {
        self.total.fetch_add(1, Ordering::Relaxed);
        let now = Instant::now();
        let mut ts = self.timestamps.lock().await;
        ts.push_back(now);
        // Keep at most 1 hour of history
        let cutoff = now - Duration::from_secs(3600);
        while ts.front().map_or(false, |t| *t < cutoff) {
            ts.pop_front();
        }
    }

    pub async fn get_stats(&self) -> StatsResponse {
        let now = Instant::now();
        let ts = self.timestamps.lock().await;
        StatsResponse {
            total_requests: self.total.load(Ordering::Relaxed),
            requests_last_second: ts.iter().filter(|t| now.duration_since(**t) < Duration::from_secs(1)).count(),
            requests_last_minute: ts.iter().filter(|t| now.duration_since(**t) < Duration::from_secs(60)).count(),
            requests_last_hour: ts.iter().filter(|t| now.duration_since(**t) < Duration::from_secs(3600)).count(),
        }
    }
}
