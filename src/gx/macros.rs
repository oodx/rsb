// --- GX Macros (module-owned) ---
// Random data helpers and simple dictionary generators/selections.

// Namespaced re-exports for selective imports
pub use crate::{
    rand_alnum, rand_alpha, rand_hex, rand_string, rand_uuid, rand_range,
    rand_dict, gen_dict,
};

// --- Random Data Macros ---
#[macro_export]
macro_rules! rand_alnum { ($n:expr) => { $crate::gx::string::get_rand_alnum($n) }; }

#[macro_export]
macro_rules! rand_alpha { ($n:expr) => { $crate::gx::string::get_rand_alpha($n) }; }

#[macro_export]
macro_rules! rand_hex { ($n:expr) => { $crate::gx::string::get_rand_hex($n) }; }

#[macro_export]
macro_rules! rand_string { ($n:expr) => { $crate::gx::string::get_rand_string($n) }; }

#[macro_export]
macro_rules! rand_uuid { () => { $crate::gx::id::get_rand_uuid() }; }

// --- Simple random range macro (inclusive) ---
#[macro_export]
macro_rules! rand_range { ($min:expr, $max:expr) => {{ $crate::random::rand_range_usize($min as usize, $max as usize) }}; }

// --- Dictionary Macros (array-backed and generators) ---
#[macro_export]
macro_rules! rand_dict {
    ($arr_name:expr) => {
        $crate::gx::collection::get_rand_from_slice(&$crate::utils::get_array($arr_name)).unwrap_or_default()
    };
    ($arr_name:expr, $n:expr) => { $crate::rand_dict!($arr_name, $n, " ") };
    ($arr_name:expr, $n:expr, $delim:expr) => {{
        let words = $crate::utils::get_array($arr_name);
        if words.is_empty() {
            String::new()
        } else {
            let mut result = Vec::new();
            for _ in 0..$n {
                result.push($crate::gx::collection::get_rand_from_slice(&words).unwrap_or_default());
            }
            result.join($delim)
        }
    }};
}

#[macro_export]
macro_rules! gen_dict {
    ($type:ident, $n:expr, into: $arr_name:expr) => {{
        let mut words = Vec::new();
        for _ in 0..$n {
            let len = $crate::rand_range!(4, 8);
            let word = match stringify!($type) {
                "alnum" => $crate::gx::string::get_rand_alnum(len),
                "alpha" => $crate::gx::string::get_rand_alpha(len),
                "hex" => $crate::gx::string::get_rand_hex(len),
                "string" => $crate::gx::string::get_rand_string(len),
                _ => String::new(),
            };
            words.push(word);
        }
        let word_strs: Vec<&str> = words.iter().map(|s| s.as_str()).collect();
        $crate::utils::set_array($arr_name, &word_strs);
    }};
}

