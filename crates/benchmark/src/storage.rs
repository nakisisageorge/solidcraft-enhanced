//! Storage utilities for benchmark data

use crate::{BenchmarkResult, BenchmarkHistory, BenchmarkError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Compressed benchmark storage format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedBenchmark {
    pub id: String,
    pub category: String,
    pub name: String,
    pub timestamp: i64, // Unix timestamp
    pub git_commit: Option<String>,
    pub git_branch: Option<String>,
    pub mean_ns: f64,
    pub median_ns: f64,
    pub std_dev_ns: f64,
    pub min_ns: f64,
    pub max_ns: f64,
    pub sample_size: usize,
    pub throughput: Option<f64>,
}

impl From<BenchmarkResult> for CompressedBenchmark {
    fn from(r: BenchmarkResult) -> Self {
        Self {
            id: r.id,
            category: format!("{:?}", r.category),
            name: r.name,
            timestamp: r.timestamp.timestamp(),
            git_commit: r.git_commit,
            git_branch: r.git_branch,
            mean_ns: r.mean_ns,
            median_ns: r.median_ns,
            std_dev_ns: r.std_dev_ns,
            min_ns: r.min_ns,
            max_ns: r.max_ns,
            sample_size: r.sample_size,
            throughput: r.throughput,
        }
    }
}

impl From<CompressedBenchmark> for BenchmarkResult {
    fn from(c: CompressedBenchmark) -> Self {
        Self {
            id: c.id,
            category: match c.category.as_str() {
                "ConstraintSolver" => crate::BenchmarkCategory::ConstraintSolver,
                "BooleanOperations" => crate::BenchmarkCategory::BooleanOperations,
                "ImportExport" => crate::BenchmarkCategory::ImportExport,
                "MeshGeneration" => crate::BenchmarkCategory::MeshGeneration,
                "ParametricRegeneration" => crate::BenchmarkCategory::ParametricRegeneration,
                "WasmBuild" => crate::BenchmarkCategory::WasmBuild,
                "WasmRuntime" => crate::BenchmarkCategory::WasmRuntime,
                _ => crate::BenchmarkCategory::ConstraintSolver,
            },
            name: c.name,
            timestamp: chrono::DateTime::from_timestamp(c.timestamp, 0).unwrap_or_default(),
            git_commit: c.git_commit,
            git_branch: c.git_branch,
            mean_ns: c.mean_ns,
            median_ns: c.median_ns,
            std_dev_ns: c.std_dev_ns,
            min_ns: c.min_ns,
            max_ns: c.max_ns,
            sample_size: c.sample_size,
            throughput: c.throughput,
            metadata: std::collections::HashMap::new(),
        }
    }
}

/// Storage backend trait
pub trait BenchmarkStorage {
    fn save_history(&self, history: &BenchmarkHistory) -> Result<(), BenchmarkError>;
    fn load_history(&self) -> Result<BenchmarkHistory, BenchmarkError>;
    fn save_results(&self, results: &[BenchmarkResult]) -> Result<(), BenchmarkError>;
    fn load_results(&self) -> Result<Vec<BenchmarkResult>, BenchmarkError>;
}

/// File-based storage
pub struct FileStorage {
    history_path: PathBuf,
    results_path: PathBuf,
}

impl FileStorage {
    pub fn new(history_path: PathBuf, results_path: PathBuf) -> Self {
        Self { history_path, results_path }
    }
}

impl BenchmarkStorage for FileStorage {
    fn save_history(&self, history: &BenchmarkHistory) -> Result<(), BenchmarkError> {
        history.save(&self.history_path)
    }

    fn load_history(&self) -> Result<BenchmarkHistory, BenchmarkError> {
        BenchmarkHistory::load(&self.history_path)
    }

    fn save_results(&self, results: &[BenchmarkResult]) -> Result<(), BenchmarkError> {
        let compressed: Vec<CompressedBenchmark> = results.iter().cloned().map(CompressedBenchmark::from).collect();
        let content = serde_json::to_string_pretty(&compressed)
            .map_err(|e| BenchmarkError::Storage(e.to_string()))?;
        std::fs::write(&self.results_path, content)
            .map_err(|e| BenchmarkError::Storage(e.to_string()))
    }

    fn load_results(&self) -> Result<Vec<BenchmarkResult>, BenchmarkError> {
        if !self.results_path.exists() {
            return Ok(Vec::new());
        }

        let content = std::fs::read_to_string(&self.results_path)
            .map_err(|e| BenchmarkError::Storage(e.to_string()))?;

        let compressed: Vec<CompressedBenchmark> = serde_json::from_str(&content)
            .map_err(|e| BenchmarkError::Storage(e.to_string()))?;

        Ok(compressed.into_iter().map(BenchmarkResult::from).collect())
    }
}

/// SQLite storage for larger datasets
#[cfg(feature = "sqlite")]
pub struct SqliteStorage {
    pool: sqlx::SqlitePool,
}

#[cfg(feature = "sqlite")]
impl SqliteStorage {
    pub async fn new(database_url: &str) -> Result<Self, BenchmarkError> {
        let pool = sqlx::SqlitePool::connect(database_url).await
            .map_err(|e| BenchmarkError::Storage(e.to_string()))?;

        // Create tables
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS benchmark_results (
                id TEXT PRIMARY KEY,
                category TEXT NOT NULL,
                name TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                git_commit TEXT,
                git_branch TEXT,
                mean_ns REAL NOT NULL,
                median_ns REAL NOT NULL,
                std_dev_ns REAL NOT NULL,
                min_ns REAL NOT NULL,
                max_ns REAL NOT NULL,
                sample_size INTEGER NOT NULL,
                throughput REAL
            )
            "#,
        )
        .execute(&pool)
        .await
        .map_err(|e| BenchmarkError::Storage(e.to_string()))?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS baselines (
                key TEXT PRIMARY KEY,
                result_id TEXT NOT NULL,
                FOREIGN KEY (result_id) REFERENCES benchmark_results(id)
            )
            "#,
        )
        .execute(&pool)
        .await
        .map_err(|e| BenchmarkError::Storage(e.to_string()))?;

        Ok(Self { pool })
    }
}

