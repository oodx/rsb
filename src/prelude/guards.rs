//! Guards Prelude - Type Guards and Predicates
//!
//! Collects all `is_*` and `has_*` guard functions from across RSB modules
//! for convenient importing via `use rsb::prelude_guards::*;`

// === Boolean/Truth Guards (com) ===
pub use crate::com::{is_false, is_false_any, is_false_val, is_true, is_true_any, is_true_val};

// === Exit Code Guards (com) ===
pub use crate::com::{is_fail, is_success};

// === Filesystem Guards (fs) ===
pub use crate::fs::{
    is_dir, is_entity, is_executable, is_file, is_link, is_nonempty_file, is_readable, is_writable,
};

// === Host/Command Guards (hosts) ===
pub use crate::hosts::is_command;

// === Environment Guards (hosts) ===
pub use crate::hosts::has_env_var;

// === Global Store Guards (global) ===
pub use crate::global::{has_var, is_token_stream};

// === String Guards (string) ===
pub use crate::string::is_name;

// === Math Predicates (math) ===
pub use crate::math::{is_even, is_negative, is_odd, is_positive, is_prime, is_zero};

// === Token Guards (token) ===
pub use crate::token::is_token_streamable;

// === Color Guards (colors) ===
#[cfg(feature = "colors-core")]
pub use crate::colors::named::is_named_color;
#[cfg(feature = "colors-core")]
pub use crate::colors::simple::is_simple_color;
#[cfg(feature = "colors-core")]
pub use crate::colors::status::is_status_color;

// === TOML Guards (toml) ===
#[cfg(feature = "toml")]
pub use crate::toml::has_namespace;

// === CLI Option Guards (cli) ===
pub use crate::cli::has_option;
