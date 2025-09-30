//! Math Module - Mathematical operations organized by type (MODULE_SPEC compliant)
//!
//! This module provides mathematical operations organized into specialized packages.
//! Each package focuses on a specific domain of mathematical operations.
//!
//! ## Architecture (MODULE_SPEC compliant)
//! - `mod.rs` - Orchestrator and curated public surface (this file)
//! - `basic/` - Core arithmetic: add, subtract, multiply, divide, power, sqrt
//! - `integers/` - Integer operations: gcd, lcm, prime, factorial, fibonacci
//! - `expressions/` - Expression evaluator with variable support
//! - `base/` - Number base conversions: hex, binary, octal, arbitrary base
//! - `percentage/` - Percentage and ratio calculations
//! - `predicates/` - Boolean tests: even, odd, sign, modulo
//! - `aggregators/` - List aggregations: min, max, avg, median, sum
//! - `random/` - Random number generation with type support
//! - `macros.rs` - Module-owned macros for all packages
//!
//! ## Usage
//! ```rust
//! use rsb::math::*;
//!
//! // Basic operations
//! let sum = add(5.0, 3.0);
//!
//! // Integer operations
//! let g = gcd(48, 18);
//!
//! // Percentages
//! let result = percent_of(100.0, 25.0);  // 25.0
//!
//! // Aggregators
//! let numbers = vec![1.0, 2.0, 3.0, 4.0];
//! let average = avg(&numbers).unwrap();
//!
//! // Random lists
//! let random_ints = random_list_string("int", 5, Some("1:100"));
//! ```

// MODULE_SPEC: Implementation packages
pub mod aggregators;
pub mod base;
pub mod basic;
pub mod expressions;
pub mod integers;
pub mod macros;
pub mod percentage;
pub mod predicates;
// random moved to gx::rand (generators, not math operations)

// MODULE_SPEC: Curated public surface - re-export key functions

// Basic operations
pub use basic::{abs, add, divide, max, min, multiply, power, sqrt, subtract};
pub use basic::{calc, ceil, eval_var, floor, parse_number, round, rounddown, roundup};

// Integer operations
pub use integers::{factorial, factors, fibonacci, gcd, is_prime, lcm, sum_range};
pub use integers::{
    int_add, int_calc, int_divide, int_multiply, int_parse, int_power, int_subtract,
};

// Expression evaluation
pub use expressions::evaluate_expression;

// Base conversion operations
pub use base::{base_convert, from_base, from_binary, from_hex, from_octal, to_base};
pub use base::{to_binary, to_hex, to_hex_upper, to_octal};

// Percentage operations
pub use percentage::{
    decimal_to_percentage, percent_change, percent_of, percentage_to_decimal, ratio,
};

// Predicate operations
pub use predicates::{is_even, is_negative, is_odd, is_positive, is_zero, modulo, same_sign, sign};

// Aggregator operations
pub use aggregators::{avg, max_list, mean, median, min_list, sum_list};

// Random operations moved to gx::rand module

// Note: Macros are exported at crate root via src/macros/mod.rs
