//! Math module UAT tests - visual demonstration of math capabilities

#[cfg(test)]
mod tests {
    use rsb::*;

    #[test]
    fn test_basic_operations_demo() {
        println!("=== Basic Math Operations Demo ===");

        println!("Addition: {} + {} = {}", 15.5, 7.3, add!(15.5, 7.3));
        println!("Subtraction: {} - {} = {}", 20.0, 8.5, subtract!(20.0, 8.5));
        println!("Multiplication: {} * {} = {}", 6.5, 4.0, multiply!(6.5, 4.0));
        println!("Division: {} / {} = {}", 42.0, 7.0, divide!(42.0, 7.0));
        println!("Power: {}^{} = {}", 2.0, 10.0, power!(2.0, 10.0));
        println!("Square root: √{} = {}", 144.0, sqrt!(144.0));

        println!("\nRounding operations:");
        let pi = 3.14159265;
        println!("π = {}", pi);
        println!("round(π, 2) = {}", roundup!(pi, 2));
        println!("roundup(π, 2) = {}", roundup!(pi, 2));
        println!("rounddown(π, 2) = {}", rounddown!(pi, 2));
        println!("floor(π) = {}", floor!(pi));
        println!("ceil(π) = {}", ceil!(pi));
    }

    #[test]
    fn test_percentage_operations_demo() {
        println!("\n=== Percentage Operations Demo ===");

        let salary = 75000.0;
        let raise_percent = 12.5;
        let bonus = percent_of!(salary, raise_percent);

        println!("Annual salary: ${}", salary);
        println!("Raise percentage: {}%", raise_percent);
        println!("Bonus amount: ${}", bonus);
        println!("New salary: ${}", salary + bonus);

        let old_price = 85.0;
        let new_price = 102.0;
        let price_change = percent_change!(old_price, new_price);

        println!("\nStock price change:");
        println!("Old price: ${}", old_price);
        println!("New price: ${}", new_price);
        println!("Change: {}%", price_change);

        println!("\nAspect ratios:");
        println!("16:9 ratio = {:.3}", ratio!(16.0, 9.0));
        println!("4:3 ratio = {:.3}", ratio!(4.0, 3.0));
    }

    #[test]
    fn test_predicate_operations_demo() {
        println!("\n=== Predicate Operations Demo ===");

        let test_numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

        print!("Even numbers: ");
        for &n in &test_numbers {
            if even!(n) {
                print!("{} ", n);
            }
        }
        println!();

        print!("Odd numbers: ");
        for &n in &test_numbers {
            if odd!(n) {
                print!("{} ", n);
            }
        }
        println!();

        println!("\nModulo operations:");
        for &n in &[10, 15, 17, 22, 25] {
            println!("{} mod 3 = {}", n, modulo!(n, 3));
        }

        println!("\nSign tests:");
        for &n in &[-5.5, -0.1, 0.0, 0.1, 5.5] {
            println!("sign({}) = {}", n, sign!(n));
        }
    }

    #[test]
    fn test_aggregator_operations_demo() {
        println!("\n=== Aggregator Operations Demo ===");

        let test_scores = vec![85.5, 92.0, 78.5, 96.5, 88.0, 91.5, 84.0, 89.0];
        let sales_data = vec![1250.75, 980.25, 1450.50, 1100.00, 1320.25, 1680.75, 890.50];

        println!("Test Scores: {:?}", test_scores);
        println!("Average: {:.2}", avg!(&test_scores));
        println!("Median: {:.2}", median!(&test_scores));
        println!("Min: {:.2}", min_list!(&test_scores));
        println!("Max: {:.2}", max_list!(&test_scores));
        println!("Sum: {:.2}", sum_list!(&test_scores));

        println!("\nWeekly Sales Data: {:?}", sales_data);
        println!("Total Sales: ${:.2}", sum_list!(&sales_data));
        println!("Average Daily: ${:.2}", avg!(&sales_data));
        println!("Best Day: ${:.2}", max_list!(&sales_data));
        println!("Worst Day: ${:.2}", min_list!(&sales_data));
    }

    #[test]
    fn test_base_conversion_demo() {
        println!("\n=== Base Conversion Demo ===");

        let decimal_nums = vec![15, 255, 1024, 65535];

        for &num in &decimal_nums {
            println!("\nDecimal {}: ", num);
            println!("  Binary:     {}", to_binary!(num));
            println!("  Octal:      {}", to_octal!(num));
            println!("  Hex (lower): {}", to_hex!(num));
            println!("  Hex (upper): {}", to_hex_upper!(num));
            println!("  Base 36:    {}", to_base!(num, 36));
        }

        println!("\nConversion examples:");
        println!("Binary '1111' to decimal: {}", from_binary!("1111"));
        println!("Hex 'FF' to decimal: {}", from_hex!("FF"));
        println!("Octal '377' to decimal: {}", from_octal!("377"));

        println!("\nBase conversion:");
        println!("Binary '11111111' to hex: {}", base_convert!("11111111", 2, 16));
        println!("Hex 'FF' to binary: {}", base_convert!("FF", 16, 2));
    }

