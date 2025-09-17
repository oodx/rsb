// --- I/O Macros (core, visual macros split to macros/visual.rs) ---
// Namespaced re-exports for selective imports
pub use crate::{echo, printf, readline, stderr};
#[macro_export]
macro_rules! readline {
    () => {{
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => input.trim().to_string(),
            Err(_) => String::new(),
        }
    }};
    ($prompt:expr) => {{
        eprint!("{}", $prompt);
        let _ = std::io::stderr().flush();
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => input.trim().to_string(),
            Err(_) => String::new(),
        }
    }};
}

#[macro_export]
macro_rules! stderr {
    ($($arg:tt)*) => {
        {
            let msg = format!($($arg)*);
            eprintln!("{}", msg);
        }
    };
}

// Returns a string with color placeholders expanded, without printing.
// --- Output Macros (core only) ---
#[macro_export]
macro_rules! echo { ($($arg:tt)*) => { println!("{}", $crate::global::expand_vars(&format!($($arg)*))); }; }
#[macro_export]
macro_rules! printf { ($($arg:tt)*) => { print!("{}", $crate::global::expand_vars(&format!($($arg)*))); }; }

// --- Core logging fallbacks (only when visuals are disabled) ---
// Provide standard log macros that write to stderr using core utils.
// These are intentionally not compiled when the `visual` feature is enabled,
// to avoid macro name conflicts with the visual variants.
#[cfg(not(feature = "visual"))]
#[macro_export]
macro_rules! info { ($($arg:tt)*) => { $crate::utils::stderrx("info", &format!($($arg)*)); }; }
#[cfg(not(feature = "visual"))]
#[macro_export]
macro_rules! okay { ($($arg:tt)*) => { $crate::utils::stderrx("okay", &format!($($arg)*)); }; }
#[cfg(not(feature = "visual"))]
#[macro_export]
macro_rules! warn { ($($arg:tt)*) => { $crate::utils::stderrx("warn", &format!($($arg)*)); }; }
#[cfg(not(feature = "visual"))]
#[macro_export]
macro_rules! error { ($($arg:tt)*) => { $crate::utils::stderrx("error", &format!($($arg)*)); }; }
#[cfg(not(feature = "visual"))]
#[macro_export]
macro_rules! fatal { ($($arg:tt)*) => { $crate::utils::stderrx("fatal", &format!($($arg)*)); }; }
#[cfg(not(feature = "visual"))]
#[macro_export]
macro_rules! debug { ($($arg:tt)*) => { $crate::utils::stderrx("debug", &format!($($arg)*)); }; }
#[cfg(not(feature = "visual"))]
#[macro_export]
macro_rules! trace { ($($arg:tt)*) => { $crate::utils::stderrx("trace", &format!($($arg)*)); }; }
