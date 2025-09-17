//! Expression evaluation sanity tests

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use rsb::global::{get_var, set_var};
    use rsb::math::expressions::*;
    use std::sync::Mutex;

    lazy_static! {
        static ref TEST_LOCK: Mutex<()> = Mutex::new(());
    }

    fn clear_globals() {
        let keys: Vec<String> = rsb::global::get_all_vars().keys().cloned().collect();
        for k in keys {
            rsb::global::unset_var(&k);
        }
    }

    #[test]
    fn test_basic_expressions() {
        let _lock = TEST_LOCK.lock().unwrap();
        clear_globals();
        assert_eq!(evaluate_expression("expr_x = 5 + 3").unwrap(), 8.0);
        assert_eq!(get_var("expr_x"), "8");

        assert_eq!(evaluate_expression("expr_y = 10 - 4").unwrap(), 6.0);
        assert_eq!(get_var("expr_y"), "6");

        assert_eq!(evaluate_expression("expr_z = 3 * 4").unwrap(), 12.0);
        assert_eq!(get_var("expr_z"), "12");

        assert_eq!(evaluate_expression("w = 15 / 3").unwrap(), 5.0);
        assert_eq!(get_var("w"), "5");
    }

    #[test]
    fn test_operator_precedence() {
        let _lock = TEST_LOCK.lock().unwrap();
        clear_globals();
        assert_eq!(
            evaluate_expression("expr_result = 2 + 3 * 4").unwrap(),
            14.0
        ); // 2 + 12
        assert_eq!(get_var("expr_result"), "14");

        assert_eq!(
            evaluate_expression("expr_result2 = (2 + 3) * 4").unwrap(),
            20.0
        ); // 5 * 4
        assert_eq!(get_var("expr_result2"), "20");

        assert_eq!(
            evaluate_expression("expr_result3 = 20 / 4 + 2").unwrap(),
            7.0
        ); // 5 + 2
        assert_eq!(get_var("expr_result3"), "7");
    }

    #[test]
    fn test_power_operations() {
        let _lock = TEST_LOCK.lock().unwrap();
        clear_globals();
        assert_eq!(evaluate_expression("expr_power1 = 2 ** 3").unwrap(), 8.0);
        assert_eq!(get_var("expr_power1"), "8");

        assert_eq!(evaluate_expression("expr_power2 = 3 ** 2").unwrap(), 9.0);
        assert_eq!(get_var("expr_power2"), "9");

        // Power has higher precedence
        assert_eq!(
            evaluate_expression("expr_power3 = 2 + 3 ** 2").unwrap(),
            11.0
        ); // 2 + 9
        assert_eq!(get_var("expr_power3"), "11");
    }

    #[test]
    fn test_parentheses() {
        let _lock = TEST_LOCK.lock().unwrap();
        clear_globals();
        assert_eq!(
            evaluate_expression("expr_paren1 = (5 + 3) * 2").unwrap(),
            16.0
        );
        assert_eq!(get_var("expr_paren1"), "16");

        assert_eq!(
            evaluate_expression("expr_paren2 = 2 * (10 - 3)").unwrap(),
            14.0
        );
        assert_eq!(get_var("expr_paren2"), "14");

        // Nested parentheses
        assert_eq!(
            evaluate_expression("expr_paren3 = ((2 + 3) * (4 - 1))").unwrap(),
            15.0
        );
        assert_eq!(get_var("expr_paren3"), "15");
    }

    #[test]
    fn test_variable_usage() {
        let _lock = TEST_LOCK.lock().unwrap();
        clear_globals();
        // Set initial variables
        set_var("expr_a", "10");
        set_var("expr_b", "5");

        // Use variables in expressions
        assert_eq!(
            evaluate_expression("expr_sum = expr_a + expr_b").unwrap(),
            15.0
        );
        assert_eq!(get_var("expr_sum"), "15");

        assert_eq!(
            evaluate_expression("expr_product = expr_a * expr_b").unwrap(),
            50.0
        );
        assert_eq!(get_var("expr_product"), "50");

        // Use result variables in new expressions
        assert_eq!(
            evaluate_expression("expr_final = expr_sum + expr_product").unwrap(),
            65.0
        );
        assert_eq!(get_var("expr_final"), "65");
    }

    #[test]
    fn test_shorthand_assignments() {
        let _lock = TEST_LOCK.lock().unwrap();
        clear_globals();
        set_var("expr_counter", "10");

        // Addition shorthand
        assert_eq!(evaluate_expression("expr_counter += 5").unwrap(), 15.0);
        assert_eq!(get_var("expr_counter"), "15");

        // Subtraction shorthand
        assert_eq!(evaluate_expression("expr_counter -= 3").unwrap(), 12.0);
        assert_eq!(get_var("expr_counter"), "12");

        // Multiplication shorthand
        assert_eq!(evaluate_expression("expr_counter *= 2").unwrap(), 24.0);
        assert_eq!(get_var("expr_counter"), "24");

        // Division shorthand
        assert_eq!(evaluate_expression("expr_counter /= 4").unwrap(), 6.0);
        assert_eq!(get_var("expr_counter"), "6");
    }

    #[test]
    fn test_floating_point_numbers() {
        let _lock = TEST_LOCK.lock().unwrap();
        clear_globals();
        assert_eq!(evaluate_expression("expr_pi = 3.14159").unwrap(), 3.14159);
        assert_eq!(get_var("expr_pi"), "3.14159");

        assert_eq!(evaluate_expression("expr_calc = 1.5 + 2.7").unwrap(), 4.2);
        assert_eq!(get_var("expr_calc"), "4.2");

        assert_eq!(
            evaluate_expression("expr_division = 22.0 / 7.0").unwrap(),
            22.0 / 7.0
        );
    }

    #[test]
    fn test_modulo_operations() {
        let _lock = TEST_LOCK.lock().unwrap();
        clear_globals();
        assert_eq!(evaluate_expression("expr_mod1 = 10 % 3").unwrap(), 1.0);
        assert_eq!(get_var("expr_mod1"), "1");

        assert_eq!(evaluate_expression("expr_mod2 = 15 % 4").unwrap(), 3.0);
        assert_eq!(get_var("expr_mod2"), "3");

        assert_eq!(evaluate_expression("expr_mod3 = 20 % 5").unwrap(), 0.0);
        assert_eq!(get_var("expr_mod3"), "0");
    }

    #[test]
    fn test_error_cases() {
        let _lock = TEST_LOCK.lock().unwrap();
        clear_globals();
        // Invalid assignment format
        assert!(evaluate_expression("5 + 3").is_err()); // no assignment
        assert!(evaluate_expression("x + y = 5").is_err()); // invalid left side

        // Invalid variable references
        set_var("expr_invalid_var", "not_a_number");
        assert!(evaluate_expression("expr_result = expr_invalid_var + 5").is_err());

        // Mismatched parentheses
        assert!(evaluate_expression("expr_bad = (5 + 3").is_err()); // missing closing paren
        assert!(evaluate_expression("expr_bad = 5 + 3)").is_err()); // extra closing paren
    }

    #[test]
    fn test_whitespace_handling() {
        let _lock = TEST_LOCK.lock().unwrap();
        clear_globals();
        assert_eq!(
            evaluate_expression("  expr_spaced  =  5  +  3  ").unwrap(),
            8.0
        );
        assert_eq!(get_var("expr_spaced"), "8");

        assert_eq!(evaluate_expression("expr_nospace=10*2").unwrap(), 20.0);
        assert_eq!(get_var("expr_nospace"), "20");

        assert_eq!(
            evaluate_expression("expr_mixed = ( 2 + 3 )*4").unwrap(),
            20.0
        );
        assert_eq!(get_var("expr_mixed"), "20");
    }

    #[test]
    fn test_complex_expressions() {
        let _lock = TEST_LOCK.lock().unwrap();
        clear_globals();
        set_var("expr_x", "2");
        set_var("expr_y", "3");
        set_var("expr_z", "4");

        // Complex mathematical expression
        assert_eq!(
            evaluate_expression("expr_complex = expr_x ** 2 + expr_y * expr_z - 5").unwrap(),
            11.0
        );
        // 2^2 + 3*4 - 5 = 4 + 12 - 5 = 11
        assert_eq!(get_var("expr_complex"), "11");

        // Nested operations with variables
        assert_eq!(
            evaluate_expression("expr_nested = (expr_x + expr_y) ** expr_z / 2").unwrap(),
            312.5
        );
        // (2+3)^4 / 2 = 5^4 / 2 = 625 / 2 = 312.5
        assert_eq!(get_var("expr_nested"), "312.5");
    }

    #[test]
    fn test_edge_cases() {
        let _lock = TEST_LOCK.lock().unwrap();
        clear_globals();
        // Zero values
        assert_eq!(evaluate_expression("expr_zero = 0 + 0").unwrap(), 0.0);
        assert_eq!(get_var("expr_zero"), "0");

        // Negative numbers
        assert_eq!(evaluate_expression("expr_neg = -5 + 3").unwrap(), -2.0);
        assert_eq!(get_var("expr_neg"), "-2");

        // Large numbers
        assert_eq!(
            evaluate_expression("expr_large = 1000000 + 1").unwrap(),
            1000001.0
        );
        assert_eq!(get_var("expr_large"), "1000001");

        // Division results in decimals
        assert_eq!(evaluate_expression("expr_decimal = 7 / 2").unwrap(), 3.5);
        assert_eq!(get_var("expr_decimal"), "3.5");
    }
}
