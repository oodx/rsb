//! Math module UAT tests - visual demonstration of math capabilities

#[cfg(test)]
mod tests {
    use rsb::*;
    use rsb::global::{set_var, get_var};

    #[test]
    fn test_basic_operations_demo() {
        println!("\n🎯 UAT Test 1: Basic Math Operations Demo");
        println!("{}", "=".repeat(50));

        println!("\n📋 Section 1.1: Basic Arithmetic");
        println!("   \x1b[36mCommand:\x1b[0m add!(15.5, 7.3)");
        println!("   \x1b[32mResult:\x1b[0m  {} + {} = {}", 15.5, 7.3, add!(15.5, 7.3));

        println!("   \x1b[36mCommand:\x1b[0m subtract!(20.0, 8.5)");
        println!("   \x1b[32mResult:\x1b[0m  {} - {} = {}", 20.0, 8.5, subtract!(20.0, 8.5));

        println!("   \x1b[36mCommand:\x1b[0m multiply!(6.5, 4.0)");
        println!("   \x1b[32mResult:\x1b[0m  {} × {} = {}", 6.5, 4.0, multiply!(6.5, 4.0));

        println!("   \x1b[36mCommand:\x1b[0m divide!(42.0, 7.0)");
        println!("   \x1b[32mResult:\x1b[0m  {} ÷ {} = {}", 42.0, 7.0, divide!(42.0, 7.0));

        println!("\n📋 Section 1.2: Advanced Operations");
        println!("   \x1b[36mCommand:\x1b[0m power!(2.0, 10.0)");
        println!("   \x1b[32mResult:\x1b[0m  2^10 = {}", power!(2.0, 10.0));

        println!("   \x1b[36mCommand:\x1b[0m sqrt!(144.0)");
        println!("   \x1b[32mResult:\x1b[0m  √144 = {}", sqrt!(144.0));

        println!("\n📋 Section 1.3: Rounding Operations");
        let pi = 3.14159265;
        println!("   π = {}", pi);

        println!("   \x1b[36mCommand:\x1b[0m roundup!(π, 2)");
        println!("   \x1b[32mResult:\x1b[0m  round(π, 2) = {}", roundup!(pi, 2));

        println!("   \x1b[36mCommand:\x1b[0m rounddown!(π, 2)");
        println!("   \x1b[32mResult:\x1b[0m  rounddown(π, 2) = {}", rounddown!(pi, 2));

        println!("   \x1b[36mCommand:\x1b[0m floor!(π)");
        println!("   \x1b[32mResult:\x1b[0m  floor(π) = {}", floor!(pi));

        println!("   \x1b[36mCommand:\x1b[0m ceil!(π)");
        println!("   \x1b[32mResult:\x1b[0m  ceil(π) = {}", ceil!(pi));

        println!("\n✅ Basic Operations Demo Complete\n");
    }

    #[test]
    fn test_percentage_operations_demo() {
        println!("\n🎯 UAT Test 2: Percentage Operations Demo");
        println!("{}", "=".repeat(50));

        println!("\n📋 Section 2.1: Salary Calculations");
        let salary = 75000.0;
        let raise_percent = 12.5;
        println!("   \x1b[36mCommand:\x1b[0m percent_of!({}, {})", salary, raise_percent);
        let bonus = percent_of!(salary, raise_percent);
        println!("   \x1b[32mResult:\x1b[0m  Annual salary: ${}", salary);
        println!("   \x1b[32mResult:\x1b[0m  Raise percentage: {}%", raise_percent);
        println!("   \x1b[32mResult:\x1b[0m  Bonus amount: ${}", bonus);
        println!("   \x1b[32mResult:\x1b[0m  New salary: ${}", salary + bonus);

        println!("\n📋 Section 2.2: Price Change Analysis");
        let old_price = 85.0;
        let new_price = 102.0;
        println!("   \x1b[36mCommand:\x1b[0m percent_change!({}, {})", old_price, new_price);
        let price_change = percent_change!(old_price, new_price);
        println!("   \x1b[32mResult:\x1b[0m  Old price: ${}", old_price);
        println!("   \x1b[32mResult:\x1b[0m  New price: ${}", new_price);
        println!("   \x1b[32mResult:\x1b[0m  Change: {}%", price_change);

        println!("\n📋 Section 2.3: Aspect Ratios");
        println!("   \x1b[36mCommand:\x1b[0m ratio!(16.0, 9.0)");
        println!("   \x1b[32mResult:\x1b[0m  16:9 ratio = {:.3}", ratio!(16.0, 9.0));

        println!("   \x1b[36mCommand:\x1b[0m ratio!(4.0, 3.0)");
        println!("   \x1b[32mResult:\x1b[0m  4:3 ratio = {:.3}", ratio!(4.0, 3.0));

        println!("\n✅ Percentage Operations Demo Complete\n");
    }

