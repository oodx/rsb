// Performance Measurement (from macros/time_math.rs)

// MACRO TO MOVE FROM macros/time_math.rs:

// benchmark!(code_block) -> Duration
// - Measures execution time of code block
// - Returns elapsed time in nanoseconds/milliseconds
// - Example: benchmark!({ expensive_operation(); }) -> Duration

// benchmark!(name, code_block) -> ()
// - Named benchmarking with automatic reporting
// - Prints benchmark results to stdout
// - Example: benchmark!("sorting", { data.sort(); })

// benchmark!(iterations, code_block) -> Vec<Duration>
// - Run benchmark multiple times for statistical analysis
// - Returns vector of individual run times
// - Example: benchmark!(100, { quick_function(); })