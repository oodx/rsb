//! Host Interaction Macros
//! 
//! Macros to be moved FROM macros/core.rs:
//! - get_env!() -> Import environment variables
//! - bootstrap!() (host parts) -> host_bootstrap!()
//! 
//! Macros to be moved FROM macros/jobs_events.rs:
//! - hostname!() -> hostname!()
//! - user!() -> user!()  
//! - home_dir!() -> home_dir!()
//! - current_dir!() -> current_dir!()

// Import environment variables into Global via Host layer
#[macro_export]
macro_rules! get_env {
    () => { $crate::hosts::import_environment(); };
}

// Optional host-only bootstrap (without CLI)
#[macro_export]
macro_rules! host_bootstrap { () => { $crate::hosts::bootstrap_from_env(); } }

// TODO: Move hostname!() from macros/jobs_events.rs
// #[macro_export]
// macro_rules! hostname { 
//     () => { crate::host::system::get_hostname() }; 
// }

// TODO: Move user!() from macros/jobs_events.rs
// #[macro_export]
// macro_rules! user { 
//     () => { crate::host::system::get_username() }; 
// }

// TODO: Move home_dir!() from macros/jobs_events.rs
// #[macro_export]
// macro_rules! home_dir { 
//     () => { crate::host::host_path::get_home_dir() }; 
// }

// TODO: Move current_dir!() from macros/jobs_events.rs
// #[macro_export]
// macro_rules! current_dir { 
//     () => { crate::host::host_path::get_current_dir() }; 
// }
