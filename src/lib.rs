/// Log benchmark data to stdout, so it can be captured by the benchmarking framework hermit-bench.
///
/// * `benchmark_name` - The name under which the benchmark will be stored.
/// * `unit` - The unit of the benchmark value.
/// * `value` - The benchmark value.
pub fn log_benchmark_data(benchmark_name: &str, unit: &str, value: f64) {
    println!(
        "/*BENCHMARK OUTPUT*/\nname: {}\nunit: {}\nvalue: {}\nplot_group: \n/*BENCHMARK OUTPUT END*/",
        benchmark_name, unit, value
    );
}

/// Log benchmark data to stdout, so it can be captured by the benchmarking framework hermit-bench.
///
/// * `benchmark_name` - The name under which the benchmark will be stored.
/// * `unit` - The unit of the benchmark value.
/// * `value` - The benchmark value.
/// * `plot_group` - The plot group in which the benchmark will be plotted.
pub fn log_benchmark_data_with_group(benchmark_name: &str, unit: &str, value: f64, plot_group: &str) {
    println!(
        "/*BENCHMARK OUTPUT*/\nname: {}\nunit: {}\nvalue: {}\nplot_group: {}\n/*BENCHMARK OUTPUT END*/",
        benchmark_name, unit, value, plot_group
    );
}