    #[test]
    fn test_predicate_operations_demo() {
        println!("\n🎯 UAT Test 3: Predicate Operations Demo");
        println!("{}", "=".repeat(50));

        println!("\n📋 Section 3.1: Even/Odd Detection");
        let test_numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

        println!("   \x1b[36mCommand:\x1b[0m even!(n) for n in 1..12");
        print!("   \x1b[32mResult:\x1b[0m  Even numbers: ");
        for &n in &test_numbers {
            if even!(n) {
                print!("{} ", n);
            }
        }
        println!();

        println!("   \x1b[36mCommand:\x1b[0m odd!(n) for n in 1..12");
        print!("   \x1b[32mResult:\x1b[0m  Odd numbers: ");
        for &n in &test_numbers {
            if odd!(n) {
                print!("{} ", n);
            }
        }
        println!();

        println!("\n📋 Section 3.2: Modulo Operations");
        for &n in &[10, 15, 17, 22, 25] {
            println!("   \x1b[36mCommand:\x1b[0m modulo!({}, 3)", n);
            println!("   \x1b[32mResult:\x1b[0m  {} mod 3 = {}", n, modulo!(n, 3));
        }

        println!("\n📋 Section 3.3: Sign Detection");
        for &n in &[-5.5, -0.1, 0.0, 0.1, 5.5] {
            println!("   \x1b[36mCommand:\x1b[0m sign!({})", n);
            println!("   \x1b[32mResult:\x1b[0m  sign({}) = {}", n, sign!(n));
        }

        println!("\n✅ Predicate Operations Demo Complete\n");
    }

    #[test]
    fn test_aggregator_operations_demo() {
        println!("\n🎯 UAT Test 4: Aggregator Operations Demo");
        println!("{}", "=".repeat(50));

        let test_scores = vec![85.5, 92.0, 78.5, 96.5, 88.0, 91.5, 84.0, 89.0];
        let sales_data = vec![1250.75, 980.25, 1450.50, 1100.00, 1320.25, 1680.75, 890.50];

        println!("\n📋 Section 4.1: Test Score Analysis");
        println!("   \x1b[36mData:\x1b[0m Test Scores: {:?}", test_scores);
        println!("   \x1b[36mCommand:\x1b[0m avg!(&test_scores)");
        println!("   \x1b[32mResult:\x1b[0m  Average: {:.2}", avg!(&test_scores));

        println!("   \x1b[36mCommand:\x1b[0m median!(&test_scores)");
        println!("   \x1b[32mResult:\x1b[0m  Median: {:.2}", median!(&test_scores));

        println!("   \x1b[36mCommand:\x1b[0m min_list!(&test_scores)");
        println!("   \x1b[32mResult:\x1b[0m  Min: {:.2}", min_list!(&test_scores));

        println!("   \x1b[36mCommand:\x1b[0m max_list!(&test_scores)");
        println!("   \x1b[32mResult:\x1b[0m  Max: {:.2}", max_list!(&test_scores));

        println!("   \x1b[36mCommand:\x1b[0m sum_list!(&test_scores)");
        println!("   \x1b[32mResult:\x1b[0m  Sum: {:.2}", sum_list!(&test_scores));

        println!("\n📋 Section 4.2: Sales Data Analysis");
        println!("   \x1b[36mData:\x1b[0m Weekly Sales: {:?}", sales_data);
        println!("   \x1b[36mCommand:\x1b[0m sum_list!(&sales_data)");
        println!("   \x1b[32mResult:\x1b[0m  Total Sales: ${:.2}", sum_list!(&sales_data));

        println!("   \x1b[36mCommand:\x1b[0m avg!(&sales_data)");
        println!("   \x1b[32mResult:\x1b[0m  Average Daily: ${:.2}", avg!(&sales_data));

        println!("   \x1b[36mCommand:\x1b[0m max_list!(&sales_data)");
        println!("   \x1b[32mResult:\x1b[0m  Best Day: ${:.2}", max_list!(&sales_data));

        println!("   \x1b[36mCommand:\x1b[0m min_list!(&sales_data)");
        println!("   \x1b[32mResult:\x1b[0m  Worst Day: ${:.2}", min_list!(&sales_data));

        println!("\n✅ Aggregator Operations Demo Complete\n");
    }