    #[test]
    fn test_integer_operations_demo() {
        println!("\n=== Integer Operations Demo ===");

        let num_pairs = vec![(48, 18), (12, 8), (15, 25), (100, 75)];

        for &(a, b) in &num_pairs {
            println!("\nNumbers: {} and {}", a, b);
            println!("  GCD: {}", gcd!(a, b));
            println!("  LCM: {}", lcm!(a, b));
        }

        println!("\nPrime number check:");
        for &n in &[2, 3, 4, 5, 17, 25, 29, 100, 101] {
            println!("  {} is {}", n, if is_prime!(n) { "prime" } else { "not prime" });
        }

        println!("\nFactorials:");
        for n in 0..=10 {
            match factorial!(n) {
                Ok(result) => println!("  {}! = {}", n, result),
                Err(e) => println!("  {}!: {}", n, e),
            }
        }

        println!("\nFibonacci sequence (first 15 numbers):");
        print!("  ");
        for i in 0..15 {
            print!("{} ", fibonacci!(i));
        }
        println!();
    }

    #[test]
    fn test_random_operations_demo() {
        println!("\n=== Random Operations Demo ===");

        println!("Random numbers in range:");
        println!("  Float [0.0, 1.0]: {:.3}", random_range!(0.0, 1.0));
        println!("  Float [10.0, 20.0]: {:.3}", random_range!(10.0, 20.0));
        println!("  Integer [1, 100]: {}", random_int_range!(1, 100));
        println!("  Integer [50, 60]: {}", random_int_range!(50, 60));

        println!("\nRandom lists:");
        println!("  3 booleans: {}", random_list!(3, "bool"));
        println!("  5 integers [1:10]: {}", random_list!(5, "int", "1:10"));
        println!("  4 floats [0.0:1.0]: {}", random_list!(4, "float", "0.0:1.0"));

        // Generate multiple samples to show variety
        println!("\nMultiple samples of random_list!(3, \"int\", \"1:6\") (dice rolls):");
        for i in 1..=5 {
            println!("  Roll {}: {}", i, random_list!(3, "int", "1:6"));
        }
    }

    #[test]
    fn test_expression_evaluation_demo() {
        println!("\n=== Expression Evaluation Demo ===");

        // Set some variables for demonstration
        set_var!("x", "10");
        set_var!("y", "5");
        set_var!("radius", "7.5");

        println!("Variables: x = {}, y = {}, radius = {}",
                get_var!("x"), get_var!("y"), get_var!("radius"));

        // Demonstrate various expressions
        let expressions = vec![
            "result1 = x + y * 2",
            "result2 = (x + y) ** 2",
            "area = 3.14159 * radius ** 2",
            "temp = x * 1.8 + 32",
            "counter = 0",
            "counter += 5",
            "counter *= 3",
        ];

        println!("\nExpression evaluation:");
        for expr in expressions {
            match math!(expr) {
                result => println!("  {} → {}", expr, result),
            }
        }

        println!("\nFinal variable values:");
        println!("  result1 = {}", get_var!("result1"));
        println!("  result2 = {}", get_var!("result2"));
        println!("  area = {}", get_var!("area"));
        println!("  temp = {}", get_var!("temp"));
        println!("  counter = {}", get_var!("counter"));
    }

    #[test]
    fn test_comprehensive_math_showcase() {
        println!("\n=== Comprehensive Math Showcase ===");
        println!("Calculating statistics for a dataset and converting results to different formats\n");

        // Simulate test scores
        let scores = vec![78.5, 85.0, 91.5, 88.0, 76.5, 93.0, 82.5, 89.5, 95.0, 87.0];

        println!("Test Scores: {:?}", scores);

        let mean_score = avg!(&scores);
        let median_score = median!(&scores);
        let min_score = min_list!(&scores);
        let max_score = max_list!(&scores);

        println!("\nStatistics:");
        println!("  Mean: {:.2}", mean_score);
        println!("  Median: {:.2}", median_score);
        println!("  Range: {:.2} - {:.2}", min_score, max_score);

        // Convert mean to different representations
        let mean_int = mean_score as i64;
        println!("\nMean score ({}) in different bases:", mean_int);
        println!("  Binary: {}", to_binary!(mean_int));
        println!("  Octal: {}", to_octal!(mean_int));
        println!("  Hex: {}", to_hex_upper!(mean_int));

        // Check if mean is even/odd
        println!("  Is even: {}", even!(mean_int));
        println!("  Is odd: {}", odd!(mean_int));

        // Calculate grade improvements
        println!("\nGrade analysis:");
        let target_score = 90.0;
        for (i, &score) in scores.iter().enumerate() {
            if score < target_score {
                let improvement_needed = target_score - score;
                let percent_improvement = percent_change!(score, target_score);
                println!("  Student {}: needs {:.1} points ({:.1}% improvement)",
                        i+1, improvement_needed, percent_improvement);
            } else {
                println!("  Student {}: exceeds target!", i+1);
            }
        }

        // Generate random bonus points
        println!("\nRandom bonus points awarded:");
        let bonus_points = random_list!(10, "int", "1:5");
        println!("  Bonus: {}", bonus_points);

        println!("\n🎯 Math module demonstration complete!");
    }
}