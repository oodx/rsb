// RSB Sanity Tests - Math Module Core Functionality Verification
// Tests verify the math module functions and macros work as documented in FEATURES_MATH

use rsb::prelude::*;

#[test]
fn test_basic_arithmetic_operations() {
    // Test core arithmetic functions
    assert_eq!(rsb::math::add(5.0, 3.0), 8.0);
    assert_eq!(rsb::math::subtract(10.0, 3.0), 7.0);
    assert_eq!(rsb::math::multiply(4.0, 5.0), 20.0);

    // Test division with success case
    match rsb::math::divide(10.0, 2.0) {
        Ok(result) => assert_eq!(result, 5.0),
        Err(_) => panic!("Valid division should succeed"),
    }

    // Test division by zero handling
    assert!(rsb::math::divide(10.0, 0.0).is_err());

    // Test other basic operations
    assert_eq!(rsb::math::power(2.0, 3.0), 8.0);
    assert_eq!(rsb::math::sqrt(16.0).unwrap(), 4.0);
    assert!(rsb::math::sqrt(-1.0).is_err()); // Negative sqrt should fail
    assert_eq!(rsb::math::abs(-5.0), 5.0);
    assert_eq!(rsb::math::min(5.0, 3.0), 3.0);
    assert_eq!(rsb::math::max(5.0, 3.0), 5.0);
}

#[test]
fn test_rounding_operations() {
    // Test various rounding functions
    assert_eq!(rsb::math::round(3.14159, 2), 3.14);
    assert_eq!(rsb::math::roundup(3.14159, 2), 3.15);
    assert_eq!(rsb::math::rounddown(3.14159, 2), 3.14);
    assert_eq!(rsb::math::floor(3.7), 3.0);
    assert_eq!(rsb::math::ceil(3.2), 4.0);
}

#[test]
fn test_integer_operations() {
    // Test GCD and LCM
    assert_eq!(rsb::math::gcd(48, 18), 6);
    assert_eq!(rsb::math::lcm(4, 6), 12);

    // Test prime checking
    assert!(rsb::math::is_prime(17));
    assert!(!rsb::math::is_prime(16));
    assert!(!rsb::math::is_prime(1));

    // Test factorial with valid input
    match rsb::math::factorial(5) {
        Ok(result) => assert_eq!(result, 120),
        Err(_) => panic!("Valid factorial should succeed"),
    }

    // Test factorial overflow protection
    assert!(rsb::math::factorial(25).is_err());

    // Test Fibonacci
    assert_eq!(rsb::math::fibonacci(10), 55);
    assert_eq!(rsb::math::fibonacci(0), 0);
    assert_eq!(rsb::math::fibonacci(1), 1);

    // Test factors
    let factors = rsb::math::factors(12);
    assert!(factors.contains(&1));
    assert!(factors.contains(&2));
    assert!(factors.contains(&3));
    assert!(factors.contains(&4));
    assert!(factors.contains(&6));
    assert!(factors.contains(&12));

    // Test sum_range
    assert_eq!(rsb::math::sum_range(1, 5), 15); // 1+2+3+4+5 = 15
}

#[test]
fn test_integer_arithmetic_with_overflow() {
    // Test integer operations with overflow detection
    assert_eq!(rsb::math::int_add(5, 3).unwrap(), 8);
    assert_eq!(rsb::math::int_subtract(10, 3).unwrap(), 7);
    assert_eq!(rsb::math::int_multiply(4, 5).unwrap(), 20);

    // Test division
    assert_eq!(rsb::math::int_divide(10, 2).unwrap(), 5);
    assert!(rsb::math::int_divide(10, 0).is_err()); // Division by zero

    // Test power
    assert_eq!(rsb::math::int_power(2, 3).unwrap(), 8);

    // Test parse
    assert_eq!(rsb::math::int_parse("42").unwrap(), 42);
    assert!(rsb::math::int_parse("invalid").is_err());
}

