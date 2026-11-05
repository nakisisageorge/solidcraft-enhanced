//! Benchmark runner CLI

use crate::{BenchmarkConfig, BenchmarkCategory, BenchmarkResult, BenchmarkHistory, BenchmarkError, RegressionReport};
use clap::{Parser, Subcommand};
use criterion::Criterion;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "solidcraft-bench", version, about = "SolidCraft benchmark runner with regression detection")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run all benchmarks
    Run {
        /// Output file for results (JSON)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// History file for regression tracking
        #[arg(short, long, default_value = "benchmark_history.json")]
        history: PathBuf,

        /// Regression threshold percentage
        #[arg(long, default_value = "10.0")]
        threshold: f64,

        /// Only run specific category
        #[arg(long)]
        category: Option<BenchmarkCategory>,

        /// Only run specific benchmark by name
        #[arg(long)]
        benchmark: Option<String>,

        /// Generate HTML report
        #[arg(long)]
        html: bool,
    },
    /// Check for regressions against history
    Check {
        /// History file
        #[arg(short, long, default_value = "benchmark_history.json")]
        history: PathBuf,

        /// Regression threshold percentage
        #[arg(long, default_value = "10.0")]
        threshold: f64,

        /// Output report file
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Fail on regression (exit code 1)
        #[arg(long)]
        fail_on_regression: bool,
    },
    /// Update baselines with current results
    UpdateBaseline {
        /// History file
        #[arg(short, long, default_value = "benchmark_history.json")]
        history: PathBuf,

        /// Results file to use for baselines
        #[arg(short, long)]
        results: PathBuf,
    },
    /// Show benchmark history summary
    Summary {
        /// History file
        #[arg(short, long, default_value = "benchmark_history.json")]
        history: PathBuf,
    },
    /// Compare two result files
    Compare {
        /// Baseline results file
        #[arg(long)]
        baseline: PathBuf,

        /// Current results file
        #[arg(long)]
        current: PathBuf,

        /// Regression threshold percentage
        #[arg(long, default_value = "10.0")]
        threshold: f64,

        /// Output report file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

/// Run the benchmark CLI
pub fn run_cli() -> Result<(), BenchmarkError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { output, history, threshold, category, benchmark, html } => {
            run_benchmarks(output, history, threshold, category, benchmark, html)
        }
        Commands::Check { history, threshold, output, fail_on_regression } => {
            check_regressions(history, threshold, output, fail_on_regression)
        }
        Commands::UpdateBaseline { history, results } => {
            update_baselines(history, results)
        }
        Commands::Summary { history } => {
            show_summary(history)
        }
        Commands::Compare { baseline, current, threshold, output } => {
            compare_results(baseline, current, threshold, output)
        }
    }
}

fn run_benchmarks(
    output: Option<PathBuf>,
    history_path: PathBuf,
    threshold: f64,
    category: Option<BenchmarkCategory>,
    benchmark_name: Option<String>,
    html: bool,
) -> Result<(), BenchmarkError> {
    let mut criterion = Criterion::default()
        .configure_from_args()
        .with_plots(html);

    if html {
        criterion = criterion.with_plots(true);
    }

    // Load existing history
    let mut history = BenchmarkHistory::load(&history_path)?;

    // Run benchmarks
    let results = crate::benches::run_all_benchmarks(&mut criterion);

    // Filter by category if specified
    let filtered_results: Vec<BenchmarkResult> = if let Some(cat) = category {
        results.into_iter().filter(|r| r.category == cat).collect()
    } else {
        results
    };

    // Filter by benchmark name if specified
    let filtered_results: Vec<BenchmarkResult> = if let Some(name) = benchmark_name {
        filtered_results.into_iter().filter(|r| r.name == name).collect()
    } else {
        filtered_results
    };

    // Add results to history
    for result in &filtered_results {
        history.add_result(result.clone());
    }

    // Save updated history
    history.save(&history_path)?;

    // Save results if output specified
    if let Some(out_path) = output {
        let content = serde_json::to_string_pretty(&filtered_results)
            .map_err(|e| BenchmarkError::Storage(e.to_string()))?;
        std::fs::write(out_path, content)
            .map_err(|e| BenchmarkError::Storage(e.to_string()))?;
    }

    // Check for regressions
    let regressions = history.check_regressions(threshold);
    if !regressions.is_empty() {
        eprintln!("⚠️  {} regression(s) detected!", regressions.len());
        for reg in &regressions {
            eprintln!("  {}: {:.1}% slower (threshold: {:.1}%)",
                reg.benchmark_id, reg.change_percent, reg.threshold_percent);
        }
    } else {
        println!("✓ No regressions detected");
    }

    // Print summary
    let summary = history.summary();
    println!("Benchmarks run: {}", filtered_results.len());
    println!("Total in history: {}", summary.total_benchmarks);
    println!("Unique benchmarks: {}", summary.unique_benchmarks);
    println!("Regressions: {}", summary.regressions_detected);

    Ok(())
}

