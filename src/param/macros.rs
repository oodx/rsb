// --- Parameter Expansion Macro ---
// Owned by param module; exported at crate root.

#[macro_export]
macro_rules! param {
    ($var:expr) => {
        $crate::param::basic::get($var)
    };
    ($var:expr, default: $default:expr) => {{
        let val = $crate::param::basic::get($var);
        if val.is_empty() {
            $default.to_string()
        } else {
            val
        }
    }};
    ($var:expr, alt: $alt:expr) => {{
        let val = $crate::param::basic::get($var);
        if val.is_empty() {
            String::new()
        } else {
            $alt.to_string()
        }
    }};

    // Substring: supports negative start/len via sub_rel
    ($var:expr, sub: $start:expr) => {
        $crate::param::basic::sub_rel(&$crate::param::basic::get($var), ($start) as isize, None)
    };
    ($var:expr, sub: $start:expr, $len:expr) => {
        $crate::param::basic::sub_rel(
            &$crate::param::basic::get($var),
            ($start) as isize,
            Some(($len) as isize),
        )
    };
    ($var:expr, prefix: $pattern:expr) => {
        $crate::param::basic::prefix(&$crate::param::basic::get($var), $pattern, false)
    };
    ($var:expr, prefix: $pattern:expr, longest) => {
        $crate::param::basic::prefix(&$crate::param::basic::get($var), $pattern, true)
    };
    ($var:expr, suffix: $pattern:expr) => {
        $crate::param::basic::suffix(&$crate::param::basic::get($var), $pattern, false)
    };
    ($var:expr, suffix: $pattern:expr, longest) => {
        $crate::param::basic::suffix(&$crate::param::basic::get($var), $pattern, true)
    };
    ($var:expr, replace: $from:expr => $to:expr) => {
        $crate::param::basic::replace(&$crate::param::basic::get($var), $from, $to, false)
    };
    ($var:expr, replace: $from:expr => $to:expr, all) => {
        $crate::param::basic::replace(&$crate::param::basic::get($var), $from, $to, true)
    };
    ($var:expr, upper) => {
        $crate::param::basic::upper(&$crate::param::basic::get($var), true)
    };
    ($var:expr, lower) => {
        $crate::param::basic::lower(&$crate::param::basic::get($var), true)
    };
    ($var:expr, upper: first) => {
        $crate::param::basic::upper(&$crate::param::basic::get($var), false)
    };
    ($var:expr, lower: first) => {
        $crate::param::basic::lower(&$crate::param::basic::get($var), false)
    };
    // Patterned first-match case transforms (${VAR^pat} / ${VAR,pat})
    ($var:expr, upper: $pattern:expr) => {
        $crate::param::basic::upper_pat_first(&$crate::param::basic::get($var), $pattern)
    };
    ($var:expr, lower: $pattern:expr) => {
        $crate::param::basic::lower_pat_first(&$crate::param::basic::get($var), $pattern)
    };

    ($var:expr, len) => {
        $crate::param::basic::len($var)
    };

    // Generalized case transforms
    ($var:expr, case: snake) => {
        $crate::string::to_snake_case(&$crate::param::basic::get($var))
    };
    ($var:expr, case: camel) => {
        $crate::string::to_camel_case(&$crate::param::basic::get($var))
    };
    ($var:expr, case: kebab) => {
        $crate::string::to_kebab_case(&$crate::param::basic::get($var))
    };
    ($var:expr, case: slug) => {
        $crate::string::to_kebab_case(&$crate::param::basic::get($var))
    };
    ($var:expr, case: dot) => {
        $crate::string::to_dot_case(&$crate::param::basic::get($var))
    };
    ($var:expr, case: space) => {
        $crate::string::to_space_case(&$crate::param::basic::get($var))
    };
    ($var:expr, case: lower) => {
        $crate::string::to_lower(&$crate::param::basic::get($var))
    };
    ($var:expr, case: upper) => {
        $crate::string::to_upper(&$crate::param::basic::get($var))
    };

    // Error if unset or empty (bash: ${VAR:?msg})
    ($var:expr, require: $msg:expr) => {{
        let val = $crate::param::basic::get($var);
        if val.is_empty() {
            $crate::stderr!("Required variable '{}' missing: {}", $var, $msg);
            std::process::exit(1);
        } else {
            val
        }
    }};
}
