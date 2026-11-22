//! Diagnostics and metrics instrumentation for Tiny68.

use std::time::Instant;

pub fn init_logging() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info,tiny68=debug")),
        )
        .try_init();
}

/// Real-time frame metrics collector.
#[derive(Debug, Clone)]
pub struct FrameMetrics {
    pub fps: f32,
    pub avg_frame_time_ms: f32,
    pub p95_frame_time_ms: f32,
    pub p99_frame_time_ms: f32,
    pub simulation_time_ms: f32,
    pub render_time_ms: f32,
    pub active_characters: usize,
    pub sleeping_characters: usize,
    pub private_memory_kb: u64,
}

impl Default for FrameMetrics {
    fn default() -> Self {
        Self {
            fps: 0.0,
            avg_frame_time_ms: 0.0,
            p95_frame_time_ms: 0.0,
            p99_frame_time_ms: 0.0,
            simulation_time_ms: 0.0,
            render_time_ms: 0.0,
            active_characters: 0,
            sleeping_characters: 0,
            private_memory_kb: 0,
        }
    }
}

pub struct MetricsTracker {
    frame_times: Vec<f32>,
    max_samples: usize,
    frame_count: u32,
    fps_timer: Instant,
    pub current_fps: f32,
}

impl MetricsTracker {
    pub fn new(max_samples: usize) -> Self {
        Self {
            frame_times: Vec::with_capacity(max_samples),
            max_samples,
            frame_count: 0,
            fps_timer: Instant::now(),
            current_fps: 0.0,
        }
    }

    pub fn record_frame(&mut self, elapsed_ms: f32) {
        if self.frame_times.len() >= self.max_samples {
            self.frame_times.remove(0);
        }
        self.frame_times.push(elapsed_ms);

        self.frame_count += 1;
        let fps_elapsed = self.fps_timer.elapsed().as_secs_f32();
        if fps_elapsed >= 0.5 {
            self.current_fps = (self.frame_count as f32) / fps_elapsed;
            self.frame_count = 0;
            self.fps_timer = Instant::now();
        }
    }

    pub fn compute_percentiles(&self) -> (f32, f32, f32) {
        if self.frame_times.is_empty() {
            return (0.0, 0.0, 0.0);
        }
        let mut sorted = self.frame_times.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let sum: f32 = sorted.iter().sum();
        let avg = sum / (sorted.len() as f32);

        let p95_idx = ((sorted.len() as f32) * 0.95) as usize;
        let p99_idx = ((sorted.len() as f32) * 0.99) as usize;

        let p95 = sorted[p95_idx.min(sorted.len() - 1)];
        let p99 = sorted[p99_idx.min(sorted.len() - 1)];

        (avg, p95, p99)
    }
}
