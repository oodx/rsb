// Sleep Operations (from macros/time_math.rs)

// MACRO TO MOVE FROM macros/time_math.rs:

// sleep!(ms) -> ()
// - Sleep for specified milliseconds
// - Example: sleep!(100) -> sleeps for 100ms

// sleep!(seconds) -> ()
// - Sleep for specified seconds
// - Example: sleep!(2) -> sleeps for 2 seconds

// sleep!("5s") -> ()
// - Sleep with time unit string parsing
// - Supports: ms, s, m, h units
// - Example: sleep!("1.5s") -> sleeps for 1.5 seconds