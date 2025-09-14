//! Math module sanity tests - wrapper file
//! Tests core functionality of all math packages

#[path = "math/sanity/basic.rs"]
mod math_basic_sanity;

#[path = "math/sanity/integers.rs"]
mod math_integers_sanity;

#[path = "math/sanity/base.rs"]
mod math_base_sanity;

#[path = "math/sanity/percentage.rs"]
mod math_percentage_sanity;

#[path = "math/sanity/predicates.rs"]
mod math_predicates_sanity;

#[path = "math/sanity/aggregators.rs"]
mod math_aggregators_sanity;

#[path = "math/sanity/random.rs"]
mod math_random_sanity;

#[path = "math/sanity/expressions.rs"]
mod math_expressions_sanity;