#[cfg(feature = "sqlite")]
impl BenchmarkStorage for SqliteStorage {
    fn save_history(&self, history: &BenchmarkHistory) -> Result<(), BenchmarkError> {
        // Save to file as fallback
        FileStorage::new("benchmark_history.json".into(), "benchmark_results.json".into())
            .save_history(history)
    }

    fn load_history(&self) -> Result<BenchmarkHistory, BenchmarkError> {
        FileStorage::new("benchmark_history.json".into(), "benchmark_results.json".into())
            .load_history()
    }

    fn save_results(&self, results: &[BenchmarkResult]) -> Result<(), BenchmarkError> {
        for result in results {
            let compressed = CompressedBenchmark::from(result.clone());
            sqlx::query(
                r#"
                INSERT OR REPLACE INTO benchmark_results
                (id, category, name, timestamp, git_commit, git_branch, mean_ns, median_ns, std_dev_ns, min_ns, max_ns, sample_size, throughput)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(&compressed.id)
            .bind(&compressed.category)
            .bind(&compressed.name)
            .bind(compressed.timestamp)
            .bind(&compressed.git_commit)
            .bind(&compressed.git_branch)
            .bind(compressed.mean_ns)
            .bind(compressed.median_ns)
            .bind(compressed.std_dev_ns)
            .bind(compressed.min_ns)
            .bind(compressed.max_ns)
            .bind(compressed.sample_size as i64)
            .bind(compressed.throughput)
            .execute(&self.pool)
            .await
            .map_err(|e| BenchmarkError::Storage(e.to_string()))?;
        }
        Ok(())
    }

    fn load_results(&self) -> Result<Vec<BenchmarkResult>, BenchmarkError> {
        let rows = sqlx::query(
            "SELECT * FROM benchmark_results ORDER BY timestamp DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BenchmarkError::Storage(e.to_string()))?;

        let mut results = Vec::new();
        for row in rows {
            let compressed = CompressedBenchmark {
                id: row.get("id"),
                category: row.get("category"),
                name: row.get("name"),
                timestamp: row.get("timestamp"),
                git_commit: row.get("git_commit"),
                git_branch: row.get("git_branch"),
                mean_ns: row.get("mean_ns"),
                median_ns: row.get("median_ns"),
                std_dev_ns: row.get("std_dev_ns"),
                min_ns: row.get("min_ns"),
                max_ns: row.get("max_ns"),
                sample_size: row.get("sample_size"),
                throughput: row.get("throughput"),
            };
            results.push(BenchmarkResult::from(compressed));
        }
        Ok(results)
    }
}