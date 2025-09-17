//! Predicates Package
//!
//! Boolean test functions: even, odd, modulo, sign tests

pub fn is_even(n: i64) -> bool {
    n % 2 == 0
}

pub fn is_odd(n: i64) -> bool {
    n % 2 != 0
}

pub fn modulo(a: i64, b: i64) -> Result<i64, String> {
    if b == 0 {
        Err("Division by zero in modulo".to_string())
    } else {
        Ok(a % b)
    }
}

pub fn sign(n: f64) -> i32 {
    if n > 0.0 {
        1
    } else if n < 0.0 {
        -1
    } else {
        0
    }
}

pub fn same_sign(a: f64, b: f64) -> bool {
    sign(a) == sign(b)
}

pub fn is_positive(n: f64) -> bool {
    n > 0.0
}

pub fn is_negative(n: f64) -> bool {
    n < 0.0
}

pub fn is_zero(n: f64) -> bool {
    n == 0.0
}