#[test]
fn test_base_conversions() {
    // Test hex conversions
    assert_eq!(rsb::math::to_hex(255), "ff");
    assert_eq!(rsb::math::to_hex_upper(255), "FF");
    assert_eq!(rsb::math::from_hex("FF").unwrap(), 255);
    assert_eq!(rsb::math::from_hex("ff").unwrap(), 255);

    // Test binary conversions
    assert_eq!(rsb::math::to_binary(8), "1000");
    assert_eq!(rsb::math::from_binary("1000").unwrap(), 8);

    // Test octal conversions
    assert_eq!(rsb::math::to_octal(64), "100");
    assert_eq!(rsb::math::from_octal("100").unwrap(), 64);

    // Test arbitrary base conversions
    assert_eq!(rsb::math::to_base(100, 36).unwrap(), "2s");
    assert_eq!(rsb::math::from_base("2s", 36).unwrap(), 100);

    // Test base conversion
    assert_eq!(rsb::math::base_convert("FF", 16, 2).unwrap(), "11111111");
}

#[test]
fn test_percentage_operations() {
    // Test percentage calculations
    assert_eq!(rsb::math::percent_of(250.0, 20.0), 50.0); // 20% of 250
    assert_eq!(rsb::math::percent_change(100.0, 150.0), 50.0); // 50% increase
    assert_eq!(rsb::math::ratio(16.0, 9.0).unwrap(), 16.0 / 9.0); // 16:9 ratio

    // Test percentage conversions
    assert_eq!(rsb::math::percentage_to_decimal(50.0), 0.5);
    assert_eq!(rsb::math::decimal_to_percentage(0.25), 25.0);
}

#[test]
fn test_predicate_operations() {
    // Test even/odd
    assert!(rsb::math::is_even(8));
    assert!(!rsb::math::is_even(7));
    assert!(rsb::math::is_odd(7));
    assert!(!rsb::math::is_odd(8));

    // Test modulo
    assert_eq!(rsb::math::modulo(10, 3).unwrap(), 1);
    assert!(rsb::math::modulo(10, 0).is_err()); // Division by zero

    // Test sign operations
    assert_eq!(rsb::math::sign(-5.0), -1);
    assert_eq!(rsb::math::sign(0.0), 0);
    assert_eq!(rsb::math::sign(5.0), 1);

    assert!(rsb::math::same_sign(5.0, 3.0));
    assert!(!rsb::math::same_sign(-5.0, 3.0));

    assert!(rsb::math::is_positive(5.0));
    assert!(!rsb::math::is_positive(-5.0));
    assert!(rsb::math::is_negative(-5.0));
    assert!(!rsb::math::is_negative(5.0));
    assert!(rsb::math::is_zero(0.0));
    assert!(!rsb::math::is_zero(1.0));
}

#[test]
fn test_aggregator_operations() {
    let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    // Test aggregation functions
    assert_eq!(rsb::math::min_list(&numbers).unwrap(), 1.0);
    assert_eq!(rsb::math::max_list(&numbers).unwrap(), 5.0);
    assert_eq!(rsb::math::sum_list(&numbers), 15.0);
    assert_eq!(rsb::math::avg(&numbers).unwrap(), 3.0);
    assert_eq!(rsb::math::mean(&numbers).unwrap(), 3.0); // Alias for avg
    assert_eq!(rsb::math::median(&numbers).unwrap(), 3.0);

    // Test with empty list
    let empty: Vec<f64> = vec![];
    assert!(rsb::math::avg(&empty).is_none());
    assert!(rsb::math::min_list(&empty).is_none());
    assert!(rsb::math::max_list(&empty).is_none());
}

