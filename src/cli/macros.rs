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
macro_rules! appref { () => { std::env::args().next().unwrap_or_default() } }

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
    ($args:expr, { $($cmd:literal => $handler:ident),* }) => {{
        // Register handlers for introspection
        $crate::cli::register_handlers(&[$(($cmd, $handler)),*]);

        // Delegate to helper function with lookup closure
        $crate::cli::execute_dispatch($args, |command| {
            match command {
                $($cmd => Some($handler),)*
                _ => None,
            }
        });
    }};
}

#[macro_export]
macro_rules! pre_dispatch {
    ($args:expr, { $($cmd:literal => $handler:ident),* }) => {{
        // Delegate to helper function with lookup closure
        $crate::cli::execute_pre_dispatch($args, |command| {
            match command {
                $($cmd => Some($handler),)*
                _ => None,
            }
        })
    }};
}