    #[test]
    fn test_base_conversion_demo() {
        println!("\n🎯 UAT Test 5: Base Conversion Demo");
        println!("{}", "=".repeat(50));

        let decimal_nums = vec![15, 255, 1024, 65535];

        println!("\n📋 Section 5.1: Number Base Conversions");
        for &num in &decimal_nums {
            println!("   \x1b[36mDecimal:\x1b[0m {}", num);
            println!("   \x1b[36mCommand:\x1b[0m to_binary!({})", num);
            println!("   \x1b[32mResult:\x1b[0m  Binary: {}", to_binary!(num));

            println!("   \x1b[36mCommand:\x1b[0m to_octal!({})", num);
            println!("   \x1b[32mResult:\x1b[0m  Octal: {}", to_octal!(num));

            println!("   \x1b[36mCommand:\x1b[0m to_hex!({})", num);
            println!("   \x1b[32mResult:\x1b[0m  Hex (lower): {}", to_hex!(num));

            println!("   \x1b[36mCommand:\x1b[0m to_hex_upper!({})", num);
            println!("   \x1b[32mResult:\x1b[0m  Hex (upper): {}", to_hex_upper!(num));

            println!("   \x1b[36mCommand:\x1b[0m to_base!({}, 36)", num);
            println!("   \x1b[32mResult:\x1b[0m  Base 36: {}", to_base!(num, 36));
            println!();
        }

        println!("📋 Section 5.2: String to Decimal Conversions");
        println!("   \x1b[36mCommand:\x1b[0m from_binary!(\"1111\")");
        println!("   \x1b[32mResult:\x1b[0m  Binary '1111' to decimal: {}", from_binary!("1111"));

        println!("   \x1b[36mCommand:\x1b[0m from_hex!(\"FF\")");
        println!("   \x1b[32mResult:\x1b[0m  Hex 'FF' to decimal: {}", from_hex!("FF"));

        println!("   \x1b[36mCommand:\x1b[0m from_octal!(\"377\")");
        println!("   \x1b[32mResult:\x1b[0m  Octal '377' to decimal: {}", from_octal!("377"));

        println!("\n📋 Section 5.3: Base-to-Base Conversions");
        println!("   \x1b[36mCommand:\x1b[0m base_convert!(\"11111111\", 2, 16)");
        println!("   \x1b[32mResult:\x1b[0m  Binary '11111111' to hex: {}", base_convert!("11111111", 2, 16));

        println!("   \x1b[36mCommand:\x1b[0m base_convert!(\"FF\", 16, 2)");
        println!("   \x1b[32mResult:\x1b[0m  Hex 'FF' to binary: {}", base_convert!("FF", 16, 2));

        println!("\n✅ Base Conversion Demo Complete\n");
    }

    #[test]
    fn test_integer_operations_demo() {
        println!("\n🎯 UAT Test 6: Integer Operations Demo");
        println!("{}", "=".repeat(50));

        let num_pairs = vec![(48, 18), (12, 8), (15, 25), (100, 75)];

        println!("\n📋 Section 6.1: GCD and LCM Calculations");
        for &(a, b) in &num_pairs {
            println!("   \x1b[36mNumbers:\x1b[0m {} and {}", a, b);
            println!("   \x1b[36mCommand:\x1b[0m gcd!({}, {})", a, b);
            println!("   \x1b[32mResult:\x1b[0m  GCD: {}", gcd!(a, b));

            println!("   \x1b[36mCommand:\x1b[0m lcm!({}, {})", a, b);
            println!("   \x1b[32mResult:\x1b[0m  LCM: {}", lcm!(a, b));
            println!();
        }

        println!("📋 Section 6.2: Prime Number Detection");
        for &n in &[2, 3, 4, 5, 17, 25, 29, 100, 101] {
            println!("   \x1b[36mCommand:\x1b[0m is_prime!({})", n);
            println!("   \x1b[32mResult:\x1b[0m  {} is {}", n, if is_prime!(n) { "prime" } else { "not prime" });
        }

        println!("\n📋 Section 6.3: Factorial Calculations");
        for n in 0..=10 {
            println!("   \x1b[36mCommand:\x1b[0m factorial!({})", n);
            match factorial!(n) {
                Ok(result) => println!("   \x1b[32mResult:\x1b[0m  {}! = {}", n, result),
                Err(e) => println!("   \x1b[32mResult:\x1b[0m  {}!: {}", n, e),
            }
        }

        println!("\n📋 Section 6.4: Fibonacci Sequence");
        println!("   \x1b[36mCommand:\x1b[0m fibonacci!(i) for i in 0..15");
        print!("   \x1b[32mResult:\x1b[0m  ");
        for i in 0..15 {
            print!("{} ", fibonacci!(i));
        }
        println!();

        println!("\n✅ Integer Operations Demo Complete\n");
    }

