/// Log benchmark data to stdout, so it can be captured by the benchmarking framework hermit-bench.
///
/// * `benchmark_name` - The name under which the benchmark will be stored.
/// * `unit` - The unit of the benchmark value.
/// * `value` - The benchmark value.
pub fn log_benchmark_data(benchmark_name: &str, unit: &str, value: f64) {
    println!(
        "/*BENCHMARK OUTPUT*/\nname: {}\nunit: {}\nvalue: {}\n/*BENCHMARK OUTPUT END*/",
        benchmark_name, unit, value
    );
}
