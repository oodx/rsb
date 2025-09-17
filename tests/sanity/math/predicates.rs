//! Predicates operations sanity tests

#[cfg(test)]
mod tests {
    use rsb::math::predicates::*;

    #[test]
    fn test_even_odd() {
        // Even numbers
        assert!(is_even(0));
        assert!(is_even(2));
        assert!(is_even(100));
        assert!(is_even(-4));

        // Odd numbers
        assert!(is_odd(1));
        assert!(is_odd(3));
        assert!(is_odd(99));
        assert!(is_odd(-3));

        // Complementary tests
        assert!(!is_even(1));
        assert!(!is_even(3));
        assert!(!is_odd(0));
        assert!(!is_odd(2));
    }

    #[test]
    fn test_modulo() {
        assert_eq!(modulo(10, 3).unwrap(), 1);
        assert_eq!(modulo(15, 4).unwrap(), 3);
        assert_eq!(modulo(20, 5).unwrap(), 0);
        assert_eq!(modulo(-7, 3).unwrap(), -1);

        // Division by zero
        assert!(modulo(10, 0).is_err());
        assert!(modulo(10, 0).unwrap_err().contains("Division by zero"));
    }

    #[test]
    fn test_sign() {
        assert_eq!(sign(5.0), 1);
        assert_eq!(sign(-5.0), -1);
        assert_eq!(sign(0.0), 0);
        assert_eq!(sign(0.1), 1);
        assert_eq!(sign(-0.1), -1);
    }

    #[test]
    fn test_same_sign() {
        assert!(same_sign(5.0, 3.0)); // both positive
        assert!(same_sign(-5.0, -3.0)); // both negative
        assert!(same_sign(0.0, 0.0)); // both zero

        assert!(!same_sign(5.0, -3.0)); // different signs
        assert!(!same_sign(-5.0, 3.0)); // different signs
        assert!(!same_sign(0.0, 5.0)); // zero vs positive
        assert!(!same_sign(0.0, -5.0)); // zero vs negative
    }

    #[test]
    fn test_sign_predicates() {
        assert!(is_positive(5.0));
        assert!(is_positive(0.1));
        assert!(!is_positive(0.0));
        assert!(!is_positive(-5.0));

        assert!(is_negative(-5.0));
        assert!(is_negative(-0.1));
        assert!(!is_negative(0.0));
        assert!(!is_negative(5.0));

        assert!(is_zero(0.0));
        assert!(!is_zero(0.1));
        assert!(!is_zero(-0.1));
    }

    #[test]
    fn test_edge_cases() {
        // Large numbers
        assert!(is_even(1000000000));
        assert!(is_odd(1000000001));

        // Floating point edge cases
        assert_eq!(sign(f64::INFINITY), 1);
        assert_eq!(sign(f64::NEG_INFINITY), -1);
        assert!(is_positive(f64::INFINITY));
        assert!(is_negative(f64::NEG_INFINITY));

        // NaN handling (NaN comparisons are always false)
        assert_eq!(sign(f64::NAN), 0); // Our sign function treats NaN as 0
    }
}
