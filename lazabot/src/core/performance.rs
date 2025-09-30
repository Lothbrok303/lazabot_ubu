use std::time::{Duration, Instant};
use tracing::{debug, info, warn};

/// Performance monitoring utility for tracking operation latencies and metrics
#[derive(Debug, Clone)]
pub struct PerformanceMonitor {
    start_time: Option<Instant>,
    operation_name: String,
}

impl PerformanceMonitor {
    /// Create a new performance monitor for a specific operation
    pub fn new(operation_name: &str) -> Self {
        Self {
            start_time: None,
            operation_name: operation_name.to_string(),
        }
    }

    /// Start timing an operation
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        debug!("Started timing operation: {}", self.operation_name);
    }

    /// End timing and log the duration
    pub fn end(&mut self) -> Duration {
        if let Some(start) = self.start_time {
            let duration = start.elapsed();
            info!(
                "Operation '{}' completed in {:?}",
                self.operation_name, duration
            );
            self.start_time = None;
            duration
        } else {
            warn!(
                "Attempted to end timing for '{}' but it was never started",
                self.operation_name
            );
            Duration::ZERO
        }
    }

    /// Get the current elapsed time without ending the timer
    pub fn elapsed(&self) -> Option<Duration> {
        self.start_time.map(|start| start.elapsed())
    }

    /// Check if timing is currently active
    pub fn is_timing(&self) -> bool {
        self.start_time.is_some()
    }

    /// Reset the timer
    pub fn reset(&mut self) {
        self.start_time = None;
        debug!("Reset timer for operation: {}", self.operation_name);
    }
}

/// Macro for easy performance monitoring
#[macro_export]
macro_rules! monitor_performance {
    ($name:expr, $block:block) => {{
        let mut monitor = PerformanceMonitor::new($name);
        monitor.start();
        let result = $block;
        monitor.end();
        result
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_performance_monitor() {
        let mut monitor = PerformanceMonitor::new("test_operation");

        // Test timing
        monitor.start();
        thread::sleep(Duration::from_millis(10));
        let duration = monitor.end();

        assert!(duration.as_millis() >= 10);
        assert!(!monitor.is_timing());
    }

    #[test]
    fn test_elapsed_time() {
        let mut monitor = PerformanceMonitor::new("test_elapsed");
        monitor.start();
        thread::sleep(Duration::from_millis(5));

        let elapsed = monitor.elapsed().unwrap();
        assert!(elapsed.as_millis() >= 5);

        monitor.end();
    }

    #[test]
    fn test_reset() {
        let mut monitor = PerformanceMonitor::new("test_reset");
        monitor.start();
        monitor.reset();
        assert!(!monitor.is_timing());
    }
}
