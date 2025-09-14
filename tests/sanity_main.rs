// Wrapper: sanity package (core + baseline demos)

#[path = "sanity.rs"]
mod sanity_core;

#[path = "sanity/baseline.rs"]
mod sanity_baseline;

#[path = "sanity/host_env.rs"]
mod sanity_host_env;

#[path = "sanity/global_adapter.rs"]
mod sanity_global_adapter;

#[path = "sanity/host_paths.rs"]
mod sanity_host_paths;
