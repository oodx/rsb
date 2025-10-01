// --- String Macros (owned by string module) ---

/// Convert a string slice to an `i32`, trimming whitespace.
/// Returns 0 on parse error unless a `default:` is provided.
#[macro_export]
macro_rules! to_number {
    ($text:expr) => {{
        $text.trim().parse::<i32>().unwrap_or(0)
    }};
    ($text:expr, default: $default:expr) => {{
        $text.trim().parse::<i32>().unwrap_or($default)
    }};
}

#[macro_export]
macro_rules! str_in {
    ($needle:expr, in: $haystack:expr) => {
        $haystack.contains($needle)
    };
}

#[macro_export]
macro_rules! str_explode {
    ($string:expr, on: $delim:expr, into: $arr_name:expr) => {{
        let items: Vec<&str> = $string.split($delim).collect();
        $crate::global::array::set_array($arr_name, &items);
    }};
}

#[macro_export]
macro_rules! str_trim {
    ($var:expr) => {
        $crate::global::get_var($var).trim().to_string()
    };
}

#[macro_export]
macro_rules! str_len {
    ($var:expr) => {
        $crate::global::get_var($var).len()
    };
}

#[macro_export]
macro_rules! str_line {
    ($ch:expr, $n:expr) => {{
        let c: char = $ch;
        std::iter::repeat(c).take($n as usize).collect::<String>()
    }};
}

// --- Case macros (value forms) ---
#[macro_export]
macro_rules! snake {
    ($s:expr) => {
        $crate::string::to_snake_case(&$s.to_string())
    };
}
#[macro_export]
macro_rules! kebab {
    ($s:expr) => {
        $crate::string::to_kebab_case(&$s.to_string())
    };
}
#[macro_export]
macro_rules! slug {
    ($s:expr) => {
        $crate::string::to_kebab_case(&$s.to_string())
    };
}
#[macro_export]
macro_rules! dot {
    ($s:expr) => {
        $crate::string::to_dot_case(&$s.to_string())
    };
}
#[macro_export]
macro_rules! space {
    ($s:expr) => {
        $crate::string::to_space_case(&$s.to_string())
    };
}
#[macro_export]
macro_rules! camel {
    ($s:expr) => {
        $crate::string::to_camel_case(&$s.to_string())
    };
}
#[macro_export]
macro_rules! pascal {
    ($s:expr) => {
        $crate::string::to_pascal_case(&$s.to_string())
    };
}
#[macro_export]
macro_rules! screaming {
    ($s:expr) => {
        $crate::string::to_screaming_snake_case(&$s.to_string())
    };
}

// --- Case macros (context var forms) ---
#[macro_export]
macro_rules! snake_var {
    ($name:expr) => {
        $crate::string::to_snake_case(&$crate::global::get_var($name))
    };
}
#[macro_export]
macro_rules! kebab_var {
    ($name:expr) => {
        $crate::string::to_kebab_case(&$crate::global::get_var($name))
    };
}
#[macro_export]
macro_rules! slug_var {
    ($name:expr) => {
        $crate::string::to_kebab_case(&$crate::global::get_var($name))
    };
}
#[macro_export]
macro_rules! dot_var {
    ($name:expr) => {
        $crate::string::to_dot_case(&$crate::global::get_var($name))
    };
}
#[macro_export]
macro_rules! space_var {
    ($name:expr) => {
        $crate::string::to_space_case(&$crate::global::get_var($name))
    };
}
#[macro_export]
macro_rules! camel_var {
    ($name:expr) => {
        $crate::string::to_camel_case(&$crate::global::get_var($name))
    };
}
#[macro_export]
macro_rules! pascal_var {
    ($name:expr) => {
        $crate::string::to_pascal_case(&$crate::global::get_var($name))
    };
}
#[macro_export]
macro_rules! screaming_var {
    ($name:expr) => {
        $crate::string::to_screaming_snake_case(&$crate::global::get_var($name))
    };
}

// --- Control Flow Macros ---
#[macro_export]
macro_rules! case {
    ($value:expr, { $($pattern:expr => $body:block),* $(, _ => $default:block)? }) => {
        {
            let val_to_match = $value;
            let mut matched = false;
            $(
                if !matched && $crate::string::str_matches(val_to_match, $pattern) {
                    matched = true;
                    $body
                }
            )*
            $(
                if !matched { $default }
            )?
        }
    };
}
