// --- I/O Macros (core, visual macros split to macros/visual.rs) ---
// Namespaced re-exports for selective imports
pub use crate::{readline, stderr, echo, printf};
#[macro_export]
macro_rules! readline {
    () => {
        {
            let mut input = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => input.trim().to_string(),
                Err(_) => String::new(),
            }
        }
    };
    ($prompt:expr) => {
        {
            eprint!("{}", $prompt);
            let _ = std::io::stderr().flush();
            let mut input = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => input.trim().to_string(),
                Err(_) => String::new(),
            }
        }
    };
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