    #[test]
    fn test_random_operations_demo() {
        println!("\n🎯 UAT Test 7: Random Operations Demo");
        println!("{}", "=".repeat(50));

        println!("\n📋 Section 7.1: Random Number Generation");
        println!("   \x1b[36mCommand:\x1b[0m random_range!(0.0, 1.0)");
        println!("   \x1b[32mResult:\x1b[0m  Float [0.0, 1.0]: {:.3}", random_range!(0.0, 1.0));

        println!("   \x1b[36mCommand:\x1b[0m random_range!(10.0, 20.0)");
        println!("   \x1b[32mResult:\x1b[0m  Float [10.0, 20.0]: {:.3}", random_range!(10.0, 20.0));

        println!("   \x1b[36mCommand:\x1b[0m random_int_range!(1, 100)");
        println!("   \x1b[32mResult:\x1b[0m  Integer [1, 100]: {}", random_int_range!(1, 100));

        println!("   \x1b[36mCommand:\x1b[0m random_int_range!(50, 60)");
        println!("   \x1b[32mResult:\x1b[0m  Integer [50, 60]: {}", random_int_range!(50, 60));

        println!("\n📋 Section 7.2: Random List Generation");
        println!("   \x1b[36mCommand:\x1b[0m random_list!(3, \"bool\")");
        println!("   \x1b[32mResult:\x1b[0m  3 booleans: {}", random_list!(3, "bool"));

        println!("   \x1b[36mCommand:\x1b[0m random_list!(5, \"int\", \"1:10\")");
        println!("   \x1b[32mResult:\x1b[0m  5 integers [1:10]: {}", random_list!(5, "int", "1:10"));

        println!("   \x1b[36mCommand:\x1b[0m random_list!(4, \"float\", \"0.0:1.0\")");
        println!("   \x1b[32mResult:\x1b[0m  4 floats [0.0:1.0]: {}", random_list!(4, "float", "0.0:1.0"));

        println!("\n📋 Section 7.3: Multiple Random Samples");
        println!("   \x1b[36mCommand:\x1b[0m random_list!(3, \"int\", \"1:6\") (dice rolls)");
        for i in 1..=5 {
            println!("   \x1b[32mResult:\x1b[0m  Roll {}: {}", i, random_list!(3, "int", "1:6"));
        }

        println!("\n✅ Random Operations Demo Complete\n");
    }

    #[test]
    fn test_expression_evaluation_demo() {
        println!("\n🎯 UAT Test 8: Expression Evaluation Demo");
        println!("{}", "=".repeat(50));

        println!("\n📋 Section 8.1: Variable Setup");
        println!("   \x1b[36mCommand:\x1b[0m set_var(\"x\", \"10\")");
        set_var("x", "10");
        println!("   \x1b[32mResult:\x1b[0m  x = {}", get_var("x"));

        println!("   \x1b[36mCommand:\x1b[0m set_var(\"y\", \"5\")");
        set_var("y", "5");
        println!("   \x1b[32mResult:\x1b[0m  y = {}", get_var("y"));

        println!("   \x1b[36mCommand:\x1b[0m set_var(\"radius\", \"7.5\")");
        set_var("radius", "7.5");
        println!("   \x1b[32mResult:\x1b[0m  radius = {}", get_var("radius"));

        let expressions = vec![
            "result1 = x + y * 2",
            "result2 = (x + y) ** 2",
            "area = 3.14159 * radius ** 2",
            "temp = x * 1.8 + 32",
            "counter = 0",
            "counter += 5",
            "counter *= 3",
        ];

        println!("\n📋 Section 8.2: Expression Evaluation");
        for expr in expressions {
            println!("   \x1b[36mCommand:\x1b[0m math!(\"{}\")", expr);
            match math!(expr) {
                result => println!("   \x1b[32mResult:\x1b[0m  {} → {}", expr, result),
            }
        }

        println!("\n📋 Section 8.3: Final Variable Values");
        println!("   \x1b[36mCommand:\x1b[0m get_var(\"result1\")");
        println!("   \x1b[32mResult:\x1b[0m  result1 = {}", get_var("result1"));

        println!("   \x1b[36mCommand:\x1b[0m get_var(\"result2\")");
        println!("   \x1b[32mResult:\x1b[0m  result2 = {}", get_var("result2"));

        println!("   \x1b[36mCommand:\x1b[0m get_var(\"area\")");
        println!("   \x1b[32mResult:\x1b[0m  area = {}", get_var("area"));

        println!("   \x1b[36mCommand:\x1b[0m get_var(\"temp\")");
        println!("   \x1b[32mResult:\x1b[0m  temp = {}", get_var("temp"));

        println!("   \x1b[36mCommand:\x1b[0m get_var(\"counter\")");
        println!("   \x1b[32mResult:\x1b[0m  counter = {}", get_var("counter"));

        println!("\n✅ Expression Evaluation Demo Complete\n");
    }

