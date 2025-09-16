//! Basic math operations sanity tests

#[cfg(test)]
mod tests {
    use rsb::math::basic::*;

    #[test]
    fn test_basic_arithmetic() {
        assert_eq!(add(5.0, 3.0), 8.0);
        assert_eq!(subtract(10.0, 3.0), 7.0);
        assert_eq!(multiply(4.0, 5.0), 20.0);
        assert_eq!(power(2.0, 3.0), 8.0);
        assert_eq!(abs(-5.0), 5.0);
        assert_eq!(abs(5.0), 5.0);
    }

    #[test]
    fn test_division() {
        assert_eq!(divide(10.0, 2.0).unwrap(), 5.0);
        assert!(divide(10.0, 0.0).is_err());
        assert!(divide(10.0, 0.0).unwrap_err().contains("Division by zero"));
    }

    #[test]
    fn test_sqrt() {
        assert_eq!(sqrt(16.0).unwrap(), 4.0);
        assert_eq!(sqrt(25.0).unwrap(), 5.0);
        assert!(sqrt(-1.0).is_err());
        assert!(sqrt(-1.0).unwrap_err().contains("Cannot take square root"));
    }

    #[test]
    fn test_min_max() {
        assert_eq!(min(5.0, 3.0), 3.0);
        assert_eq!(max(5.0, 3.0), 5.0);
        assert_eq!(min(-2.0, -5.0), -5.0);
        assert_eq!(max(-2.0, -5.0), -2.0);
    }

    #[test]
    fn test_rounding() {
        assert_eq!(round(3.14159, 2), 3.14);
        assert_eq!(roundup(3.14159, 2), 3.15);
        assert_eq!(rounddown(3.14159, 2), 3.14);

        assert_eq!(floor(3.9), 3.0);
        assert_eq!(ceil(3.1), 4.0);

        // Test integer rounding
        assert_eq!(round(3.14159, 0), 3.0);
        assert_eq!(roundup(3.1, 0), 4.0);
        assert_eq!(rounddown(3.9, 0), 3.0);
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("123.45").unwrap(), 123.45);
        assert_eq!(parse_number("  -42.7  ").unwrap(), -42.7);
        assert!(parse_number("not_a_number").is_err());
        assert!(parse_number("").is_err());
    }

    #[test]
    fn test_calc_string_interface() {
        assert_eq!(calc("add", "5", "3"), "8");
        assert_eq!(calc("subtract", "10", "3"), "7");
        assert_eq!(calc("multiply", "4", "5"), "20");
        assert_eq!(calc("divide", "10", "2"), "5");
        assert_eq!(calc("power", "2", "3"), "8");
        assert_eq!(calc("min", "5", "3"), "3");
        assert_eq!(calc("max", "5", "3"), "5");

        // Test aliases
        assert_eq!(calc("+", "5", "3"), "8");
        assert_eq!(calc("-", "10", "3"), "7");
        assert_eq!(calc("*", "4", "5"), "20");
        assert_eq!(calc("/", "10", "2"), "5");
        assert_eq!(calc("^", "2", "3"), "8");

        // Test errors
        assert!(calc("divide", "10", "0").contains("Error"));
        assert!(calc("unknown", "5", "3").contains("Error"));
        assert!(calc("add", "not_a_number", "3").contains("Error"));
    }
}