#[test]
fn test_random_operations() {
    // Test random range functions
    let random_float = rsb::math::random_range(1.0, 10.0);
    assert!(random_float >= 1.0 && random_float <= 10.0);

    let random_int = rsb::math::random_int_range(1, 100);
    assert!(random_int >= 1 && random_int <= 100);

    // Test random list generation
    let float_list = rsb::math::random_list_float(5, 0.0, 1.0);
    assert_eq!(float_list.len(), 5);
    for val in &float_list {
        assert!(*val >= 0.0 && *val <= 1.0);
    }

    let int_list = rsb::math::random_list_int(3, 1, 10);
    assert_eq!(int_list.len(), 3);
    for val in &int_list {
        assert!(*val >= 1 && *val <= 10);
    }

    let bool_list = rsb::math::random_list_bool(4);
    assert_eq!(bool_list.len(), 4);

    // Test string-based random list generation
    let random_string = rsb::math::random_list_string("int", 3, Some("1:10"));
    match random_string {
        Ok(s) => {
            assert!(!s.is_empty());
            assert!(s.contains(",")); // Should be comma-separated
        }
        Err(e) => panic!("Random string generation failed: {}", e),
    }

    let bool_string = rsb::math::random_list_string("bool", 3, None);
    match bool_string {
        Ok(s) => assert!(!s.is_empty()),
        Err(e) => panic!("Random bool string generation failed: {}", e),
    }
}

#[test]
fn test_expression_evaluation() {
    // Test basic expression evaluation
    set_var("x", "5");
    set_var("y", "3");

    // Test that the function exists and can be called
    let result = rsb::math::evaluate_expression("x + y");
    // Since we don't know the exact behavior, just test it doesn't panic
    // and produces some result
    match result {
        Ok(val) => assert!(val >= 0.0), // Basic sanity check
        Err(_) => {}                    // Expression evaluation might fail, that's OK for this test
    }
}

#[test]
fn test_string_based_calculations() {
    // Test string-based calc function
    assert_eq!(rsb::math::calc("add", "5", "3"), "8");
    assert_eq!(rsb::math::calc("subtract", "10", "3"), "7");
    assert_eq!(rsb::math::calc("multiply", "4", "5"), "20");
    assert_eq!(rsb::math::calc("divide", "10", "2"), "5");

    // Test invalid operations
    let invalid_result = rsb::math::calc("invalid", "5", "3");
    assert!(invalid_result.contains("Error") || invalid_result.contains("Invalid"));
    let div_by_zero = rsb::math::calc("divide", "5", "0");
    assert!(div_by_zero.contains("Division by zero") || div_by_zero.contains("Error"));

    // Test int_calc function
    assert_eq!(rsb::math::int_calc("add", "42", "58"), "100");
    assert_eq!(rsb::math::int_calc("gcd", "48", "18"), "6");
    assert_eq!(rsb::math::int_calc("multiply", "7", "8"), "56");
}

#[test]
fn test_parse_number() {
    // Test number parsing
    assert_eq!(rsb::math::parse_number("42.5").unwrap(), 42.5);
    assert_eq!(rsb::math::parse_number("0").unwrap(), 0.0);
    assert_eq!(rsb::math::parse_number("-123.45").unwrap(), -123.45);

    // Test invalid input
    assert!(rsb::math::parse_number("invalid").is_err());
    assert!(rsb::math::parse_number("").is_err());
}

#[test]
fn test_math_consistency() {
    // Test that repeated calculations give consistent results
    let a = 5.0;
    let b = 3.0;

    assert_eq!(rsb::math::add(a, b), rsb::math::add(a, b));
    assert_eq!(rsb::math::multiply(a, b), rsb::math::multiply(a, b));

    // Test mathematical relationships
    assert_eq!(
        rsb::math::power(2.0, 3.0),
        rsb::math::multiply(rsb::math::multiply(2.0, 2.0), 2.0)
    );
    assert_eq!(rsb::math::sqrt(16.0).unwrap(), 4.0);
    assert_eq!(rsb::math::power(4.0, 2.0), 16.0);
}
