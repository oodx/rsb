//! Integer-Specific Operations
//!
//! Specialized operations for integer arithmetic and utilities
//! Separate from floating-point operations for precision control

// Integer-specific utilities
/// Greatest common divisor using Euclidean algorithm
pub fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Least common multiple
pub fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a.abs() / gcd(a, b)) * b.abs()
    }
}

/// Prime number test
pub fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let limit = (n as f64).sqrt() as i64;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// Find all factors of integer
pub fn factors(n: i64) -> Vec<i64> {
    if n == 0 {
        return vec![];
    }

    let n = n.abs();
    let mut factors = Vec::new();
    let limit = (n as f64).sqrt() as i64;

    for i in 1..=limit {
        if n % i == 0 {
            factors.push(i);
            if i != n / i {
                factors.push(n / i);
            }
        }
    }

    factors.sort();
    factors
}

/// Generate nth Fibonacci number
pub fn fibonacci(n: usize) -> i64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut a = 0i64;
    let mut b = 1i64;

    for _ in 2..=n {
        let next = a.saturating_add(b);
        a = b;
        b = next;
    }
    b
}

/// Calculate factorial with overflow protection
pub fn factorial(n: usize) -> Result<i64, String> {
    if n > 20 {
        return Err(format!("Factorial of {} would overflow i64", n));
    }

    let mut result = 1i64;
    for i in 1..=n {
        result = result.saturating_mul(i as i64);
    }

    Ok(result)
}

/// Sum integers in range (inclusive)
pub fn sum_range(start: i64, end: i64) -> i64 {
    if start > end {
        return 0;
    }

    let count = end - start + 1;
    let sum = (start + end).saturating_mul(count) / 2;
    sum
}

// Integer arithmetic with overflow detection
/// Integer addition with overflow detection
pub fn int_add(a: i64, b: i64) -> Result<i64, String> {
    a.checked_add(b).ok_or_else(|| format!("Integer overflow: {} + {}", a, b))
}

/// Integer subtraction with overflow detection
pub fn int_subtract(a: i64, b: i64) -> Result<i64, String> {
    a.checked_sub(b).ok_or_else(|| format!("Integer overflow: {} - {}", a, b))
}

/// Integer multiplication with overflow detection
pub fn int_multiply(a: i64, b: i64) -> Result<i64, String> {
    a.checked_mul(b).ok_or_else(|| format!("Integer overflow: {} * {}", a, b))
}

/// Integer division with zero-check
pub fn int_divide(a: i64, b: i64) -> Result<i64, String> {
    if b == 0 {
        return Err("Division by zero".to_string());
    }
    a.checked_div(b).ok_or_else(|| format!("Integer overflow: {} / {}", a, b))
}

/// Integer exponentiation with overflow protection
pub fn int_power(base: i64, exp: u32) -> Result<i64, String> {
    base.checked_pow(exp).ok_or_else(|| format!("Integer overflow: {} ^ {}", base, exp))
}

// String-first RSB interfaces
/// Parse string to integer with error reporting
pub fn int_parse(text: &str) -> Result<i64, String> {
    text.trim().parse::<i64>()
        .map_err(|_| format!("Could not parse '{}' as integer", text))
}

/// Generic integer calculation with string inputs/outputs
pub fn int_calc(operation: &str, a: &str, b: &str) -> String {
    let a_val = match int_parse(a) {
        Ok(v) => v,
        Err(e) => return format!("Error: {}", e),
    };

    let b_val = match int_parse(b) {
        Ok(v) => v,
        Err(e) => return format!("Error: {}", e),
    };

    let result = match operation {
        "add" | "+" => int_add(a_val, b_val),
        "subtract" | "sub" | "-" => int_subtract(a_val, b_val),
        "multiply" | "mul" | "*" => int_multiply(a_val, b_val),
        "divide" | "div" | "/" => int_divide(a_val, b_val),
        "gcd" => Ok(gcd(a_val, b_val)),
        "lcm" => Ok(lcm(a_val, b_val)),
        _ => Err(format!("Unknown operation: {}", operation)),
    };

    match result {
        Ok(v) => v.to_string(),
        Err(e) => format!("Error: {}", e),
    }
}

