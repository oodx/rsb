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
    ($args:expr) => {{
        $crate::cli::options($args);
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
