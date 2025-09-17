// Wrapper: features/prompts tests
#![cfg(feature = "prompts")]

// features/prompts modules moved under prompts/; point wrapper at the live files.
#[path = "prompts/macros.rs"]
mod features_prompts_macros;
#[path = "prompts/functions.rs"]
mod features_prompts_functions;
#[path = "prompts/contexts.rs"]
mod features_prompts_contexts;
