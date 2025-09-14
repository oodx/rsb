// src/prelude.rs

//! The RSB prelude.
//!
//! This module re-exports all the common traits, functions, and macros
//! for easy importing into user code via `use rsb::prelude::*;`.

// Re-export all public structs and functions.
pub use crate::cli::Args;
// Global surfaces (store/expansion/config/helpers)
pub use crate::global::{
    expand_vars, export_vars, get_var, has_var, is_token_stream, load_config_file, parse_config_content,
    save_config_file, set_var, unset_var, is_true, is_false,
};
pub use crate::fs::*;
pub use crate::os::*;
pub use crate::streamable::{
    Streamable, StreamApply,
    // Advanced streamables for Pattern 2 & 3 composability
    Replace, UpperCase, LowerCase, Trim, Reverse,
    Base64Encode, Base64Decode, UrlEncode, UrlDecode,
    Head, Tail, Grep, Sort, Unique, WordCount,
    Sed, SedLines,
};

// Re-export the streamable! macro
pub use crate::streamable;
pub use crate::streams::Stream;
// Expose module namespaces and alias their utils to avoid glob ambiguity
pub use crate::date as date;
pub use crate::date::utils as date_utils;
pub use crate::string as string;
pub use crate::string::utils as string_utils;
// Bring common string helpers into prelude without exposing module `utils`
pub use crate::string::{
    str_sub, str_prefix, str_suffix, str_replace, str_upper, str_lower,
};
// Unified top-level utils remain available
pub use crate::utils::*;
pub use crate::xcls::{xsed, XSed, ToXSed};
pub use crate::random::*;
pub use crate::math::*;

// Re-export external dependencies so users importing the prelude
// also get convenient access to third-party crates RSB depends on.
pub use crate::deps::*;

// Re-export all macros.
pub use crate::{
    appref, args, backup, benchmark, bootstrap, case, cap_stream, cat, chmod, cmd,
    current_dir, curl, dict, dispatch, echo, event, export, file_in, for_in, get, get_env, gen_dict, home_dir, hostname,
    job, json_get, json_get_file,
    kill_pid, kill_process, load_config, lock, math, mock_cmd, options, pack, param,
    pid_of, pipe, pre_dispatch, printf, process_exists,
    rand_alnum, rand_alpha, rand_dict, rand_hex, rand_string, rand_uuid,
    rand_range,
    readline, require_command, require_dir, require_file, require_var, sed_around, sed_around_file,
    sed_insert, sed_insert_file, sed_lines, sed_lines_file, sed_replace, sed_template,
    sed_template_file, run, shell, src, stderr, stream, str_explode, str_in, str_len,
    str_trim, str_line, subst, tar, tar_gz, test, tmp, to_number, trap, unpack, unlock, user, validate,
    with_lock, zip, sleep, path_canon, path_split, meta_keys,
    // string case macros
    snake, kebab, slug, dot, space, camel,
    snake_var, kebab_var, slug_var, dot_var, space_var, camel_var,
};

// Global core surface (store/expansion/helpers)
pub use crate::global::*;

// Note: Visual macros (colored!, info!, etc.) are NOT re-exported in the prelude.
// Visual and other optional packages are opt-in via explicit imports.

// Re-export macro groups for selective imports via the prelude.
pub mod macros {
    // Legacy grouped macros (core, text, fs_data, etc.)
    pub use crate::macros::*;
    // Module-owned macros not in crate::macros
    pub use crate::{
        // param module
        param,
        // string module
        str_in, str_explode, str_trim, str_len, str_line,
        // case macros
        snake, kebab, slug, dot, space, camel,
        snake_var, kebab_var, slug_var, dot_var, space_var, camel_var,
        // date module (macros exported at crate root; import optional)
    };
}
