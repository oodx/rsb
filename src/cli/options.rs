//! Command Line Options Processing - ported from macros to function implementation.
//!
//! Provides the `options` function that works with the `options!` macro for parsing
//! command line arguments and setting global option variables.

use crate::cli::Args;
use crate::global;
use std::path::Path;

/// Parse command line options and set global option variables.
///
/// This function processes Args and sets global variables in the format `opt_<name>`
/// for each option found. It handles:
/// - Long options: `--verbose`, `--config=value`
/// - Short options: `-v`, `-d`
/// - Negation: `--not-verbose` sets `opt_verbose=1` (REBEL false)
/// - Multi-options: `--multi=a,b,!c` sets multiple flags
/// - Path validation: automatically validates path/file options
/// - Standard options: when `stdopts` feature is enabled
///
/// # Examples
/// ```rust
/// use rsb::cli::{Args, options};
///
/// let args = Args::new(&["bin".into(), "--verbose".into(), "--config=app.conf".into()]);
/// options(&args);
///
/// // Now opt_verbose and opt_config are set in global context
/// assert_eq!(rsb::param!("opt_verbose"), "0");
/// assert_eq!(rsb::param!("opt_config"), "app.conf");
/// ```
pub fn options(args: &Args) {
    for arg in args.all() {
        if arg.starts_with("--") {
            let arg_clean = &arg[2..];
            let (arg_key, _maybe_val) = if let Some(eq_pos) = arg_clean.find('=') {
                (&arg_clean[..eq_pos], Some(&arg_clean[eq_pos + 1..]))
            } else {
                (arg_clean, None)
            };

            // Handle negation: --not-verbose
            if arg_key.starts_with("not-") {
                let base = arg_key.trim_start_matches("not-").replace("-", "_");
                // REBEL boolean: 1 = false
                global::set_var(&format!("opt_{}", base), "1");
                continue;
            }

            if let Some(eq_pos) = arg_clean.find('=') {
                let opt_name = &arg_clean[..eq_pos];
                let opt_value = &arg_clean[eq_pos + 1..];

                // Handle multi-options: --multi=a,b,!c
                if opt_name == "multi" {
                    let mut neg = false;
                    for ch in opt_value.chars() {
                        if ch == ',' {
                            neg = false;
                            continue;
                        }
                        if ch == '!' {
                            neg = !neg;
                            continue;
                        }
                        if ch.is_ascii_alphabetic() {
                            let base_key = format!("opt_{}", ch);
                            // REBEL boolean: 0 = true, 1 = false
                            if neg {
                                global::set_var(&base_key, "1");
                            } else {
                                global::set_var(&base_key, "0");
                            }
                        }
                    }
                    continue;
                }

                // Path validation for path/file options
                if opt_name.contains("path") || opt_name.contains("file") {
                    let path = Path::new(opt_value);
                    if !path.exists() {
                        eprintln!("Path does not exist: {}", opt_value);
                        std::process::exit(1);
                    }
                }

                global::set_var(&format!("opt_{}", opt_name.replace("-", "_")), opt_value);
            } else {
                // Flag option (no value) — REBEL boolean: 0 = true
                global::set_var(&format!("opt_{}", arg_clean.replace("-", "_")), "0");
            }
        } else if arg.starts_with("-") && arg.len() == 2 {
            // Short option
            let opt_char = &arg[1..2];
            // REBEL boolean: 0 = true
            global::set_var(&format!("opt_{}", opt_char), "0");

            // Standard options mapping (when stdopts feature is enabled)
            #[cfg(feature = "stdopts")]
            match opt_char {
                // REBEL boolean: 0 = true
                "d" => global::set_var("opt_debug", "0"),
                "q" => global::set_var("opt_quiet", "0"),
                "t" => global::set_var("opt_trace", "0"),
                "D" => global::set_var("opt_dev_mode", "0"),
                "y" => global::set_var("opt_yes", "0"),
                "s" => global::set_var("opt_safe", "0"),
                _ => {}
            }
        }
    }
}

/// Check if an option was provided (convenience function).
///
/// # Examples
/// ```rust
/// use rsb::cli::{Args, options, has_option};
///
/// let args = Args::new(&["bin".into(), "--verbose".into()]);
/// options(&args);
/// assert!(has_option("verbose"));
/// assert!(!has_option("quiet"));
/// ```
pub fn has_option(name: &str) -> bool {
    crate::global::is_true_val(global::get_var(&format!("opt_{}", name)))
}

/// Get option value if it was provided.
///
/// # Examples
/// ```rust
/// use rsb::cli::{Args, options, get_option_value};
///
/// let args = Args::new(&["bin".into(), "--config=app.conf".into()]);
/// options(&args);
/// assert_eq!(get_option_value("config"), Some("app.conf".to_string()));
/// assert_eq!(get_option_value("missing"), None);
/// ```
pub fn get_option_value(name: &str) -> Option<String> {
    let value = global::get_var(&format!("opt_{}", name));
    if value.is_empty() || value == "0" {
        None
    } else {
        Some(value)
    }
}
