//! Host bootstrap orchestration
//!
//! Sequences environment import, XDG/RSB path setup, directory creation,
//! mode flags, script context, and args context.

/// Populate ARGC and ARGV_* in Global for convenience when scripting.
pub fn setup_args_context(args: &[String]) {
    crate::global::set_var("ARGC", &args.len().to_string());
    for (i, a) in args.iter().enumerate() {
        crate::global::set_var(&format!("ARGV_{}", i), a);
    }
}

/// Full host bootstrap; see RSB_BASHFX_ALIGN.md for details.
pub fn bootstrap(args: &[String]) {
    // 1) Env → Global
    crate::hosts::import_environment();

    // 2) XDG (XDG(0) + XDG(1))
    crate::hosts::setup_xdg_paths();

    // 3) RSB paths
    crate::hosts::setup_rsb_paths();

    // 4) Ensure dir structure exists
    crate::hosts::ensure_xdg_directories();

    // 5) Mode flags
    crate::hosts::setup_standard_modes();

    // 6) Script context
    crate::hosts::setup_execution_context(args);

    // 7) Args context
    setup_args_context(args);
}

/// Bootstrap using `std::env::args()`.
pub fn bootstrap_from_env() {
    let args: Vec<String> = std::env::args().collect();
    bootstrap(&args);
}

