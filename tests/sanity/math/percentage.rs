//! Percentage operations sanity tests

#[cfg(test)]
mod tests {
    use rsb::math::percentage::*;

    #[test]
    fn test_percent_of() {
        assert_eq!(percent_of(100.0, 25.0), 25.0);
        assert_eq!(percent_of(200.0, 50.0), 100.0);
        assert_eq!(percent_of(80.0, 12.5), 10.0);
        assert_eq!(percent_of(1000.0, 0.0), 0.0);
    }

    #[test]
    fn test_percent_change() {
        assert_eq!(percent_change(100.0, 150.0), 50.0); // 50% increase
        assert_eq!(percent_change(100.0, 75.0), -25.0); // 25% decrease
        assert_eq!(percent_change(100.0, 100.0), 0.0); // no change
        assert_eq!(percent_change(50.0, 100.0), 100.0); // 100% increase

        // Edge case: from zero
        assert_eq!(percent_change(0.0, 0.0), 0.0);
        assert!(percent_change(0.0, 10.0).is_infinite());
    }

    #[test]
    fn test_ratio() {
        assert_eq!(ratio(3.0, 4.0).unwrap(), 0.75);
        assert_eq!(ratio(1.0, 2.0).unwrap(), 0.5);
        assert_eq!(ratio(10.0, 5.0).unwrap(), 2.0);
        assert_eq!(ratio(0.0, 5.0).unwrap(), 0.0);

        // Division by zero
        assert!(ratio(5.0, 0.0).is_err());
        assert!(ratio(5.0, 0.0).unwrap_err().contains("Division by zero"));
    }

    #[test]
    fn test_percentage_decimal_conversion() {
        assert_eq!(percentage_to_decimal(25.0), 0.25);
        assert_eq!(percentage_to_decimal(100.0), 1.0);
        assert_eq!(percentage_to_decimal(0.0), 0.0);
        assert_eq!(percentage_to_decimal(150.0), 1.5);

        assert_eq!(decimal_to_percentage(0.25), 25.0);
        assert_eq!(decimal_to_percentage(1.0), 100.0);
        assert_eq!(decimal_to_percentage(0.0), 0.0);
        assert_eq!(decimal_to_percentage(1.5), 150.0);
    }

    #[test]
    fn test_real_world_scenarios() {
        // Sales tax calculation
        let price = 100.0;
        let tax_rate = 8.5;
        let tax_amount = percent_of(price, tax_rate);
        assert_eq!(tax_amount, 8.5);

        // Stock price change
        let old_price = 50.0;
        let new_price = 55.0;
        let change_percent = percent_change(old_price, new_price);
        assert_eq!(change_percent, 10.0);

        // Aspect ratio
        let width = 1920.0;
        let height = 1080.0;
        let aspect_ratio = ratio(width, height).unwrap();
        assert!((aspect_ratio - 16.0 / 9.0).abs() < 0.001);
    }
}
