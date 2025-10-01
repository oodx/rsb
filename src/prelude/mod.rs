// src/prelude/mod.rs

//! The RSB prelude module.
//!
//! Contains multiple prelude variants for different use cases:
//! - `prelude::*` (this module) - Standard prelude for production
//! - `prelude::guards::*` - All is_* and has_* guard functions
//! - `prelude::dev::*` - Development and testing helpers
//! - `prelude::ez::*` - Everything (convenience prelude)
//!
//! The standard prelude re-exports all the common traits, functions, and macros
//! for easy importing into user code via `use rsb::prelude::*;`.

pub mod dev;
pub mod ez;
pub mod guards;

// Re-export all public structs and functions.
pub use crate::cli::Args;
// Global surfaces (store/expansion/config/helpers)
pub use crate::fs::*;
pub use crate::global::{
    clear_all, clear_pattern, clear_prefix, clear_suffix,
    expand_vars, export_vars, get_var, has_var, is_false, is_token_stream, is_true,
    load_config_file, parse_config_content, save_config_file, set_var, unset_var,
    ns_get, ns_get_all, ns_set,
};
// OS/host functions now under hosts module
pub use crate::hosts::*;
pub use crate::streamable::{
    Base64Decode,
    Base64Encode,
    Grep,
    Head,
    LowerCase,
    // Advanced streamables for Pattern 2 & 3 composability
    Replace,
    Reverse,
    Sed,
    SedLines,
    Sort,
    StreamApply,
    Streamable,
    Tail,
    Trim,
    Unique,
    UpperCase,
    UrlDecode,
    UrlEncode,
    WordCount,
};

// Re-export the streamable! macro
pub use crate::streamable;
pub use crate::streams::Stream;
// Expose module namespaces and alias their utils to avoid glob ambiguity
pub use crate::date;
pub use crate::date::utils as date_utils;
pub use crate::string;
pub use crate::string::utils as string_utils;
// Bring common string helpers into prelude without exposing module `utils`
pub use crate::string::{str_lower, str_prefix, str_replace, str_sub, str_suffix, str_upper};
// Unified top-level utils remain available
pub use crate::math::*;
// random module removed - use rsb::gx for generators
pub use crate::utils::*;
pub use crate::xcls::{xsed, ToXSed, XSed};

// Re-export external dependencies so users importing the prelude
// also get convenient access to third-party crates RSB depends on.
pub use crate::deps::*;

// Re-export all macros.
pub use crate::{
    appref,
    args,
    backup,
    benchmark,
    bootstrap,
    cli_arg,
    cli_argc,
    cli_args,
    cli_argv,
    cli_has_arg,
    cli_prog,
    camel,
    camel_var,
    cap_stream,
    case,
    cat,
    chmod,
    cmd,
    curl,
    current_dir,
    dict,
    dispatch,
    dot,
    dot_var,
    echo,
    event,
    export,
    file_in,
    for_in,
    gen_dict,
    get,
    get_env,
    home_dir,
    hostname,
    is_false,
    // boolean helpers
    is_true,
    job,
    json_get,
    json_get_file,
    kebab,
    kebab_var,
    kill_pid,
    kill_process,
    load_config,
    lock,
    math,
    meta_keys,
    mock_cmd,
    options,
    pack,
    param,
    path_canon,
    path_split,
    pid_of,
    pipe,
    pre_dispatch,
    printf,
    process_exists,
    repl_arg,
    repl_argc,
    repl_args,
    repl_argv,
    repl_dispatch,
    rand_alnum,
    rand_alpha,
    rand_dict,
    rand_hex,
    rand_range,
    rand_string,
    rand_uuid,
    readline,
    require_command,
    require_dir,
    require_file,
    require_var,
    run,
    sed_around,
    sed_around_file,
    sed_insert,
    sed_insert_file,
    sed_lines,
    sed_lines_file,
    sed_read,
    sed_replace,
    sed_template,
    sed_template_file,
    shell,
    sleep,
    slug,
    slug_var,
    // string case macros
    snake,
    snake_var,
    space,
    space_var,
    src,
    stderr,
    str_explode,
    str_in,
    str_len,
    str_line,
    str_trim,
    stream,
    subst,
    tar,
    tar_gz,
    test,
    tmp,
    to_number,
    trap,
    unlock,
    unpack,
    user,
    validate,
    // fs counters
    wc,
    wc_chars,
    wc_chars_file,
    wc_file,
    wc_lines,
    wc_lines_file,
    wc_words,
    wc_words_file,
    with_lock,
    zip,
};

// Global core surface (store/expansion/helpers)
pub use crate::global::*;

// Object type for flexible configuration
#[cfg(feature = "object")]
pub use crate::object::{
    Object, AnyObject, HubConfig, InfConfig, RsbConfig,
    HubShape, InfShape, RsbShape,
    get_object, get_hub, get_inf, get_rsb,
};

// REPL support for interactive command processing
pub use crate::repl::{Repl, ReplParser, ReplResult, SimpleParser, store_repl_args_global};

// Object-related macros are already exported at crate level via #[macro_export]
// They don't need to be re-exported here

// Note: Visual macros (colored!, info!, etc.) are NOT re-exported in the prelude.
// Visual and other optional packages are opt-in via explicit imports.

// Re-export macro groups for selective imports via the prelude.
pub mod macros {
    // Module-owned macros (now distributed across domain modules)
    pub use crate::{
        camel,
        camel_var,
        // date module (macros exported at crate root; import optional)
        dot,
        dot_var,
        kebab,
        kebab_var,
        // param module
        param,
        slug,
        slug_var,
        // case macros
        snake,
        snake_var,
        space,
        space_var,
        str_explode,
        // string module
        str_in,
        str_len,
        str_line,
        str_trim,
    };
}
