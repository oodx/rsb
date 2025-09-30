// --- Math Macros ---
// Module-owned macros for mathematical operations

pub use crate::math;

#[macro_export]
macro_rules! math {
    ($expr:expr) => {
        match $crate::math::evaluate_expression($expr) {
            Ok(result) => result,
            Err(e) => {
                $crate::utils::stderrx("error", &format!("Math expression failed: {}", e));
                0.0
            }
        }
    };
}

#[macro_export]
macro_rules! calc {
    ($op:expr, $a:expr, $b:expr) => {
        $crate::math::calc($op, $a, $b)
    };
}

#[macro_export]
macro_rules! int_calc {
    ($op:expr, $a:expr, $b:expr) => {
        $crate::math::int_calc($op, $a, $b)
    };
}

#[macro_export]
macro_rules! gcd {
    ($a:expr, $b:expr) => {
        $crate::math::gcd($a, $b)
    };
}

#[macro_export]
macro_rules! lcm {
    ($a:expr, $b:expr) => {
        $crate::math::lcm($a, $b)
    };
}

#[macro_export]
macro_rules! is_prime {
    ($n:expr) => {
        $crate::math::is_prime($n)
    };
}

#[macro_export]
macro_rules! factorial {
    ($n:expr) => {
        $crate::math::factorial($n)
    };
}

#[macro_export]
macro_rules! fibonacci {
    ($n:expr) => {
        $crate::math::fibonacci($n)
    };
}

#[macro_export]
macro_rules! add {
    ($a:expr, $b:expr) => {
        $crate::math::add($a, $b)
    };
}

#[macro_export]
macro_rules! subtract {
    ($a:expr, $b:expr) => {
        $crate::math::subtract($a, $b)
    };
}

#[macro_export]
macro_rules! multiply {
    ($a:expr, $b:expr) => {
        $crate::math::multiply($a, $b)
    };
}

#[macro_export]
macro_rules! divide {
    ($a:expr, $b:expr) => {
        match $crate::math::divide($a, $b) {
            Ok(result) => result,
            Err(e) => {
                $crate::utils::stderrx("error", &format!("Division error: {}", e));
                0.0
            }
        }
    };
}

#[macro_export]
macro_rules! power {
    ($base:expr, $exp:expr) => {
        $crate::math::power($base, $exp)
    };
}

#[macro_export]
macro_rules! sqrt {
    ($n:expr) => {
        match $crate::math::sqrt($n) {
            Ok(result) => result,
            Err(e) => {
                $crate::utils::stderrx("error", &format!("Square root error: {}", e));
                0.0
            }
        }
    };
}

#[macro_export]
macro_rules! min {
    ($a:expr, $b:expr) => {
        $crate::math::min($a, $b)
    };
}

#[macro_export]
macro_rules! max {
    ($a:expr, $b:expr) => {
        $crate::math::max($a, $b)
    };
}

#[macro_export]
macro_rules! round {
    ($n:expr) => {
        $crate::math::round($n, 0)
    };
    ($n:expr, $places:expr) => {
        $crate::math::round($n, $places)
    };
}

#[macro_export]
macro_rules! roundup {
    ($n:expr) => {
        $crate::math::roundup($n, 0)
    };
    ($n:expr, $places:expr) => {
        $crate::math::roundup($n, $places)
    };
}

#[macro_export]
macro_rules! rounddown {
    ($n:expr) => {
        $crate::math::rounddown($n, 0)
    };
    ($n:expr, $places:expr) => {
        $crate::math::rounddown($n, $places)
    };
}

#[macro_export]
macro_rules! floor {
    ($n:expr) => {
        $crate::math::floor($n)
    };
}

#[macro_export]
macro_rules! ceil {
    ($n:expr) => {
        $crate::math::ceil($n)
    };
}

#[macro_export]
macro_rules! to_hex {
    ($n:expr) => {
        $crate::math::to_hex($n)
    };
}

#[macro_export]
macro_rules! to_hex_upper {
    ($n:expr) => {
        $crate::math::to_hex_upper($n)
    };
}

#[macro_export]
macro_rules! to_binary {
    ($n:expr) => {
        $crate::math::to_binary($n)
    };
}

#[macro_export]
macro_rules! to_octal {
    ($n:expr) => {
        $crate::math::to_octal($n)
    };
}

#[macro_export]
macro_rules! from_hex {
    ($hex_str:expr) => {
        match $crate::math::from_hex($hex_str) {
            Ok(result) => result,
            Err(e) => {
                $crate::utils::stderrx("error", &format!("Hex parsing error: {}", e));
                0
            }
        }
    };
}

#[macro_export]
macro_rules! from_binary {
    ($bin_str:expr) => {
        match $crate::math::from_binary($bin_str) {
            Ok(result) => result,
            Err(e) => {
                $crate::utils::stderrx("error", &format!("Binary parsing error: {}", e));
                0
            }
        }
    };
}

