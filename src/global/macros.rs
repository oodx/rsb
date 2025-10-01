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

// --- Array Iteration Macros ---
#[macro_export]
macro_rules! for_in {
    ($var:ident in $array_name:expr => $body:block) => {
        for item in $crate::global::array::get_array($array_name) {
            $crate::global::set_var(stringify!($var), &item);
            $body
        }
    };
    ($index:ident, $var:ident in $array_name:expr => $body:block) => {
        for (i, item) in $crate::global::array::get_array($array_name).iter().enumerate() {
            $crate::global::set_var(stringify!($index), &i.to_string());
            $crate::global::set_var(stringify!($var), item);
            $body
        }
    };
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
pub use crate::{echo, export, for_in, load_config, printf, require_var, src};
