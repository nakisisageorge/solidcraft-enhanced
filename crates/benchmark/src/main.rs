//! SolidCraft Benchmark Runner Binary

use solidcraft_benchmark::run_cli;

fn main() {
    if let Err(e) = run_cli() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}