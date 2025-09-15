//! Host Interaction Macros
//! 
//! Curated host-related macros. These wrap the `hosts` module utilities
//! and provide a stable, user-friendly surface aligned with MODULE_SPEC.

// Import environment variables into Global via Host layer
#[macro_export]
macro_rules! get_env {
    () => { $crate::hosts::import_environment(); };
}

// Optional host-only bootstrap (without CLI)
#[macro_export]
macro_rules! host_bootstrap { () => { $crate::hosts::bootstrap_from_env(); } }

// Host/system info wrappers
#[macro_export]
macro_rules! hostname { () => { $crate::hosts::get_hostname() }; }

#[macro_export]
macro_rules! user { () => { $crate::hosts::get_username() }; }

// Host path wrappers
#[macro_export]
macro_rules! home_dir { () => { $crate::hosts::host_path::get_home_dir() }; }

#[macro_export]
macro_rules! current_dir { () => { $crate::hosts::host_path::get_current_dir() }; }