fn check_regressions(
    history_path: PathBuf,
    threshold: f64,
    output: Option<PathBuf>,
    fail_on_regression: bool,
) -> Result<(), BenchmarkError> {
    let history = BenchmarkHistory::load(&history_path)?;
    let report = RegressionReport::generate(&history, threshold);

    report.print();

    if let Some(out_path) = output {
        report.save(&out_path)?;
    }

    if fail_on_regression && !report.passed {
        return Err(BenchmarkError::Regression(
            format!("{} regression(s) detected", report.regressions.len())
        ));
    }

    Ok(())
}

fn update_baselines(history_path: PathBuf, results_path: PathBuf) -> Result<(), BenchmarkError> {
    let mut history = BenchmarkHistory::load(&history_path)?;

    let content = std::fs::read_to_string(&results_path)
        .map_err(|e| BenchmarkError::Storage(e.to_string()))?;

    let results: Vec<BenchmarkResult> = serde_json::from_str(&content)
        .map_err(|e| BenchmarkError::Storage(e.to_string()))?;

    for result in results {
        let key = format!("{:?}_{}", result.category, result.name);
        history.baselines.insert(key, result);
    }

    history.last_updated = chrono::Utc::now();
    history.save(&history_path)?;

    println!("Baselines updated from {}", results_path.display());
    Ok(())
}

fn show_summary(history_path: PathBuf) -> Result<(), BenchmarkError> {
    let history = BenchmarkHistory::load(&history_path)?;
    let summary = history.summary();

    println!("=== Benchmark History Summary ===");
    println!("Total benchmarks run: {}", summary.total_benchmarks);
    println!("Unique benchmarks: {}", summary.unique_benchmarks);
    println!("Regressions detected: {}", summary.regressions_detected);
    println!("Last updated: {}", summary.last_updated.format("%Y-%m-%d %H:%M:%S UTC"));

    println!("\nBaselines:");
    for (key, baseline) in &history.baselines {
        println!("  {}: {:.0} ns (commit: {})",
            key,
            baseline.mean_ns,
            baseline.git_commit.as_deref().unwrap_or("unknown").chars().take(8).collect::<String>()
        );
    }

    Ok(())
}

fn compare_results(
    baseline_path: PathBuf,
    current_path: PathBuf,
    threshold: f64,
    output: Option<PathBuf>,
) -> Result<(), BenchmarkError> {
    let baseline_content = std::fs::read_to_string(&baseline_path)
        .map_err(|e| BenchmarkError::Storage(e.to_string()))?;
    let baseline_results: Vec<BenchmarkResult> = serde_json::from_str(&baseline_content)
        .map_err(|e| BenchmarkError::Storage(e.to_string()))?;

    let current_content = std::fs::read_to_string(&current_path)
        .map_err(|e| BenchmarkError::Storage(e.to_string()))?;
    let current_results: Vec<BenchmarkResult> = serde_json::from_str(&current_content)
        .map_err(|e| BenchmarkError::Storage(e.to_string()))?;

    let mut regressions = Vec::new();

    for current in &current_results {
        for baseline in &baseline_results {
            if baseline.category == current.category && baseline.name == current.name {
                let reg = crate::detect_regression(current, baseline, threshold);
                if reg.is_regression {
                    regressions.push(reg);
                }
                break;
            }
        }
    }

    let report = RegressionReport {
        timestamp: chrono::Utc::now(),
        git_commit: get_git_commit(),
        git_branch: get_git_branch(),
        regressions,
        summary: BenchmarkHistory {
            results: current_results,
            baselines: baseline_results.into_iter().map(|r| (format!("{:?}_{}", r.category, r.name), r)).collect(),
            last_updated: chrono::Utc::now(),
        }.summary(),
        passed: regressions.is_empty(),
    };

    report.print();

    if let Some(out_path) = output {
        report.save(&out_path)?;
    }

    if !report.passed {
        return Err(BenchmarkError::Regression(
            format!("{} regression(s) detected", report.regressions.len())
        ));
    }

    Ok(())
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