    #[test]
    fn test_comprehensive_math_showcase() {
        println!("\n🎯 UAT Test 9: Comprehensive Math Showcase");
        println!("{}", "=".repeat(50));
        println!("Calculating statistics for a dataset and converting results to different formats\n");

        let scores = vec![78.5, 85.0, 91.5, 88.0, 76.5, 93.0, 82.5, 89.5, 95.0, 87.0];

        println!("📋 Section 9.1: Dataset Statistical Analysis");
        println!("   \x1b[36mData:\x1b[0m Test Scores: {:?}", scores);

        let mean_score = avg!(&scores);
        let median_score = median!(&scores);
        let min_score = min_list!(&scores);
        let max_score = max_list!(&scores);

        println!("   \x1b[36mCommand:\x1b[0m avg!(&scores)");
        println!("   \x1b[32mResult:\x1b[0m  Mean: {:.2}", mean_score);

        println!("   \x1b[36mCommand:\x1b[0m median!(&scores)");
        println!("   \x1b[32mResult:\x1b[0m  Median: {:.2}", median_score);

        println!("   \x1b[36mCommand:\x1b[0m min_list!(&scores), max_list!(&scores)");
        println!("   \x1b[32mResult:\x1b[0m  Range: {:.2} - {:.2}", min_score, max_score);

        let mean_int = mean_score as i64;
        println!("\n📋 Section 9.2: Base Conversion of Mean Score");
        println!("   \x1b[36mValue:\x1b[0m Mean score as integer: {}", mean_int);
        println!("   \x1b[36mCommand:\x1b[0m to_binary!({})", mean_int);
        println!("   \x1b[32mResult:\x1b[0m  Binary: {}", to_binary!(mean_int));

        println!("   \x1b[36mCommand:\x1b[0m to_octal!({})", mean_int);
        println!("   \x1b[32mResult:\x1b[0m  Octal: {}", to_octal!(mean_int));

        println!("   \x1b[36mCommand:\x1b[0m to_hex_upper!({})", mean_int);
        println!("   \x1b[32mResult:\x1b[0m  Hex: {}", to_hex_upper!(mean_int));

        println!("   \x1b[36mCommand:\x1b[0m even!({}), odd!({})", mean_int, mean_int);
        println!("   \x1b[32mResult:\x1b[0m  Is even: {}, Is odd: {}", even!(mean_int), odd!(mean_int));

        println!("\n📋 Section 9.3: Student Performance Analysis");
        let target_score = 90.0;
        for (i, &score) in scores.iter().enumerate() {
            if score < target_score {
                let improvement_needed = target_score - score;
                let percent_improvement = percent_change!(score, target_score);
                println!("   \x1b[36mCommand:\x1b[0m percent_change!({}, {})", score, target_score);
                println!("   \x1b[32mResult:\x1b[0m  Student {}: needs {:.1} points ({:.1}% improvement)",
                        i+1, improvement_needed, percent_improvement);
            } else {
                println!("   \x1b[32mResult:\x1b[0m  Student {}: exceeds target!", i+1);
            }
        }

        println!("\n📋 Section 9.4: Random Bonus Generation");
        println!("   \x1b[36mCommand:\x1b[0m random_list!(10, \"int\", \"1:5\")");
        let bonus_points = random_list!(10, "int", "1:5");
        println!("   \x1b[32mResult:\x1b[0m  Bonus points: {}", bonus_points);

        println!("\n✅ Comprehensive Math Showcase Complete\n");
        println!("🎯 Math module demonstration complete!");
    }
}
