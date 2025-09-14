//! Aggregators Package
//!
//! Functions for aggregating lists of numbers: min, max, avg, mean, sum

pub fn min_list(numbers: &[f64]) -> Option<f64> {
    if numbers.is_empty() {
        None
    } else {
        Some(numbers.iter().fold(f64::INFINITY, |a, &b| a.min(b)))
    }
}

pub fn max_list(numbers: &[f64]) -> Option<f64> {
    if numbers.is_empty() {
        None
    } else {
        Some(numbers.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)))
    }
}

pub fn sum_list(numbers: &[f64]) -> f64 {
    numbers.iter().sum()
}

pub fn avg(numbers: &[f64]) -> Option<f64> {
    if numbers.is_empty() {
        None
    } else {
        Some(sum_list(numbers) / numbers.len() as f64)
    }
}

pub fn mean(numbers: &[f64]) -> Option<f64> {
    avg(numbers)  // mean is same as average
}

pub fn median(numbers: &[f64]) -> Option<f64> {
    if numbers.is_empty() {
        return None;
    }

    let mut sorted = numbers.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let len = sorted.len();
    if len % 2 == 0 {
        Some((sorted[len/2 - 1] + sorted[len/2]) / 2.0)
    } else {
        Some(sorted[len/2])
    }
}