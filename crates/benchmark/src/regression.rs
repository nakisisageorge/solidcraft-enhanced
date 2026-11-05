//! Regression detection and historical tracking

use crate::{BenchmarkResult, RegressionResult, RegressionSeverity, BenchmarkError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Historical benchmark storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkHistory {
    pub results: Vec<BenchmarkResult>,
    pub baselines: HashMap<String, BenchmarkResult>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl Default for BenchmarkHistory {
    fn default() -> Self {
        Self {
            results: Vec::new(),
            baselines: HashMap::new(),
            last_updated: chrono::Utc::now(),
        }
    }
}

impl BenchmarkHistory {
    /// Load history from file
    pub fn load(path: &PathBuf) -> Result<Self, BenchmarkError> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(path)
            .map_err(|e| BenchmarkError::Storage(e.to_string()))?;

        serde_json::from_str(&content)
            .map_err(|e| BenchmarkError::Storage(e.to_string()))
    }

    /// Save history to file
    pub fn save(&self, path: &PathBuf) -> Result<(), BenchmarkError> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| BenchmarkError::Storage(e.to_string()))?;

        std::fs::write(path, content)
            .map_err(|e| BenchmarkError::Storage(e.to_string()))
    }

    /// Add a new result and update baseline if needed
    pub fn add_result(&mut self, result: BenchmarkResult) {
        self.results.push(result.clone());
        self.last_updated = chrono::Utc::now();

        // Update baseline if this is the first result for this benchmark
        // or if it's significantly better than the current baseline
        let key = format!("{:?}_{}", result.category, result.name);
        let should_update = match self.baselines.get(&key) {
            None => true,
            Some(baseline) => result.mean_ns < baseline.mean_ns * 0.95, // 5% improvement
        };

        if should_update {
            self.baselines.insert(key, result);
        }
    }

    /// Get baseline for a benchmark
    pub fn get_baseline(&self, category: crate::BenchmarkCategory, name: &str) -> Option<&BenchmarkResult> {
        let key = format!("{:?}_{}", category, name);
        self.baselines.get(&key)
    }

    /// Check for regressions against baselines
    pub fn check_regressions(&self, threshold_percent: f64) -> Vec<RegressionResult> {
        let mut regressions = Vec::new();

        for result in &self.results {
            if let Some(baseline) = self.get_baseline(result.category, &result.name) {
                let regression = crate::detect_regression(result, baseline, threshold_percent);
                if regression.is_regression {
                    regressions.push(regression);
                }
            }
        }

        regressions
    }

    /// Get summary statistics
    pub fn summary(&self) -> BenchmarkSummary {
        let total_benchmarks = self.results.len();
        let unique_benchmarks = self.baselines.len();
        let regressions = self.check_regressions(10.0).len();

        BenchmarkSummary {
            total_benchmarks,
            unique_benchmarks,
            regressions_detected: regressions,
            last_updated: self.last_updated,
        }
    }
}

/// Benchmark summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSummary {
    pub total_benchmarks: usize,
    pub unique_benchmarks: usize,
    pub regressions_detected: usize,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// Regression report for CI/CD
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionReport {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub git_commit: Option<String>,
    pub git_branch: Option<String>,
    pub regressions: Vec<RegressionResult>,
    pub summary: BenchmarkSummary,
    pub passed: bool,
}

impl RegressionReport {
    /// Generate report from history
    pub fn generate(history: &BenchmarkHistory, threshold_percent: f64) -> Self {
        let regressions = history.check_regressions(threshold_percent);
        let summary = history.summary();
        let passed = regressions.is_empty();

        Self {
            timestamp: chrono::Utc::now(),
            git_commit: get_git_commit(),
            git_branch: get_git_branch(),
            regressions,
            summary,
            passed,
        }
    }

