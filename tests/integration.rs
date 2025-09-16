// Category orchestrator: integration tests
// This file enables category-level execution of integration tests

// CLI integration tests (extracted from old CLI binary tests)
#[path = "integration/cli/help_and_commands.rs"]
mod cli_help_and_commands;

#[path = "integration/cli/config_and_meta.rs"]
mod cli_config_and_meta;

#[path = "integration/cli/array_system_macros.rs"]
mod cli_array_system_macros;

// Include existing integration test modules here when added
#[path = "integration/adapter_global.rs"]
mod adapter_global;

#[path = "integration/features_global.rs"]
mod features_global;

#[path = "integration/host_bootstrap.rs"]
mod host_bootstrap;

#[path = "integration/host_environment.rs"]
mod host_environment;

#[path = "integration/host_global.rs"]
mod host_global;

#[path = "integration/host_paths.rs"]
mod host_paths;
