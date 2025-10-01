#[macro_export]
macro_rules! is_true {
    (var: $key:expr) => {
        $crate::com::is_true($key)
    };
    (true) => {
        true
    };
    (false) => {
        false
    };
    ($v:expr) => {
        $crate::com::is_true_any(&$v)
    };
}

#[macro_export]
macro_rules! is_false {
    (var: $key:expr) => {
        $crate::com::is_false($key)
    };
    (true) => {
        false
    };
    (false) => {
        true
    };
    ($v:expr) => {
        $crate::com::is_false_any(&$v)
    };
}

// --- Validation Macros ---
#[macro_export]
macro_rules! validate {
    ($condition:expr, $($arg:tt)*) => {
        if !$condition {
            $crate::error!("Validation failed: {}", format!($($arg)*));

            // Detect if running in test environment
            let is_test = std::env::var("CARGO_TEST").is_ok() || std::thread::current().name().map_or(false, |name| name.contains("test"));
            if !is_test {
                std::process::exit(1);
            } else {
                panic!("Validation failed: {}", format!($($arg)*));
            }
        }
    };
    ($condition:expr, exit_code: $code:expr, $($arg:tt)*) => {
        if !$condition {
            $crate::error!("Validation failed: {}", format!($($arg)*));

            // Detect if running in test environment
            let is_test = std::env::var("CARGO_TEST").is_ok() || std::thread::current().name().map_or(false, |name| name.contains("test"));
            if !is_test {
                std::process::exit($code);
            } else {
                panic!("Validation failed: {}", format!($($arg)*));
            }
        }
    };
}
