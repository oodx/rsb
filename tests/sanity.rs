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