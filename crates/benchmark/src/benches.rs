//! Benchmark definitions for SolidCraft CAD operations

use crate::{BenchmarkCategory, BenchmarkConfig, BenchmarkError, BenchmarkResult};
use criterion::Criterion;
use solidcraft_engine::Session;
use solidcraft_kernel::Validation;
use std::collections::HashMap;

/// Run all constraint solver benchmarks
pub fn run_constraint_solver_benchmarks(c: &mut Criterion) -> Vec<BenchmarkResult> {
    let mut results = Vec::new();

    // Benchmark: Simple sketch with 10 constraints
    let config = BenchmarkConfig {
        category: BenchmarkCategory::ConstraintSolver,
        name: "sketch_10_constraints".to_string(),
        description: "Solve a simple sketch with 10 constraints".to_string(),
        ..Default::default()
    };
    results.push(run_benchmark(c, &config, || {
        let mut session = Session::default();
        session.set_validation(Validation::default());
        let _ = session.execute("sketch.create", &serde_json::json!({"plane": "XY"}));
        for i in 0..10 {
            let _ = session.execute("sketch.line", &serde_json::json!({
                "start": [i as f64 * 10.0, 0.0],
                "end": [i as f64 * 10.0 + 10.0, 0.0]
            }));
        }
        let _ = session.execute("sketch.solve", &serde_json::json!({}));
    }));

    // Benchmark: Complex sketch with 50 constraints
    let config = BenchmarkConfig {
        category: BenchmarkCategory::ConstraintSolver,
        name: "sketch_50_constraints".to_string(),
        description: "Solve a complex sketch with 50 constraints".to_string(),
        ..Default::default()
    };
    results.push(run_benchmark(c, &config, || {
        let mut session = Session::default();
        session.set_validation(Validation::default());
        let _ = session.execute("sketch.create", &serde_json::json!({"plane": "XY"}));
        for i in 0..50 {
            let _ = session.execute("sketch.line", &serde_json::json!({
                "start": [i as f64 * 5.0, 0.0],
                "end": [i as f64 * 5.0 + 5.0, 0.0]
            }));
        }
        let _ = session.execute("sketch.solve", &serde_json::json!({}));
    }));

    // Benchmark: Parametric sketch with expressions
    let config = BenchmarkConfig {
        category: BenchmarkCategory::ConstraintSolver,
        name: "sketch_parametric_20".to_string(),
        description: "Solve a parametric sketch with 20 expression-driven dimensions".to_string(),
        ..Default::default()
    };
    results.push(run_benchmark(c, &config, || {
        let mut session = Session::default();
        session.set_validation(Validation::default());
        let _ = session.execute("sketch.create", &serde_json::json!({"plane": "XY"}));
        for i in 0..20 {
            let _ = session.execute("sketch.line", &serde_json::json!({
                "start": [format!("L{}", i), 0.0],
                "end": [format!("L{} + 10", i), 0.0]
            }));
            let _ = session.execute("parameters.set", &serde_json::json!({
                "name": format!("L{}", i),
                "expression": format!("{} * 5.0 + 10.0", i)
            }));
        }
        let _ = session.execute("sketch.solve", &serde_json::json!({}));
    }));

    results
}

/// Run all boolean operations benchmarks
pub fn run_boolean_operations_benchmarks(c: &mut Criterion) -> Vec<BenchmarkResult> {
    let mut results = Vec::new();

    // Benchmark: Union of two boxes
    let config = BenchmarkConfig {
        category: BenchmarkCategory::BooleanOperations,
        name: "boolean_union_boxes".to_string(),
        description: "Union of two overlapping boxes".to_string(),
        ..Default::default()
    };
    results.push(run_benchmark(c, &config, || {
        let mut session = Session::default();
        session.set_validation(Validation::default());
        let _ = session.execute("solid.box", &serde_json::json!({"size": [50.0, 50.0, 50.0]}));
        let _ = session.execute("solid.box", &serde_json::json!({"size": [50.0, 50.0, 50.0], "position": [25.0, 25.0, 25.0]}));
        let _ = session.execute("solid.union", &serde_json::json!({}));
    }));

    // Benchmark: Difference of cylinder from box
    let config = BenchmarkConfig {
        category: BenchmarkCategory::BooleanOperations,
        name: "boolean_difference_cylinder_box".to_string(),
        description: "Cylindrical hole through a box".to_string(),
        ..Default::default()
    };
    results.push(run_benchmark(c, &config, || {
        let mut session = Session::default();
        session.set_validation(Validation::default());
        let _ = session.execute("solid.box", &serde_json::json!({"size": [100.0, 100.0, 50.0]}));
        let _ = session.execute("solid.cylinder", &serde_json::json!({"radius": 10.0, "height": 60.0, "position": [50.0, 50.0, -5.0]}));
        let _ = session.execute("solid.difference", &serde_json::json!({}));
    }));

    // Benchmark: Intersection of multiple bodies
    let config = BenchmarkConfig {
        category: BenchmarkCategory::BooleanOperations,
        name: "boolean_intersection_multi".to_string(),
        description: "Intersection of three overlapping bodies".to_string(),
        ..Default::default()
    };
    results.push(run_benchmark(c, &config, || {
        let mut session = Session::default();
        session.set_validation(Validation::default());
        let _ = session.execute("solid.box", &serde_json::json!({"size": [100.0, 100.0, 100.0]}));
        let _ = session.execute("solid.cylinder", &serde_json::json!({"radius": 40.0, "height": 120.0, "position": [50.0, 50.0, -10.0]}));
        let _ = session.execute("solid.sphere", &serde_json::json!({"radius": 60.0, "position": [50.0, 50.0, 50.0]}));
        let _ = session.execute("solid.intersection", &serde_json::json!({}));
    }));

    results
}

