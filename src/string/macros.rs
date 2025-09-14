// --- String Macros (owned by string module) ---

#[macro_export]
macro_rules! str_in {
    ($needle:expr, in: $haystack:expr) => { $haystack.contains($needle) };
}

#[macro_export]
macro_rules! str_explode {
    ($string:expr, on: $delim:expr, into: $arr_name:expr) => {{
        let items: Vec<&str> = $string.split($delim).collect();
        $crate::utils::set_array($arr_name, &items);
    }};
}

#[macro_export]
macro_rules! str_trim {
    ($var:expr) => { $crate::global::get_var($var).trim().to_string() };
}

#[macro_export]
macro_rules! str_len {
    ($var:expr) => { $crate::global::get_var($var).len() };
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
macro_rules! snake { ($s:expr) => { $crate::string::to_snake_case(&$s.to_string()) }; }
#[macro_export]
macro_rules! kebab { ($s:expr) => { $crate::string::to_kebab_case(&$s.to_string()) }; }
#[macro_export]
macro_rules! slug { ($s:expr) => { $crate::string::to_kebab_case(&$s.to_string()) }; }
#[macro_export]
macro_rules! dot { ($s:expr) => { $crate::string::to_dot_case(&$s.to_string()) }; }
#[macro_export]
macro_rules! space { ($s:expr) => { $crate::string::to_space_case(&$s.to_string()) }; }
#[macro_export]
macro_rules! camel { ($s:expr) => { $crate::string::to_camel_case(&$s.to_string()) }; }

// --- Case macros (context var forms) ---
#[macro_export]
macro_rules! snake_var { ($name:expr) => { $crate::string::to_snake_case(&$crate::global::get_var($name)) }; }
#[macro_export]
macro_rules! kebab_var { ($name:expr) => { $crate::string::to_kebab_case(&$crate::global::get_var($name)) }; }
#[macro_export]
macro_rules! slug_var { ($name:expr) => { $crate::string::to_kebab_case(&$crate::global::get_var($name)) }; }
#[macro_export]
macro_rules! dot_var { ($name:expr) => { $crate::string::to_dot_case(&$crate::global::get_var($name)) }; }
#[macro_export]
macro_rules! space_var { ($name:expr) => { $crate::string::to_space_case(&$crate::global::get_var($name)) }; }
#[macro_export]
macro_rules! camel_var { ($name:expr) => { $crate::string::to_camel_case(&$crate::global::get_var($name)) }; }
