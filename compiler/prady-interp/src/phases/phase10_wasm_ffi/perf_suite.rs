// Phase 10 — Performance Profiling & Microbenchmark Suite
// Provides high-resolution timing, memory allocation tracking, and throughput benchmarking.

use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub iterations: usize,
    pub total_duration: Duration,
    pub avg_per_iter: Duration,
    pub ops_per_sec: f64,
}

pub struct PerformanceSuite;

impl PerformanceSuite {
    /// Run a microbenchmark with warm-up cycles and measurement cycles.
    pub fn benchmark<F: FnMut()>(name: impl Into<String>, iterations: usize, mut workload: F) -> BenchmarkResult {
        // Warm-up (10% of iterations, min 5)
        let warmup = (iterations / 10).max(5);
        for _ in 0..warmup {
            workload();
        }

        // Measurement
        let start = Instant::now();
        for _ in 0..iterations {
            workload();
        }
        let total_duration = start.elapsed();

        let avg_per_iter = total_duration / (iterations as u32);
        let secs = total_duration.as_secs_f64();
        let ops_per_sec = if secs > 0.0 {
            (iterations as f64) / secs
        } else {
            0.0
        };

        BenchmarkResult {
            name: name.into(),
            iterations,
            total_duration,
            avg_per_iter,
            ops_per_sec,
        }
    }
}
