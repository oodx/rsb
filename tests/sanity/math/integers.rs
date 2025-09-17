//! Integer operations sanity tests

#[cfg(test)]
mod tests {
    use rsb::math::integers::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(17, 13), 1); // coprime
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
        assert_eq!(gcd(0, 0), 0);

        // Test with negative numbers
        assert_eq!(gcd(-48, 18), 6);
        assert_eq!(gcd(48, -18), 6);
        assert_eq!(gcd(-48, -18), 6);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(12, 8), 24);
        assert_eq!(lcm(17, 13), 221); // coprime
        assert_eq!(lcm(0, 5), 0);
        assert_eq!(lcm(5, 0), 0);

        // Test with negative numbers
        assert_eq!(lcm(-4, 6), 12);
        assert_eq!(lcm(4, -6), 12);
    }

    #[test]
    fn test_is_prime() {
        // Primes
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(is_prime(11));
        assert!(is_prime(13));
        assert!(is_prime(17));
        assert!(is_prime(97));

        // Non-primes
        assert!(!is_prime(1));
        assert!(!is_prime(4));
        assert!(!is_prime(6));
        assert!(!is_prime(8));
        assert!(!is_prime(9));
        assert!(!is_prime(10));
        assert!(!is_prime(15));
        assert!(!is_prime(100));

        // Edge cases
        assert!(!is_prime(0));
        assert!(!is_prime(-1));
        assert!(!is_prime(-7)); // negative numbers are not prime
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0).unwrap(), 1);
        assert_eq!(factorial(1).unwrap(), 1);
        assert_eq!(factorial(2).unwrap(), 2);
        assert_eq!(factorial(3).unwrap(), 6);
        assert_eq!(factorial(4).unwrap(), 24);
        assert_eq!(factorial(5).unwrap(), 120);
        assert_eq!(factorial(10).unwrap(), 3628800);

        // Overflow protection
        assert!(factorial(21).is_err());
        assert!(factorial(25).is_err());
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(6), 8);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(15), 610);

        // Test larger values (should use saturating_add)
        let large_fib = fibonacci(100);
        assert!(large_fib > 0); // Should not overflow to negative
    }

    #[test]
    fn test_factors() {
        assert_eq!(factors(12), vec![1, 2, 3, 4, 6, 12]);
        assert_eq!(factors(1), vec![1]);
        assert_eq!(factors(7), vec![1, 7]); // prime
        assert_eq!(factors(0), vec![] as Vec<i64>);
        assert_eq!(factors(16), vec![1, 2, 4, 8, 16]);

        // Test with negative (should take absolute value)
        assert_eq!(factors(-12), vec![1, 2, 3, 4, 6, 12]);
    }

    #[test]
    fn test_sum_range() {
        assert_eq!(sum_range(1, 5), 15); // 1+2+3+4+5
        assert_eq!(sum_range(1, 10), 55);
        assert_eq!(sum_range(5, 5), 5); // single number
        assert_eq!(sum_range(10, 1), 0); // invalid range
        assert_eq!(sum_range(-3, 3), 0); // -3+(-2)+(-1)+0+1+2+3

        // Test larger ranges
        assert_eq!(sum_range(1, 100), 5050);
    }

    #[test]
    fn test_int_arithmetic_with_overflow() {
        // Normal operations
        assert_eq!(int_add(5, 3).unwrap(), 8);
        assert_eq!(int_subtract(10, 3).unwrap(), 7);
        assert_eq!(int_multiply(4, 5).unwrap(), 20);
        assert_eq!(int_divide(10, 2).unwrap(), 5);
        assert_eq!(int_power(2, 3).unwrap(), 8);

        // Division by zero
        assert!(int_divide(10, 0).is_err());

        // Test overflow detection (use large values)
        assert!(int_add(i64::MAX, 1).is_err());
        assert!(int_subtract(i64::MIN, 1).is_err());
        assert!(int_multiply(i64::MAX, 2).is_err());
        assert!(int_power(2, 100).is_err()); // 2^100 > i64::MAX
    }

    #[test]
    fn test_int_parse() {
        assert_eq!(int_parse("123").unwrap(), 123);
        assert_eq!(int_parse("  -456  ").unwrap(), -456);
        assert_eq!(int_parse("0").unwrap(), 0);

        // Error cases
        assert!(int_parse("not_a_number").is_err());
        assert!(int_parse("123.45").is_err()); // float
        assert!(int_parse("").is_err());
    }

    #[test]
    fn test_int_calc() {
        assert_eq!(int_calc("add", "5", "3"), "8");
        assert_eq!(int_calc("subtract", "10", "3"), "7");
        assert_eq!(int_calc("multiply", "4", "5"), "20");
        assert_eq!(int_calc("divide", "10", "2"), "5");
        assert_eq!(int_calc("gcd", "48", "18"), "6");
        assert_eq!(int_calc("lcm", "4", "6"), "12");

        // Test aliases
        assert_eq!(int_calc("+", "5", "3"), "8");
        assert_eq!(int_calc("-", "10", "3"), "7");
        assert_eq!(int_calc("*", "4", "5"), "20");
        assert_eq!(int_calc("/", "10", "2"), "5");

        // Error cases
        assert!(int_calc("divide", "10", "0").contains("Error"));
        assert!(int_calc("unknown", "5", "3").contains("Error"));
        assert!(int_calc("add", "not_a_number", "3").contains("Error"));
    }

    #[test]
    fn test_mathematical_properties() {
        // GCD and LCM relationship: gcd(a,b) * lcm(a,b) = a * b
        let a = 12;
        let b = 8;
        assert_eq!(gcd(a, b) * lcm(a, b), a * b);

        // GCD is commutative
        assert_eq!(gcd(a, b), gcd(b, a));

        // LCM is commutative
        assert_eq!(lcm(a, b), lcm(b, a));

        // Factorial growth
        assert!(factorial(5).unwrap() > factorial(4).unwrap());

        // Fibonacci growth
        assert!(fibonacci(10) > fibonacci(9));
    }
}
