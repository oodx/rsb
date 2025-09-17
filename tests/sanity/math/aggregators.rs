//! Aggregators operations sanity tests

#[cfg(test)]
mod tests {
    use rsb::math::aggregators::*;

    #[test]
    fn test_min_max_list() {
        let numbers = vec![5.0, 2.0, 8.0, 1.0, 9.0];
        assert_eq!(min_list(&numbers).unwrap(), 1.0);
        assert_eq!(max_list(&numbers).unwrap(), 9.0);

        // Single element
        let single = vec![42.0];
        assert_eq!(min_list(&single).unwrap(), 42.0);
        assert_eq!(max_list(&single).unwrap(), 42.0);

        // Empty list
        let empty: Vec<f64> = vec![];
        assert!(min_list(&empty).is_none());
        assert!(max_list(&empty).is_none());

        // Negative numbers
        let negatives = vec![-5.0, -2.0, -8.0, -1.0];
        assert_eq!(min_list(&negatives).unwrap(), -8.0);
        assert_eq!(max_list(&negatives).unwrap(), -1.0);
    }

    #[test]
    fn test_sum_list() {
        let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(sum_list(&numbers), 15.0);

        let empty: Vec<f64> = vec![];
        assert_eq!(sum_list(&empty), 0.0);

        let negatives = vec![-1.0, -2.0, 3.0];
        assert_eq!(sum_list(&negatives), 0.0);

        let single = vec![42.5];
        assert_eq!(sum_list(&single), 42.5);
    }

    #[test]
    fn test_avg_mean() {
        let numbers = vec![2.0, 4.0, 6.0, 8.0];
        assert_eq!(avg(&numbers).unwrap(), 5.0);
        assert_eq!(mean(&numbers).unwrap(), 5.0); // mean is alias for avg

        let single = vec![42.0];
        assert_eq!(avg(&single).unwrap(), 42.0);

        let empty: Vec<f64> = vec![];
        assert!(avg(&empty).is_none());
        assert!(mean(&empty).is_none());

        // Test with decimal result
        let decimals = vec![1.0, 2.0, 3.0];
        assert_eq!(avg(&decimals).unwrap(), 2.0);
    }

    #[test]
    fn test_median() {
        // Odd number of elements
        let odd = vec![1.0, 3.0, 5.0, 7.0, 9.0];
        assert_eq!(median(&odd).unwrap(), 5.0);

        // Even number of elements
        let even = vec![1.0, 2.0, 3.0, 4.0];
        assert_eq!(median(&even).unwrap(), 2.5); // (2+3)/2

        // Single element
        let single = vec![42.0];
        assert_eq!(median(&single).unwrap(), 42.0);

        // Empty list
        let empty: Vec<f64> = vec![];
        assert!(median(&empty).is_none());

        // Unsorted input should still work
        let unsorted = vec![9.0, 1.0, 5.0, 3.0, 7.0];
        assert_eq!(median(&unsorted).unwrap(), 5.0);
    }

    #[test]
    fn test_real_world_scenarios() {
        // Test scores
        let test_scores = vec![85.0, 92.0, 78.0, 96.0, 88.0, 91.0, 84.0];
        let average_score = avg(&test_scores).unwrap();
        assert!((average_score - 87.714).abs() < 0.01);

        // Temperature readings
        let temperatures = vec![72.5, 75.2, 73.8, 71.9, 76.1];
        let min_temp = min_list(&temperatures).unwrap();
        let max_temp = max_list(&temperatures).unwrap();
        let avg_temp = avg(&temperatures).unwrap();

        assert_eq!(min_temp, 71.9);
        assert_eq!(max_temp, 76.1);
        assert!((avg_temp - 73.9).abs() < 0.1);

        // Sales data
        let daily_sales = vec![1250.0, 980.0, 1450.0, 1100.0, 1320.0, 1680.0, 890.0];
        let total_sales = sum_list(&daily_sales);
        let median_sales = median(&daily_sales).unwrap();

        assert_eq!(total_sales, 8670.0);
        assert_eq!(median_sales, 1250.0);
    }

    #[test]
    fn test_edge_cases() {
        // All same values
        let same_values = vec![5.0, 5.0, 5.0, 5.0];
        assert_eq!(min_list(&same_values).unwrap(), 5.0);
        assert_eq!(max_list(&same_values).unwrap(), 5.0);
        assert_eq!(avg(&same_values).unwrap(), 5.0);
        assert_eq!(median(&same_values).unwrap(), 5.0);

        // Very large numbers
        let large_numbers = vec![1e10, 2e10, 3e10];
        assert_eq!(sum_list(&large_numbers), 6e10);
        assert_eq!(avg(&large_numbers).unwrap(), 2e10);
    }
}