#[macro_export]
macro_rules! from_octal {
    ($oct_str:expr) => {
        match $crate::math::from_octal($oct_str) {
            Ok(result) => result,
            Err(e) => {
                $crate::utils::stderrx("error", &format!("Octal parsing error: {}", e));
                0
            }
        }
    };
}

#[macro_export]
macro_rules! to_base {
    ($n:expr, $base:expr) => {
        match $crate::math::to_base($n, $base) {
            Ok(result) => result,
            Err(e) => {
                $crate::utils::stderrx("error", &format!("Base conversion error: {}", e));
                String::new()
            }
        }
    };
}

#[macro_export]
macro_rules! from_base {
    ($num_str:expr, $base:expr) => {
        match $crate::math::from_base($num_str, $base) {
            Ok(result) => result,
            Err(e) => {
                $crate::utils::stderrx("error", &format!("Base parsing error: {}", e));
                0
            }
        }
    };
}

#[macro_export]
macro_rules! base_convert {
    ($num_str:expr, $from_base:expr, $to_base:expr) => {
        match $crate::math::base_convert($num_str, $from_base, $to_base) {
            Ok(result) => result,
            Err(e) => {
                $crate::utils::stderrx("error", &format!("Base conversion error: {}", e));
                String::new()
            }
        }
    };
}

// === Percentage Package Macros ===

#[macro_export]
macro_rules! percent_of {
    ($total:expr, $percent:expr) => {
        $crate::math::percent_of($total, $percent)
    };
}

#[macro_export]
macro_rules! percent_change {
    ($original:expr, $new:expr) => {
        $crate::math::percent_change($original, $new)
    };
}

#[macro_export]
macro_rules! ratio {
    ($numerator:expr, $denominator:expr) => {
        match $crate::math::ratio($numerator, $denominator) {
            Ok(result) => result,
            Err(e) => {
                $crate::utils::stderrx("error", &format!("Ratio error: {}", e));
                0.0
            }
        }
    };
}

// === Predicate Package Macros ===

#[macro_export]
macro_rules! even {
    ($n:expr) => {
        $crate::math::is_even($n)
    };
}

#[macro_export]
macro_rules! odd {
    ($n:expr) => {
        $crate::math::is_odd($n)
    };
}

#[macro_export]
macro_rules! modulo {
    ($a:expr, $b:expr) => {
        match $crate::math::modulo($a, $b) {
            Ok(result) => result,
            Err(e) => {
                $crate::utils::stderrx("error", &format!("Modulo error: {}", e));
                0
            }
        }
    };
}

#[macro_export]
macro_rules! sign {
    ($n:expr) => {
        $crate::math::sign($n)
    };
}

#[macro_export]
macro_rules! same_sign {
    ($a:expr, $b:expr) => {
        $crate::math::same_sign($a, $b)
    };
}

// === Aggregator Package Macros ===

#[macro_export]
macro_rules! min_list {
    ($numbers:expr) => {
        $crate::math::min_list($numbers).unwrap_or(0.0)
    };
}

#[macro_export]
macro_rules! max_list {
    ($numbers:expr) => {
        $crate::math::max_list($numbers).unwrap_or(0.0)
    };
}

#[macro_export]
macro_rules! avg {
    ($numbers:expr) => {
        $crate::math::avg($numbers).unwrap_or(0.0)
    };
}

#[macro_export]
macro_rules! mean {
    ($numbers:expr) => {
        $crate::math::mean($numbers).unwrap_or(0.0)
    };
}

#[macro_export]
macro_rules! median {
    ($numbers:expr) => {
        $crate::math::median($numbers).unwrap_or(0.0)
    };
}

#[macro_export]
macro_rules! sum_list {
    ($numbers:expr) => {
        $crate::math::sum_list($numbers)
    };
}

// === Random Package Macros (now in gx::rand) ===

#[macro_export]
macro_rules! random_range {
    ($min:expr, $max:expr) => {
        $crate::gx::rand::random_range($min, $max)
    };
}

#[macro_export]
macro_rules! random_int_range {
    ($min:expr, $max:expr) => {
        $crate::gx::rand::random_int_range($min, $max)
    };
}

#[macro_export]
macro_rules! random_list {
    ($count:expr, $type:expr) => {
        match $crate::gx::rand::random_list_string($type, $count, None) {
            Ok(result) => result,
            Err(e) => {
                $crate::utils::stderrx("error", &format!("Random list error: {}", e));
                String::new()
            }
        }
    };
    ($count:expr, $type:expr, $range:expr) => {
        match $crate::gx::rand::random_list_string($type, $count, Some($range)) {
            Ok(result) => result,
            Err(e) => {
                $crate::utils::stderrx("error", &format!("Random list error: {}", e));
                String::new()
            }
        }
    };
}
