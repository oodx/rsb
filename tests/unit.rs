// Category orchestrator: unit tests
// This file enables category-level execution of unit tests

// Subdirectory-based modules
#[path = "unit/param/helpers.rs"]
mod param_helpers;

#[path = "unit/param/param_test.rs"]
mod param_test;

#[path = "unit/param/macro_import.rs"]
mod param_macro_import;

#[path = "unit/prompts/functions.rs"]
mod prompts_functions;

#[path = "unit/prompts/contexts.rs"]
mod prompts_contexts;

#[path = "unit/prompts/macros.rs"]
mod prompts_macros;

#[path = "unit/colors/runtime.rs"]
mod colors_runtime;

#[path = "unit/colors/sanity.rs"]
mod colors_sanity;

#[path = "unit/global/core.rs"]
mod global_core_unit;

#[path = "unit/date/date_test.rs"]
mod date_test;

#[path = "unit/string/string_test.rs"]
mod string_test;

#[path = "unit/string/invalid_glob_test.rs"]
mod string_invalid_glob_test;

#[path = "unit/string/ascii_filter_test.rs"]
mod string_ascii_filter_test;

#[path = "unit/string/macros.rs"]
mod string_macros;

#[path = "unit/string/errors_test.rs"]
mod string_errors_test;

#[path = "unit/string/case_test.rs"]
mod string_case_test;

#[path = "unit/tokens/comprehensive.rs"]
mod tokens_comprehensive;

// Root-level unit files
#[path = "unit/features_prompts.rs"]
mod features_prompts;

#[path = "unit/args_processing.rs"]
mod args_processing;

#[path = "unit/adapter_global_light.rs"]
mod adapter_global_light;

#[path = "unit/global_namespace.rs"]
mod global_namespace;

#[path = "unit/features_dispatch.rs"]
mod features_dispatch;

#[path = "unit/global_core.rs"]
mod global_core_root;

#[path = "unit/features_tokens.rs"]
mod features_tokens;

#[path = "unit/options.rs"]
mod options;