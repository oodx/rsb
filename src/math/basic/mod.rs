//! Basic Mathematical Operations Package
//!
//! Core arithmetic operations: add, subtract, multiply, divide, power, sqrt, etc.

use crate::global::get_var;

pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

pub fn power(base: f64, exp: f64) -> f64 {
    base.powf(exp)
}

pub fn sqrt(n: f64) -> Result<f64, String> {
    if n < 0.0 {
        Err("Cannot take square root of negative number".to_string())
    } else {
        Ok(n.sqrt())
    }
}

pub fn abs(n: f64) -> f64 {
    n.abs()
}

pub fn min(a: f64, b: f64) -> f64 {
    a.min(b)
}

pub fn max(a: f64, b: f64) -> f64 {
    a.max(b)
}

pub fn round(n: f64, places: usize) -> f64 {
    let multiplier = 10.0_f64.powi(places as i32);
    (n * multiplier).round() / multiplier
}

pub fn roundup(n: f64, places: usize) -> f64 {
    let multiplier = 10.0_f64.powi(places as i32);
    (n * multiplier).ceil() / multiplier
}

pub fn rounddown(n: f64, places: usize) -> f64 {
    let multiplier = 10.0_f64.powi(places as i32);
    (n * multiplier).floor() / multiplier
}

pub fn floor(n: f64) -> f64 {
    n.floor()
}

pub fn ceil(n: f64) -> f64 {
    n.ceil()
}

pub fn parse_number(text: &str) -> Result<f64, String> {
    text.trim().parse::<f64>()
        .map_err(|_| format!("Could not parse '{}' as number", text))
}

pub fn calc(operation: &str, a_str: &str, b_str: &str) -> String {
    let a = match parse_number(a_str) {
        Ok(v) => v,
        Err(e) => return format!("Error: {}", e),
    };

    let b = match parse_number(b_str) {
        Ok(v) => v,
        Err(e) => return format!("Error: {}", e),
    };

    let result = match operation {
        "add" | "+" => Ok(add(a, b)),
        "subtract" | "sub" | "-" => Ok(subtract(a, b)),
        "multiply" | "mul" | "*" => Ok(multiply(a, b)),
        "divide" | "div" | "/" => divide(a, b),
        "power" | "pow" | "**" | "^" => Ok(power(a, b)),
        "min" => Ok(min(a, b)),
        "max" => Ok(max(a, b)),
        _ => Err(format!("Unknown operation: {}", operation)),
    };

    match result {
        Ok(v) => v.to_string(),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn eval_var(var_name: &str) -> Result<f64, String> {
    let val_str = get_var(var_name);
    parse_number(&val_str)
        .map_err(|_| format!("Variable '{}' is not a valid number: {}", var_name, val_str))
}