    /// Save report to file
    pub fn save(&self, path: &PathBuf) -> Result<(), BenchmarkError> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| BenchmarkError::Storage(e.to_string()))?;

        std::fs::write(path, content)
            .map_err(|e| BenchmarkError::Storage(e.to_string()))
    }

    /// Print human-readable report
    pub fn print(&self) {
        println!("=== Benchmark Regression Report ===");
        println!("Timestamp: {}", self.timestamp.format("%Y-%m-%d %H:%M:%S UTC"));
        if let Some(commit) = &self.git_commit {
            println!("Commit: {}", &commit[..8.min(commit.len())]);
        }
        if let Some(branch) = &self.git_branch {
            println!("Branch: {}", branch);
        }
        println!("Passed: {}", if self.passed { "YES" } else { "NO" });
        println!();

        if self.regressions.is_empty() {
            println!("No regressions detected. ✓");
        } else {
            println!("REGRESSIONS DETECTED ({}):", self.regressions.len());
            for reg in &self.regressions {
                let severity_marker = match reg.severity {
                    RegressionSeverity::Critical => "🔴 CRITICAL",
                    RegressionSeverity::Major => "🟠 MAJOR",
                    RegressionSeverity::Minor => "🟡 MINOR",
                    RegressionSeverity::None => "🟢 NONE",
                };
                println!(
                    "  {} {}: {:.1}% slower (threshold: {:.1}%)",
                    severity_marker, reg.benchmark_id, reg.change_percent, reg.threshold_percent
                );
            }
        }
        println!();
        println!("Summary:");
        println!("  Total benchmarks: {}", self.summary.total_benchmarks);
        println!("  Unique benchmarks: {}", self.summary.unique_benchmarks);
        println!("  Regressions: {}", self.summary.regressions_detected);
    }
}

fn get_git_commit() -> Option<String> {
    std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
}

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
    fn test_history_load_save() {
        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join("test_benchmark_history.json");

        let mut history = BenchmarkHistory::default();
        let result = BenchmarkResult {
            id: "test".to_string(),
            category: crate::BenchmarkCategory::ConstraintSolver,
            name: "test_bench".to_string(),
            timestamp: chrono::Utc::now(),
            git_commit: None,
            git_branch: None,
            mean_ns: 100.0,
            median_ns: 98.0,
            std_dev_ns: 5.0,
            min_ns: 90.0,
            max_ns: 110.0,
            sample_size: 100,
            throughput: None,
            metadata: HashMap::new(),
        };

        history.add_result(result.clone());
        history.save(&path).unwrap();

        let loaded = BenchmarkHistory::load(&path).unwrap();
        assert_eq!(loaded.results.len(), 1);
        assert_eq!(loaded.baselines.len(), 1);

        std::fs::remove_file(path).ok();
    }

    #[test]
    fn test_regression_report() {
        let mut history = BenchmarkHistory::default();
        let baseline = BenchmarkResult {
            id: "baseline".to_string(),
            category: crate::BenchmarkCategory::ConstraintSolver,
            name: "test".to_string(),
            timestamp: chrono::Utc::now(),
            git_commit: None,
            git_branch: None,
            mean_ns: 100.0,
            median_ns: 98.0,
            std_dev_ns: 5.0,
            min_ns: 90.0,
            max_ns: 110.0,
            sample_size: 100,
            throughput: None,
            metadata: HashMap::new(),
        };
        history.baselines.insert("ConstraintSolver_test".to_string(), baseline);

        let current = BenchmarkResult {
            id: "current".to_string(),
            category: crate::BenchmarkCategory::ConstraintSolver,
            name: "test".to_string(),
            timestamp: chrono::Utc::now(),
            git_commit: None,
            git_branch: None,
            mean_ns: 120.0, // 20% slower
            median_ns: 118.0,
            std_dev_ns: 6.0,
            min_ns: 105.0,
            max_ns: 135.0,
            sample_size: 100,
            throughput: None,
            metadata: HashMap::new(),
        };
        history.results.push(current);

        let report = RegressionReport::generate(&history, 10.0);
        assert!(!report.passed);
        assert_eq!(report.regressions.len(), 1);
        assert_eq!(report.regressions[0].severity, RegressionSeverity::Minor);
    }
}