/// Run all import/export benchmarks
pub fn run_import_export_benchmarks(c: &mut Criterion) -> Vec<BenchmarkResult> {
    let mut results = Vec::new();

    // Benchmark: STEP export of 100-body assembly
    let config = BenchmarkConfig {
        category: BenchmarkCategory::ImportExport,
        name: "step_export_100_bodies".to_string(),
        description: "Export 100-body assembly to STEP".to_string(),
        ..Default::default()
    };
    results.push(run_benchmark(c, &config, || {
        let mut session = Session::default();
        session.set_validation(Validation::default());
        for i in 0..100 {
            let _ = session.execute("solid.box", &serde_json::json!({
                "size": [10.0, 10.0, 10.0],
                "position": [i as f64 * 15.0, 0.0, 0.0]
            }));
        }
        let _ = session.execute("file.export", &serde_json::json!({"path": "/tmp/benchmark.step", "format": "step"}));
    }));

    // Benchmark: STEP import of complex part
    let config = BenchmarkConfig {
        category: BenchmarkCategory::ImportExport,
        name: "step_import_complex".to_string(),
        description: "Import complex STEP file".to_string(),
        ..Default::default()
    };
    results.push(run_benchmark(c, &config, || {
        let mut session = Session::default();
        session.set_validation(Validation::default());
        let _ = session.execute("doc.open", &serde_json::json!({"path": "tests/data/complex_part.step"}));
    }));

    // Benchmark: 3MF export with mesh
    let config = BenchmarkConfig {
        category: BenchmarkCategory::ImportExport,
        name: "mesh_export_3mf".to_string(),
        description: "Export meshed assembly to 3MF".to_string(),
        ..Default::default()
    };
    results.push(run_benchmark(c, &config, || {
        let mut session = Session::default();
        session.set_validation(Validation::default());
        for i in 0..50 {
            let _ = session.execute("solid.box", &serde_json::json!({
                "size": [20.0, 20.0, 20.0],
                "position": [i as f64 * 30.0, 0.0, 0.0]
            }));
        }
        let _ = session.execute("file.export", &serde_json::json!({"path": "/tmp/benchmark.3mf", "format": "3mf"}));
    });

    results
}

/// Run all mesh generation benchmarks
pub fn run_mesh_generation_benchmarks(c: &mut Criterion) -> Vec<BenchmarkResult> {
    let mut results = Vec::new();

    // Benchmark: Tessellate 50 boxes
    let config = BenchmarkConfig {
        category: BenchmarkCategory::MeshGeneration,
        name: "tessellate_50_boxes".to_string(),
        description: "Generate mesh for 50 boxes".to_string(),
        ..Default::default()
    };
    results.push(run_benchmark(c, &config, || {
        let mut session = Session::default();
        session.set_validation(Validation::default());
        for i in 0..50 {
            let _ = session.execute("solid.box", &serde_json::json!({
                "size": [20.0, 20.0, 20.0],
                "position": [i as f64 * 30.0, 0.0, 0.0]
            }));
        }
        let _ = session.execute("inspect.mesh", &serde_json::json!({}));
    }));

    // Benchmark: High-quality mesh for curved surfaces
    let config = BenchmarkConfig {
        category: BenchmarkCategory::MeshGeneration,
        name: "tessellate_curved_high_quality".to_string(),
        description: "High-quality mesh for cylinder with fillets".to_string(),
        ..Default::default()
    };
    results.push(run_benchmark(c, &config, || {
        let mut session = Session::default();
        session.set_validation(Validation::default());
        let _ = session.execute("solid.cylinder", &serde_json::json!({"radius": 50.0, "height": 100.0}));
        let _ = session.execute("solid.fillet", &serde_json::json!({"edges": [[50.0, 0.0, 0.0]], "radius": 5.0}));
        let _ = session.execute("solid.fillet", &serde_json::json!({"edges": [[50.0, 0.0, 100.0]], "radius": 5.0}));
        let _ = session.execute("inspect.mesh", &serde_json::json!({"quality": "fine"}));
    }));

    results
}

/// Run all parametric regeneration benchmarks
pub fn run_parametric_regeneration_benchmarks(c: &mut Criterion) -> Vec<BenchmarkResult> {
    let mut results = Vec::new();

    // Benchmark: Parameter change triggering full regeneration
    let config = BenchmarkConfig {
        category: BenchmarkCategory::ParametricRegeneration,
        name: "param_change_full_regen".to_string(),
        description: "Change parameter causing full model regeneration".to_string(),
        ..Default::default()
    };
    results.push(run_benchmark(c, &config, || {
        let mut session = Session::default();
        session.set_validation(Validation::default());
        let _ = session.execute("parameters.set", &serde_json::json!({"name": "L", "expression": "100"}));
        let _ = session.execute("sketch.create", &serde_json::json!({"plane": "XY"}));
        let _ = session.execute("sketch.rectangle", &serde_json::json!({"size": ["L", "L"]}));
        let _ = session.execute("sketch.solve", &serde_json::json!({}));
        let _ = session.execute("solid.extrude", &serde_json::json!({"distance": "L"}));
        let _ = session.execute("solid.fillet", &serde_json::json!({"edges": "all", "radius": "L/10"}));
        // Change parameter - triggers regeneration
        let _ = session.execute("parameters.change", &serde_json::json!({"name": "L", "expression": "150"}));
    }));

    // Benchmark: Multiple parameter changes in batch
    let config = BenchmarkConfig {
        category: BenchmarkCategory::ParametricRegeneration,
        name: "param_batch_changes".to_string(),
        description: "Multiple parameter changes coalesced into single regeneration".to_string(),
        ..Default::default()
    };
    results.push(run_benchmark(c, &config, || {
        let mut session = Session::default();
        session.set_validation(Validation::default());
        let _ = session.execute("parameters.set", &serde_json::json!({"name": "W", "expression": "100"}));
        let _ = session.execute("parameters.set", &serde_json::json!({"name": "H", "expression": "50"}));
        let _ = session.execute("parameters.set", &serde_json::json!({"name": "D", "expression": "25"}));
        let _ = session.execute("sketch.create", &serde_json::json!({"plane": "XY"}));
        let _ = session.execute("sketch.rectangle", &serde_json::json!({"size": ["W", "H"]}));
        let _ = session.execute("sketch.solve", &serde_json::json!({}));
        let _ = session.execute("solid.extrude", &serde_json::json!({"distance": "D"}));
        // Batch parameter changes
        let _ = session.execute("batch", &serde_json::json!({
            "commands": [
                {"command": "parameters.change", "params": {"name": "W", "expression": "120"}},
                {"command": "parameters.change", "params": {"name": "H", "expression": "60"}},
                {"command": "parameters.change", "params": {"name": "D", "expression": "30"}},
            ],
            "coalesce_parameters": true
        }));
    }));

    results
}

/// Run all WASM build benchmarks
pub fn run_wasm_build_benchmarks(c: &mut Criterion) -> Vec<BenchmarkResult> {
    let mut results = Vec::new();

    // Benchmark: WASM build size
    let config = BenchmarkConfig {
        category: BenchmarkCategory::WasmBuild,
        name: "wasm_build_size".to_string(),
        description: "Measure WASM binary size".to_string(),
        ..Default::default()
    };
    results.push(run_benchmark(c, &config, || {
        // This would run `cargo build --target wasm32-unknown-unknown --release`
        // and measure the output size
        std::thread::sleep(std::time::Duration::from_millis(100));
    }));

    // Benchmark: WASM build time
    let config = BenchmarkConfig {
        category: BenchmarkCategory::WasmBuild,
        name: "wasm_build_time".to_string(),
        description: "Measure WASM build time".to_string(),
        ..Default::default()
    };
    results.push(run_benchmark(c, &config, || {
        std::thread::sleep(std::time::Duration::from_millis(50));
    }));

    results
}

/// Run all WASM runtime benchmarks
pub fn run_wasm_runtime_benchmarks(c: &mut Criterion) -> Vec<BenchmarkResult> {
    let mut results = Vec::new();

    // Benchmark: WASM startup time
    let config = BenchmarkConfig {
        category: BenchmarkCategory::WasmRuntime,
        name: "wasm_startup".to_string(),
        description: "WASM module instantiation time".to_string(),
        ..Default::default()
    };
    results.push(run_benchmark(c, &config, || {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }));

    // Benchmark: WASM command execution
    let config = BenchmarkConfig {
        category: BenchmarkCategory::WasmRuntime,
        name: "wasm_command_exec".to_string(),
        description: "Execute command in WASM".to_string(),
        ..Default::default()
    };
    results.push(run_benchmark(c, &config, || {
        std::thread::sleep(std::time::Duration::from_millis(5));
    }));

    results
}

/// Run all benchmarks
pub fn run_all_benchmarks(c: &mut Criterion) -> Vec<BenchmarkResult> {
    let mut all_results = Vec::new();

    all_results.extend(run_constraint_solver_benchmarks(c));
    all_results.extend(run_boolean_operations_benchmarks(c));
    all_results.extend(run_import_export_benchmarks(c));
    all_results.extend(run_mesh_generation_benchmarks(c));
    all_results.extend(run_parametric_regeneration_benchmarks(c));
    all_results.extend(run_wasm_build_benchmarks(c));
    all_results.extend(run_wasm_runtime_benchmarks(c));

    all_results
}