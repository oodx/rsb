//! String-to-Number Comparison Utilities
//!
//! Parse strings as f64 and compare numerically.

fn to_f64(s: &str) -> Option<f64> {
    s.parse::<f64>().ok()
}

/// Compare two string numbers for equality (within f64::EPSILON)
pub fn num_eq(a: &str, b: &str) -> bool {
    match (to_f64(a), to_f64(b)) {
        (Some(na), Some(nb)) => (na - nb).abs() < f64::EPSILON,
        _ => false,
    }
}

/// Check if string number a is less than b
pub fn num_lt(a: &str, b: &str) -> bool {
    match (to_f64(a), to_f64(b)) {
        (Some(na), Some(nb)) => na < nb,
        _ => false,
    }
}

/// Check if string number a is greater than b
pub fn num_gt(a: &str, b: &str) -> bool {
    match (to_f64(a), to_f64(b)) {
        (Some(na), Some(nb)) => na > nb,
        _ => false,
    }
}
