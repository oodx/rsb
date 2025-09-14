//! Random Package
//!
//! Random number generation and list creation with type support

use rand::Rng;

// Basic random generation functions - self-contained in math/random

/// Generates a random f64 value between 0.0 and 1.0.
fn random_f64() -> f64 {
    rand::rng().random()
}

/// Generates a random i64 value.
fn random_i64() -> i64 {
    rand::rng().random()
}

/// Generates a random boolean value.
fn random_bool() -> bool {
    rand::rng().random()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    min + random_f64() * (max - min)
}

pub fn random_int_range(min: i64, max: i64) -> i64 {
    min + (random_i64().abs() % (max - min + 1))
}

pub fn random_list_float(count: usize, min: f64, max: f64) -> Vec<f64> {
    (0..count).map(|_| random_range(min, max)).collect()
}

pub fn random_list_int(count: usize, min: i64, max: i64) -> Vec<i64> {
    (0..count).map(|_| random_int_range(min, max)).collect()
}

pub fn random_list_bool(count: usize) -> Vec<bool> {
    (0..count).map(|_| random_bool()).collect()
}

pub fn random_list_string(list_type: &str, count: usize, range_str: Option<&str>) -> Result<String, String> {
    match list_type.to_lowercase().as_str() {
        "bool" | "boolean" => {
            let bools = random_list_bool(count);
            let as_ints: Vec<String> = bools.iter().map(|&b| if b { "1".to_string() } else { "0".to_string() }).collect();
            Ok(as_ints.join(","))
        },
        "int" | "integer" => {
            let (min, max) = if let Some(range) = range_str {
                parse_range_int(range)?
            } else {
                (1, 100)  // default range
            };
            let ints = random_list_int(count, min, max);
            Ok(ints.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","))
        },
        "float" | "f64" => {
            let (min, max) = if let Some(range) = range_str {
                parse_range_float(range)?
            } else {
                (0.0, 1.0)  // default range
            };
            let floats = random_list_float(count, min, max);
            Ok(floats.iter().map(|f| format!("{:.2}", f)).collect::<Vec<_>>().join(","))
        },
        _ => Err(format!("Unknown type: {}. Use 'bool', 'int', or 'float'", list_type))
    }
}

fn parse_range_int(range_str: &str) -> Result<(i64, i64), String> {
    if let Some(colon_pos) = range_str.find(':') {
        let min_str = &range_str[..colon_pos];
        let max_str = &range_str[colon_pos + 1..];
        let min = min_str.parse::<i64>().map_err(|_| format!("Invalid min value: {}", min_str))?;
        let max = max_str.parse::<i64>().map_err(|_| format!("Invalid max value: {}", max_str))?;
        if min > max {
            return Err(format!("Min ({}) cannot be greater than max ({})", min, max));
        }
        Ok((min, max))
    } else {
        Err("Range format should be 'min:max'".to_string())
    }
}

fn parse_range_float(range_str: &str) -> Result<(f64, f64), String> {
    if let Some(colon_pos) = range_str.find(':') {
        let min_str = &range_str[..colon_pos];
        let max_str = &range_str[colon_pos + 1..];
        let min = min_str.parse::<f64>().map_err(|_| format!("Invalid min value: {}", min_str))?;
        let max = max_str.parse::<f64>().map_err(|_| format!("Invalid max value: {}", max_str))?;
        if min > max {
            return Err(format!("Min ({}) cannot be greater than max ({})", min, max));
        }
        Ok((min, max))
    } else {
        Err("Range format should be 'min:max'".to_string())
    }
}