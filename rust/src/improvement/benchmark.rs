//! Performance benchmark framework — run benchmarks, detect regressions.

use serde::{Deserialize, Serialize};

/// Result of a single benchmark run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub name: String,
    pub duration_ms: f64,
    pub timestamp: f64,
    #[serde(default)]
    pub commit: String,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

/// Alert when a benchmark regresses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionAlert {
    pub benchmark: String,
    pub baseline_ms: f64,
    pub current_ms: f64,
    pub regression_pct: f64,
    pub threshold_pct: f64,
}

/// Run a benchmark function multiple times and return median duration.
pub fn run_benchmark<F: Fn()>(name: &str, f: F, iterations: usize) -> BenchmarkResult {
    let mut durations = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        let start = std::time::Instant::now();
        f();
        let elapsed = start.elapsed().as_secs_f64() * 1000.0;
        durations.push(elapsed);
    }
    durations.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = durations[durations.len() / 2];

    BenchmarkResult {
        name: name.to_string(),
        duration_ms: (median * 1000.0).round() / 1000.0,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64(),
        commit: String::new(),
        metadata: None,
    }
}

/// Detect if current result is a regression vs historical baseline.
///
/// Uses median of historical results as baseline.
pub fn detect_regression(
    current: &BenchmarkResult,
    history: &[BenchmarkResult],
    threshold_pct: f64,
    min_history: usize,
) -> Option<RegressionAlert> {
    if history.len() < min_history {
        return None;
    }

    let mut durations: Vec<f64> = history.iter().map(|r| r.duration_ms).collect();
    durations.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let baseline = durations[durations.len() / 2];

    if baseline == 0.0 {
        return None;
    }

    let regression_pct = ((current.duration_ms - baseline) / baseline) * 100.0;

    if regression_pct > threshold_pct {
        Some(RegressionAlert {
            benchmark: current.name.clone(),
            baseline_ms: baseline,
            current_ms: current.duration_ms,
            regression_pct: (regression_pct * 10.0).round() / 10.0,
            threshold_pct,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_benchmark() {
        let result = run_benchmark("test_bench", || {
            let _ = (0..100).sum::<i32>();
        }, 3);
        assert_eq!(result.name, "test_bench");
        assert!(result.duration_ms >= 0.0);
    }

    #[test]
    fn test_detect_regression_insufficient_history() {
        let current = BenchmarkResult {
            name: "bench".into(),
            duration_ms: 100.0,
            timestamp: 0.0,
            commit: String::new(),
            metadata: None,
        };
        assert!(detect_regression(&current, &[], 10.0, 3).is_none());
    }

    #[test]
    fn test_detect_regression_found() {
        let current = BenchmarkResult {
            name: "bench".into(),
            duration_ms: 150.0,
            timestamp: 0.0,
            commit: String::new(),
            metadata: None,
        };
        let history: Vec<BenchmarkResult> = (0..5)
            .map(|_| BenchmarkResult {
                name: "bench".into(),
                duration_ms: 100.0,
                timestamp: 0.0,
                commit: String::new(),
                metadata: None,
            })
            .collect();
        let alert = detect_regression(&current, &history, 10.0, 3);
        assert!(alert.is_some());
        assert_eq!(alert.unwrap().regression_pct, 50.0);
    }

    #[test]
    fn test_detect_regression_within_threshold() {
        let current = BenchmarkResult {
            name: "bench".into(),
            duration_ms: 105.0,
            timestamp: 0.0,
            commit: String::new(),
            metadata: None,
        };
        let history: Vec<BenchmarkResult> = (0..5)
            .map(|_| BenchmarkResult {
                name: "bench".into(),
                duration_ms: 100.0,
                timestamp: 0.0,
                commit: String::new(),
                metadata: None,
            })
            .collect();
        assert!(detect_regression(&current, &history, 10.0, 3).is_none());
    }
}
