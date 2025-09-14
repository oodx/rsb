//! Percentage and Ratio Package
//!
//! Functions for calculating percentages, ratios, and percentage changes

pub fn percent_of(total: f64, percent: f64) -> f64 {
    total * (percent / 100.0)
}

pub fn percent_change(original: f64, new: f64) -> f64 {
    if original == 0.0 {
        if new == 0.0 { 0.0 } else { f64::INFINITY }
    } else {
        ((new - original) / original) * 100.0
    }
}

pub fn ratio(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Division by zero in ratio".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

pub fn percentage_to_decimal(percent: f64) -> f64 {
    percent / 100.0
}

pub fn decimal_to_percentage(decimal: f64) -> f64 {
    decimal * 100.0
}