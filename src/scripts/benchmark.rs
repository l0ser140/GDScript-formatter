//! This module tests the performance of the GDScript formatter. Use this to quickly test the
//! performance impact of changes to the formatter locally.
//!
//! Run cargo run --bin benchmark --release to compile and run the benchmark.
//! You can use it in a shell script to compare performance between two git revisions.
//!
//! For example, to compare between this commit and the previous one:
//!
//! ```sh
//! cargo run --bin benchmark --release > benchmark_results.txt
//! echo "On previous commit:\n" >> benchmark_results.txt
//! git checkout HEAD^
//! cargo run --bin benchmark --release >> benchmark_results.txt
//! git checkout -
//! ```
use gdscript_formatter::{formatter::format_gdscript_with_config, FormatterConfig};
use std::{fs, time::Instant};

const ITERATIONS: u16 = 100;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let short_content = fs::read_to_string("benchmarks/gdscript_files/short.gd")?;
    let long_content = fs::read_to_string("benchmarks/gdscript_files/long.gd")?;
    let config = FormatterConfig::default();

    println!("Running GDScript Formatter Benchmark...");

    println!("Running short file warmup (10 iterations)");
    for _ in 0..10 {
        let _ = format_gdscript_with_config(&short_content, &config)?;
    }

    println!("Benchmarking short file ({} iterations)", ITERATIONS);
    let mut start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = format_gdscript_with_config(&short_content, &config)?;
    }
    let duration_short_file = start.elapsed();

    // Benchmark long file
    println!("Benchmarking long file ({} iterations)...", ITERATIONS);
    start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = format_gdscript_with_config(&long_content, &config)?;
    }
    let long_time = start.elapsed();

    let average_time_short = duration_short_file.as_micros() as f64 / ITERATIONS as f64;
    let average_time_long = long_time.as_micros() as f64 / ITERATIONS as f64;

    println!("\nBenchmark Results:");
    println!("=================");
    println!(
        "Short file ({} iterations): {:?} (avg: {:.2}ms per iteration)",
        ITERATIONS,
        duration_short_file,
        average_time_short / 1000.0
    );
    println!(
        "Long file ({} iterations):   {:?} (avg: {:.2}ms per iteration)",
        ITERATIONS,
        long_time,
        average_time_long / 1000.0
    );

    Ok(())
}
