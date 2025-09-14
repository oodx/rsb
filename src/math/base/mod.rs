//! Base Conversion Package
//!
//! Number base conversion functions: hex, binary, octal, arbitrary base

pub fn to_hex(n: i64) -> String {
    format!("{:x}", n)
}

pub fn to_hex_upper(n: i64) -> String {
    format!("{:X}", n)
}

pub fn to_binary(n: i64) -> String {
    format!("{:b}", n)
}

pub fn to_octal(n: i64) -> String {
    format!("{:o}", n)
}

pub fn from_hex(hex_str: &str) -> Result<i64, String> {
    let cleaned = hex_str.trim_start_matches("0x").trim_start_matches("0X");
    i64::from_str_radix(cleaned, 16)
        .map_err(|_| format!("Could not parse '{}' as hexadecimal", hex_str))
}

pub fn from_binary(bin_str: &str) -> Result<i64, String> {
    let cleaned = bin_str.trim_start_matches("0b").trim_start_matches("0B");
    i64::from_str_radix(cleaned, 2)
        .map_err(|_| format!("Could not parse '{}' as binary", bin_str))
}

pub fn from_octal(oct_str: &str) -> Result<i64, String> {
    let cleaned = oct_str.trim_start_matches("0o").trim_start_matches("0O");
    i64::from_str_radix(cleaned, 8)
        .map_err(|_| format!("Could not parse '{}' as octal", oct_str))
}

pub fn to_base(n: i64, base: u32) -> Result<String, String> {
    if base < 2 || base > 36 {
        return Err("Base must be between 2 and 36".to_string());
    }

    if n == 0 {
        return Ok("0".to_string());
    }

    let mut result = String::new();
    let mut num = n.abs();
    let digits = "0123456789abcdefghijklmnopqrstuvwxyz";

    while num > 0 {
        let digit_index = (num % base as i64) as usize;
        result.insert(0, digits.chars().nth(digit_index).unwrap());
        num /= base as i64;
    }

    if n < 0 {
        result.insert(0, '-');
    }

    Ok(result)
}

pub fn from_base(num_str: &str, base: u32) -> Result<i64, String> {
    if base < 2 || base > 36 {
        return Err("Base must be between 2 and 36".to_string());
    }

    i64::from_str_radix(num_str, base)
        .map_err(|_| format!("Could not parse '{}' as base-{} number", num_str, base))
}

pub fn base_convert(num_str: &str, from_base: u32, to_base: u32) -> Result<String, String> {
    let decimal_value = from_base(num_str, from_base)?;
    to_base(decimal_value, to_base)
}