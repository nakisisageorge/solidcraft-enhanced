//! SolidCraft Benchmark Suite with Regression Detection
//!
//! Provides benchmarks for critical CAD operations with regression detection
//! and historical tracking for CI/CD integration.

pub mod benches;
pub mod regression;
pub mod runner;
pub mod storage;

use criterion::{Criterion, BenchmarkId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

/// Benchmark categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkCategory {
    ConstraintSolver,
    BooleanOperations,
    ImportExport,
    MeshGeneration,
    ParametricRegeneration,
    WasmBuild,
    WasmRuntime,
}

/// Benchmark configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    pub category: BenchmarkCategory,
    pub name: String,
    pub description: String,
    pub sample_size: usize,
    pub measurement_time_secs: u64,
    pub warmup_time_secs: u64,
    pub regression_threshold_percent: f64,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            category: BenchmarkCategory::ConstraintSolver,
            name: "default".to_string(),
            description: "Default benchmark".to_string(),
            sample_size: 100,
            measurement_time_secs: 10,
            warmup_time_secs: 3,
            regression_threshold_percent: 10.0,
        }
    }
}

/// Benchmark result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub id: String,
    pub category: BenchmarkCategory,
    pub name: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub git_commit: Option<String>,
    pub git_branch: Option<String>,
    pub mean_ns: f64,
    pub median_ns: f64,
    pub std_dev_ns: f64,
    pub min_ns: f64,
    pub max_ns: f64,
    pub sample_size: usize,
    pub throughput: Option<f64>,
    pub metadata: HashMap<String, String>,
}

/// Regression detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionResult {
    pub benchmark_id: String,
    pub current_mean_ns: f64,
    pub baseline_mean_ns: f64,
    pub change_percent: f64,
    pub is_regression: bool,
    pub threshold_percent: f64,
    pub severity: RegressionSeverity,
}

/// Regression severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RegressionSeverity {
    None,
    Minor,
    Major,
    Critical,
}

/// Error type for benchmark operations
#[derive(Error, Debug)]
pub enum BenchmarkError {
    #[error("Benchmark execution failed: {0}")]
    Execution(String),
    #[error("Storage error: {0}")]
    Storage(String),
    #[error("Configuration error: {0}")]
    Configuration(String),
    #[error("Regression detected: {0}")]
    Regression(String),
}

/// Run a single benchmark
pub fn run_benchmark(
    c: &mut Criterion,
    config: &BenchmarkConfig,
    bench_fn: impl FnMut() + 'static,
) -> BenchmarkResult {
    let mut group = c.benchmark_group(&config.name);
    group.sample_size(config.sample_size);
    group.measurement_time(std::time::Duration::from_secs(config.measurement_time_secs));
    group.warm_up_time(std::time::Duration::from_secs(config.warmup_time_secs));

    let id = uuid::Uuid::new_v4().to_string();
    let start = std::time::Instant::now();

    group.bench_with_input(BenchmarkId::new(&config.name, ""), &(), |b, _| {
        b.iter(bench_fn);
    });

    let elapsed = start.elapsed();

    // Get statistics from criterion
    let stats = group.benchmarks().values().next().unwrap().stats();

    BenchmarkResult {
        id,
        category: config.category,
        name: config.name.clone(),
        timestamp: chrono::Utc::now(),
        git_commit: get_git_commit(),
        git_branch: get_git_branch(),
        mean_ns: stats.mean.as_secs_f64() * 1e9,
        median_ns: stats.median.as_secs_f64() * 1e9,
        std_dev_ns: stats.std_dev.as_secs_f64() * 1e9,
        min_ns: stats.min.as_secs_f64() * 1e9,
        max_ns: stats.max.as_secs_f64() * 1e9,
        sample_size: config.sample_size,
        throughput: None,
        metadata: HashMap::new(),
    }
}

/// Detect regression by comparing with baseline
pub fn detect_regression(
    current: &BenchmarkResult,
    baseline: &BenchmarkResult,
    threshold_percent: f64,
) -> RegressionResult {
    let change_percent = ((current.mean_ns - baseline.mean_ns) / baseline.mean_ns) * 100.0;
    let is_regression = change_percent > threshold_percent;

    let severity = if !is_regression {
        RegressionSeverity::None
    } else if change_percent < threshold_percent * 2.0 {
        RegressionSeverity::Minor
    } else if change_percent < threshold_percent * 5.0 {
        RegressionSeverity::Major
    } else {
        RegressionSeverity::Critical
    };

    RegressionResult {
        benchmark_id: current.id.clone(),
        current_mean_ns: current.mean_ns,
        baseline_mean_ns: baseline.mean_ns,
        change_percent,
        is_regression,
        threshold_percent,
        severity,
    }
}

/// Get current git commit hash
fn get_git_commit() -> Option<String> {
    std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
}

/// Get current git branch
fn get_git_branch() -> Option<String> {
    std::process::Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regression_detection() {
        let current = BenchmarkResult {
            id: "test".to_string(),
            category: BenchmarkCategory::ConstraintSolver,
            name: "test".to_string(),
            timestamp: chrono::Utc::now(),
            git_commit: None,
            git_branch: None,
            mean_ns: 110.0,
            median_ns: 105.0,
            std_dev_ns: 5.0,
            min_ns: 95.0,
            max_ns: 125.0,
            sample_size: 100,
            throughput: None,
            metadata: HashMap::new(),
        };

        let baseline = BenchmarkResult {
            id: "test".to_string(),
            category: BenchmarkCategory::ConstraintSolver,
            name: "test".to_string(),
            timestamp: chrono::Utc::now(),
            git_commit: None,
            git_branch: None,
            mean_ns: 100.0,
            median_ns: 98.0,
            std_dev_ns: 4.0,
            min_ns: 90.0,
            max_ns: 115.0,
            sample_size: 100,
            throughput: None,
            metadata: HashMap::new(),
        };

        let result = detect_regression(&current, &baseline, 10.0);
        assert!(result.is_regression);
        assert_eq!(result.change_percent, 10.0);
        assert_eq!(result.severity, RegressionSeverity::Minor);

        let result2 = detect_regression(&current, &baseline, 5.0);
        assert!(result2.is_regression);
        assert_eq!(result2.severity, RegressionSeverity::Major);
    }
}