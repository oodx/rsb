//! Expression evaluation sanity tests

#[cfg(test)]
mod tests {
    use rsb::math::expressions::*;
    use rsb::global::{set_var, get_var};

    #[test]
    fn test_basic_expressions() {
        assert_eq!(evaluate_expression("x = 5 + 3").unwrap(), 8.0);
        assert_eq!(get_var("x"), "8");

        assert_eq!(evaluate_expression("y = 10 - 4").unwrap(), 6.0);
        assert_eq!(get_var("y"), "6");

        assert_eq!(evaluate_expression("z = 3 * 4").unwrap(), 12.0);
        assert_eq!(get_var("z"), "12");

        assert_eq!(evaluate_expression("w = 15 / 3").unwrap(), 5.0);
        assert_eq!(get_var("w"), "5");
    }

    #[test]
    fn test_operator_precedence() {
        assert_eq!(evaluate_expression("result = 2 + 3 * 4").unwrap(), 14.0); // 2 + 12
        assert_eq!(get_var("result"), "14");

        assert_eq!(evaluate_expression("result2 = (2 + 3) * 4").unwrap(), 20.0); // 5 * 4
        assert_eq!(get_var("result2"), "20");

        assert_eq!(evaluate_expression("result3 = 20 / 4 + 2").unwrap(), 7.0); // 5 + 2
        assert_eq!(get_var("result3"), "7");
    }

    #[test]
    fn test_power_operations() {
        assert_eq!(evaluate_expression("power1 = 2 ** 3").unwrap(), 8.0);
        assert_eq!(get_var("power1"), "8");

        assert_eq!(evaluate_expression("power2 = 3 ** 2").unwrap(), 9.0);
        assert_eq!(get_var("power2"), "9");

        // Power has higher precedence
        assert_eq!(evaluate_expression("power3 = 2 + 3 ** 2").unwrap(), 11.0); // 2 + 9
        assert_eq!(get_var("power3"), "11");
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(evaluate_expression("paren1 = (5 + 3) * 2").unwrap(), 16.0);
        assert_eq!(get_var("paren1"), "16");

        assert_eq!(evaluate_expression("paren2 = 2 * (10 - 3)").unwrap(), 14.0);
        assert_eq!(get_var("paren2"), "14");

        // Nested parentheses
        assert_eq!(evaluate_expression("paren3 = ((2 + 3) * (4 - 1))").unwrap(), 15.0);
        assert_eq!(get_var("paren3"), "15");
    }

    #[test]
    fn test_variable_usage() {
        // Set initial variables
        set_var("a", "10");
        set_var("b", "5");

        // Use variables in expressions
        assert_eq!(evaluate_expression("sum = a + b").unwrap(), 15.0);
        assert_eq!(get_var("sum"), "15");

        assert_eq!(evaluate_expression("product = a * b").unwrap(), 50.0);
        assert_eq!(get_var("product"), "50");

        // Use result variables in new expressions
        assert_eq!(evaluate_expression("final = sum + product").unwrap(), 65.0);
        assert_eq!(get_var("final"), "65");
    }

    #[test]
    fn test_shorthand_assignments() {
        set_var("counter", "10");

        // Addition shorthand
        assert_eq!(evaluate_expression("counter += 5").unwrap(), 15.0);
        assert_eq!(get_var("counter"), "15");

        // Subtraction shorthand
        assert_eq!(evaluate_expression("counter -= 3").unwrap(), 12.0);
        assert_eq!(get_var("counter"), "12");

        // Multiplication shorthand
        assert_eq!(evaluate_expression("counter *= 2").unwrap(), 24.0);
        assert_eq!(get_var("counter"), "24");

        // Division shorthand
        assert_eq!(evaluate_expression("counter /= 4").unwrap(), 6.0);
        assert_eq!(get_var("counter"), "6");
    }

    #[test]
    fn test_floating_point_numbers() {
        assert_eq!(evaluate_expression("pi = 3.14159").unwrap(), 3.14159);
        assert_eq!(get_var("pi"), "3.14159");

        assert_eq!(evaluate_expression("calc = 1.5 + 2.7").unwrap(), 4.2);
        assert_eq!(get_var("calc"), "4.2");

        assert_eq!(evaluate_expression("division = 22.0 / 7.0").unwrap(), 22.0 / 7.0);
    }

    #[test]
    fn test_modulo_operations() {
        assert_eq!(evaluate_expression("mod1 = 10 % 3").unwrap(), 1.0);
        assert_eq!(get_var("mod1"), "1");

        assert_eq!(evaluate_expression("mod2 = 15 % 4").unwrap(), 3.0);
        assert_eq!(get_var("mod2"), "3");

        assert_eq!(evaluate_expression("mod3 = 20 % 5").unwrap(), 0.0);
        assert_eq!(get_var("mod3"), "0");
    }

    #[test]
    fn test_error_cases() {
        // Invalid assignment format
        assert!(evaluate_expression("5 + 3").is_err()); // no assignment
        assert!(evaluate_expression("x + y = 5").is_err()); // invalid left side

        // Invalid variable references
        set_var("invalid_var", "not_a_number");
        assert!(evaluate_expression("result = invalid_var + 5").is_err());

        // Mismatched parentheses
        assert!(evaluate_expression("bad = (5 + 3").is_err()); // missing closing paren
        assert!(evaluate_expression("bad = 5 + 3)").is_err()); // extra closing paren
    }

    #[test]
    fn test_whitespace_handling() {
        assert_eq!(evaluate_expression("  spaced  =  5  +  3  ").unwrap(), 8.0);
        assert_eq!(get_var("spaced"), "8");

        assert_eq!(evaluate_expression("nospace=10*2").unwrap(), 20.0);
        assert_eq!(get_var("nospace"), "20");

        assert_eq!(evaluate_expression("mixed = ( 2 + 3 )*4").unwrap(), 20.0);
        assert_eq!(get_var("mixed"), "20");
    }

    #[test]
    fn test_complex_expressions() {
        set_var("x", "2");
        set_var("y", "3");
        set_var("z", "4");

        // Complex mathematical expression
        assert_eq!(evaluate_expression("complex = x ** 2 + y * z - 5").unwrap(), 11.0);
        // 2^2 + 3*4 - 5 = 4 + 12 - 5 = 11
        assert_eq!(get_var("complex"), "11");

        // Nested operations with variables
        assert_eq!(evaluate_expression("nested = (x + y) ** z / 2").unwrap(), 312.5);
        // (2+3)^4 / 2 = 5^4 / 2 = 625 / 2 = 312.5
        assert_eq!(get_var("nested"), "312.5");
    }

    #[test]
    fn test_edge_cases() {
        // Zero values
        assert_eq!(evaluate_expression("zero = 0 + 0").unwrap(), 0.0);
        assert_eq!(get_var("zero"), "0");

        // Negative numbers
        assert_eq!(evaluate_expression("neg = -5 + 3").unwrap(), -2.0);
        assert_eq!(get_var("neg"), "-2");

        // Large numbers
        assert_eq!(evaluate_expression("large = 1000000 + 1").unwrap(), 1000001.0);
        assert_eq!(get_var("large"), "1000001");

        // Division results in decimals
        assert_eq!(evaluate_expression("decimal = 7 / 2").unwrap(), 3.5);
        assert_eq!(get_var("decimal"), "3.5");
    }
}