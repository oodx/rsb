//! Basic helpers for param expansions (used by `param!`).

use crate::global::get_var as get_var_ctx;
use crate::string::{
    str_case_first_match as util_case_first_match, str_lower as util_lower,
    str_prefix as util_prefix, str_replace as util_replace, str_sub as util_sub,
    str_suffix as util_suffix, str_upper as util_upper,
};

pub fn get(name: &str) -> String {
    get_var_ctx(name)
}

pub fn sub(s: &str, start: usize, len: Option<usize>) -> String {
    util_sub(s, start, len)
}

// Relative substring: supports negative start/length (count from end)
pub fn sub_rel(s: &str, start: isize, len: Option<isize>) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len() as isize;
    if n == 0 {
        return String::new();
    }

    let mut i = if start < 0 { n + start } else { start };
    if i < 0 {
        i = 0;
    }
    if i > n {
        i = n;
    }

    let j = match len {
        None => n,
        Some(l) => {
            if l < 0 {
                // negative length: take until n + l (exclusive)
                let end = n + l;
                if end < i {
                    i
                } else {
                    end
                }
            } else {
                let end = i + l;
                if end > n {
                    n
                } else {
                    end
                }
            }
        }
    };

    chars[i as usize..j as usize].iter().collect()
}

pub fn prefix(s: &str, pattern: &str, longest: bool) -> String {
    util_prefix(s, pattern, longest)
}

pub fn suffix(s: &str, pattern: &str, longest: bool) -> String {
    util_suffix(s, pattern, longest)
}

pub fn replace(s: &str, from: &str, to: &str, all: bool) -> String {
    util_replace(s, from, to, all)
}

pub fn upper(s: &str, all: bool) -> String {
    util_upper(s, all)
}

pub fn lower(s: &str, all: bool) -> String {
    util_lower(s, all)
}

pub fn len(name: &str) -> usize {
    get(name).len()
}

// Patterned first-match case transforms
pub fn upper_pat_first(s: &str, pattern: &str) -> String {
    util_case_first_match(s, pattern, true)
}

pub fn lower_pat_first(s: &str, pattern: &str) -> String {
    util_case_first_match(s, pattern, false)
}
