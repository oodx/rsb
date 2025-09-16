// Category orchestrator: sanity tests
// This file includes all sanity test modules for category-level execution

#[path = "sanity/baseline.rs"]
mod baseline;

#[path = "sanity/bash.rs"]
mod bash;

#[path = "sanity/com.rs"]
mod com;

#[path = "sanity/core.rs"]
mod core;

#[path = "sanity/global_adapter.rs"]
mod global_adapter;

#[path = "sanity/host_env.rs"]
mod host_env;

#[path = "sanity/host_paths.rs"]
mod host_paths;

#[path = "sanity/threads.rs"]
mod threads;

// NEW: GX module subdirectory
#[path = "sanity/gx/string.rs"]
mod gx_string;

#[path = "sanity/gx/collection.rs"]
mod gx_collection;

#[path = "sanity/gx/id.rs"]
mod gx_id;

// NEW: Math module subdirectory
#[path = "sanity/math/basic.rs"]
mod math_basic;

#[path = "sanity/math/random.rs"]
mod math_random;

#[path = "sanity/math/integers.rs"]
mod math_integers;

#[path = "sanity/math/aggregators.rs"]
mod math_aggregators;

#[path = "sanity/math/percentage.rs"]
mod math_percentage;

#[path = "sanity/math/expressions.rs"]
mod math_expressions;

#[path = "sanity/math/base.rs"]
mod math_base;

#[path = "sanity/math/predicates.rs"]
mod math_predicates;

// NEW: Tokens module subdirectory
#[path = "sanity/tokens/basic.rs"]
mod tokens_basic;

// Date module sanity tests
#[path = "sanity/date.rs"]
mod date;

// Math module sanity tests
#[path = "sanity/math.rs"]
mod math;