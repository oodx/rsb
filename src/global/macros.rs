//! Global variable and config macros

// --- Output Macros (expand_vars wrappers) ---
#[macro_export]
macro_rules! echo {
    ($($arg:tt)*) => {
        println!("{}", $crate::global::expand_vars(&format!($($arg)*)));
    };
}

#[macro_export]
macro_rules! printf {
    ($($arg:tt)*) => {
        print!("{}", $crate::global::expand_vars(&format!($($arg)*)));
    };
}

// --- Config Macros ---
#[macro_export]
macro_rules! export {
    () => {
        $crate::global::export_vars(&$crate::global::get_var("RSB_EXPORT"));
    };
    ($path:expr) => {
        $crate::global::export_vars($path);
    };
}

#[macro_export]
macro_rules! src {
    ($($path:expr),+) => { $crate::load_config!($($path),+); };
}

#[macro_export]
macro_rules! load_config {
    ($($path:expr),+) => { $( $crate::global::load_config_file($path); )+ };
}

// --- Validation Macros ---
#[macro_export]
macro_rules! require_var {
    ($var:expr) => {
        $crate::validate!(
            $crate::global::has_var($var),
            "Required variable not set: {}",
            $var
        );
    };
}

// Namespaced re-exports for selective imports
pub use crate::{echo, export, load_config, printf, require_var, src};
