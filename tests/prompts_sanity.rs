// Wrapper: prompts sanity tests - core functionality verification
#![cfg(feature = "prompts")]

#[path = "features/prompts/macros.rs"]
mod prompts_sanity_macros;
#[path = "features/prompts/functions.rs"]
mod prompts_sanity_functions;
#[path = "features/prompts/contexts.rs"]
mod prompts_sanity_contexts;