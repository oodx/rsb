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
pub mod basic;
pub mod integers;
pub mod expressions;
pub mod base;
pub mod percentage;
pub mod predicates;
pub mod aggregators;
pub mod random;
pub mod macros;  // Module-owned macros

// MODULE_SPEC: Curated public surface - re-export key functions

// Basic operations
pub use basic::{add, subtract, multiply, divide, power, sqrt, abs, min, max};
pub use basic::{round, roundup, rounddown, floor, ceil, parse_number, calc, eval_var};

// Integer operations
pub use integers::{gcd, lcm, is_prime, factorial, fibonacci, factors, sum_range};
pub use integers::{int_add, int_subtract, int_multiply, int_divide, int_power, int_parse, int_calc};

// Expression evaluation
pub use expressions::evaluate_expression;

// Base conversion operations
pub use base::{to_hex, to_hex_upper, to_binary, to_octal};
pub use base::{from_hex, from_binary, from_octal, to_base, from_base, base_convert};

// Percentage operations
pub use percentage::{percent_of, percent_change, ratio, percentage_to_decimal, decimal_to_percentage};

// Predicate operations
pub use predicates::{is_even, is_odd, modulo, sign, same_sign, is_positive, is_negative, is_zero};

// Aggregator operations
pub use aggregators::{min_list, max_list, sum_list, avg, mean, median};

// Random operations
pub use random::{random_range, random_int_range, random_list_float, random_list_int, random_list_bool, random_list_string};

// Note: Macros are exported at crate root via src/macros/mod.rs

