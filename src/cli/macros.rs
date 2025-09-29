// CLI Building Macros

// --- Bootstrap & Args ---
#[macro_export]
macro_rules! bootstrap {
    () => {{
        let args: Vec<String> = std::env::args().collect();
        $crate::cli::cli_bootstrap(&args);
        $crate::cli::Args::new(&args)
    }};
}

#[macro_export]
macro_rules! args {
    () => {
        std::env::args().collect::<Vec<String>>()
    };
}

#[macro_export]
macro_rules! appref {
    () => {
        std::env::args().next().unwrap_or_default()
    };
}

// --- Options Parsing ---
#[macro_export]
macro_rules! options {
    // Default form: uses strategy from config
    ($args:expr) => {{
        use $crate::cli::OptionsStrategy;
        let strategy = OptionsStrategy::from_config();
        let context = $crate::cli::options($args);
        $args.apply_options_strategy(strategy, &context);
    }};
    // Explicit strategy form
    ($args:expr, strategy: $strat:expr) => {{
        use $crate::cli::OptionsStrategy;
        let strategy = OptionsStrategy::from_str($strat);
        let context = $crate::cli::options($args);
        $args.apply_options_strategy(strategy, &context);
    }};
}

// Extended options macro with strategy selection
#[macro_export]
macro_rules! options_ex {
    // With explicit strategy
    ($args:expr, $strategy:expr) => {{
        use $crate::cli::OptionsStrategy;
        let context = $crate::cli::options($args);
        $args.apply_options_strategy($strategy, &context);
    }};
}

// --- Dispatch ---
#[macro_export]
macro_rules! dispatch {
    // Enhanced form: optional descriptions per command via `desc: "..."`
    ($args:expr, { $( $cmd:literal => $handler:ident $(, desc: $desc:expr )? ),* $(,)? }) => {{
        // Register handlers for introspection (ensures built-ins like inspect work)
        $crate::cli::register_handlers(&[$(($cmd, $handler)),*]);

        // If vanity descriptions are provided, store them (overrides empty entries)
        $(
            let _ = dispatch!(@maybe_register_desc $cmd $(, $desc)?);
        )*

        // Delegate to helper function with lookup closure
        $crate::cli::execute_dispatch($args, |command| {
            match command {
                $($cmd => Some($handler),)*
                _ => None,
            }
        });
    }};

    (@maybe_register_desc $name:expr) => { () };
    (@maybe_register_desc $name:expr, $d:expr) => {{ $crate::global::register_function($name, $d); }};
}

#[macro_export]
macro_rules! pre_dispatch {
    // Enhanced form with optional descriptions, mirrors dispatch! behavior
    ($args:expr, { $( $cmd:literal => $handler:ident $(, desc: $desc:expr )? ),* $(,)? }) => {{
        // Register handlers for introspection
        $crate::cli::register_handlers(&[$(($cmd, $handler)),*]);

        // If descriptions provided, record them
        $(
            let _ = pre_dispatch!(@maybe_register_desc $cmd $(, $desc)?);
        )*

        // Delegate to helper function with lookup closure
        $crate::cli::execute_pre_dispatch($args, |command| {
            match command {
                $($cmd => Some($handler),)*
                _ => None,
            }
        })
    }};

    // Back-compat: simple form without descriptions
    ($args:expr, { $($cmd:literal => $handler:ident),* }) => {{
        $crate::cli::register_handlers(&[$(($cmd, $handler)),*]);
        $crate::cli::execute_pre_dispatch($args, |command| {
            match command {
                $($cmd => Some($handler),)*
                _ => None,
            }
        })
    }};

    (@maybe_register_desc $name:expr) => { () };
    (@maybe_register_desc $name:expr, $d:expr) => {{ $crate::global::register_function($name, $d); }};
}


// --- CLI Args Access Macros (v0.7.0+) ---
// Provides convenient access to CLI arguments stored in global

/// Get CLI argument by position (1-based indexing, bash convention)
/// Returns empty string if argument doesn't exist
#[macro_export]
macro_rules! cli_arg {
    ($n:expr) => {{
        $crate::global::get_var(&format!("cli_arg_{}", $n))
    }};
}

/// Get total count of CLI arguments (excluding program name)
/// Returns 0 if cli_argc is not set or cannot be parsed as usize
#[macro_export]
macro_rules! cli_argc {
    () => {{
        let argc_str = $crate::global::get_var("cli_argc");
        if argc_str.is_empty() {
            0
        } else {
            argc_str.parse::<usize>().unwrap_or(0)
        }
    }};
}

/// Get all CLI arguments as semicolon-separated string (excluding program name)
#[macro_export]
macro_rules! cli_args {
    () => {{
        $crate::global::get_var("cli_args")
    }};
}

/// Get all CLI arguments as a Vec<String> (excluding program name)
#[macro_export]
macro_rules! cli_argv {
    () => {{
        let args_str = $crate::global::get_var("cli_args");
        if args_str.is_empty() {
            Vec::new()
        } else {
            args_str.split(';').map(String::from).collect::<Vec<String>>()
        }
    }};
}

/// Get the program name (argv[0])
#[macro_export]
macro_rules! cli_prog {
    () => {{
        $crate::global::get_var("cli_prog")
    }};
}

/// Check if a CLI argument exists at position n (1-based)
#[macro_export]
macro_rules! cli_has_arg {
    ($n:expr) => {{
        $crate::global::has_var(&format!("cli_arg_{}", $n))
    }};
}
