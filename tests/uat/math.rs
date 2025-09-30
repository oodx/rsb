// RSB UAT Tests - Math Module Visual Demonstrations
// Tests demonstrate the comprehensive math functionality with visual output for user acceptance
// Based on FEATURES_MATH documentation

use rsb::prelude::*;

#[test]
fn uat_math_basic_arithmetic_demo() {
    println!("\n=== UAT: Basic Arithmetic Operations ===\n");

    println!("🔢 Core Arithmetic:");
    println!("   5.0 + 3.0     = {}", rsb::math::add(5.0, 3.0));
    println!("   10.0 - 3.0    = {}", rsb::math::subtract(10.0, 3.0));
    println!("   4.0 × 5.0     = {}", rsb::math::multiply(4.0, 5.0));

    match rsb::math::divide(10.0, 2.0) {
        Ok(result) => println!("   10.0 ÷ 2.0    = {}", result),
        Err(e) => println!("   Division error: {}", e),
    }

    println!("   2^3           = {}", rsb::math::power(2.0, 3.0));
    println!(
        "   √16           = {}",
        rsb::math::sqrt(16.0).unwrap_or(-1.0)
    );
    println!("   |−5|          = {}", rsb::math::abs(-5.0));
    println!("   min(5, 3)     = {}", rsb::math::min(5.0, 3.0));
    println!("   max(5, 3)     = {}", rsb::math::max(5.0, 3.0));

    println!("\n📐 Rounding Operations:");
    let pi = 3.14159;
    println!("   π = {}", pi);
    println!("   round(π, 2)   = {}", rsb::math::round(pi, 2));
    println!("   roundup(π, 2) = {}", rsb::math::roundup(pi, 2));
    println!("   rounddown(π, 2) = {}", rsb::math::rounddown(pi, 2));
    println!("   floor(3.7)    = {}", rsb::math::floor(3.7));
    println!("   ceil(3.2)     = {}", rsb::math::ceil(3.2));

    println!("\n❌ Error Handling:");
    match rsb::math::divide(10.0, 0.0) {
        Ok(_) => println!("   Unexpected success on divide by zero!"),
        Err(e) => println!("   10.0 ÷ 0      = ERROR: {}", e),
    }

    match rsb::math::sqrt(-1.0) {
        Ok(_) => println!("   Unexpected success on negative sqrt!"),
        Err(e) => println!("   √(-1)         = ERROR: {}", e),
    }
}

#[test]
fn uat_math_integer_operations_demo() {
    println!("\n=== UAT: Integer Operations ===\n");

    println!("🔢 Integer Mathematics:");
    println!("   gcd(48, 18)   = {}", rsb::math::gcd(48, 18));
    println!("   lcm(4, 6)     = {}", rsb::math::lcm(4, 6));
    println!("   is_prime(17)  = {}", rsb::math::is_prime(17));
    println!("   is_prime(16)  = {}", rsb::math::is_prime(16));

    match rsb::math::factorial(5) {
        Ok(result) => println!("   5!            = {}", result),
        Err(e) => println!("   5! ERROR:     = {}", e),
    }

    println!("   fibonacci(10) = {}", rsb::math::fibonacci(10));

    let factors = rsb::math::factors(12);
    println!("   factors(12)   = {:?}", factors);
    println!("   sum_range(1,5)= {}", rsb::math::sum_range(1, 5));

    println!("\n🛡️ Overflow Protection:");
    match rsb::math::factorial(25) {
        Ok(result) => println!("   25! = {} (unexpected!)", result),
        Err(e) => println!("   25! → ERROR: {}", e),
    }

    println!(
        "   int_add(5, 3) = {}",
        rsb::math::int_add(5, 3).unwrap_or(-1)
    );
    println!(
        "   int_mult(7,8) = {}",
        rsb::math::int_multiply(7, 8).unwrap_or(-1)
    );
}

#[test]
fn uat_math_base_conversions_demo() {
    println!("\n=== UAT: Base Conversion Operations ===\n");

    let number = 255;
    println!("🔢 Converting {} to different bases:", number);
    println!("   Decimal:      {}", number);
    println!("   Hexadecimal:  {}", rsb::math::to_hex(number));
    println!("   Hex (upper):  {}", rsb::math::to_hex_upper(number));
    println!("   Binary:       {}", rsb::math::to_binary(number));
    println!("   Octal:        {}", rsb::math::to_octal(number));
    match rsb::math::to_base(number, 36) {
        Ok(s) => println!("   Base 36:      {}", s),
        Err(e) => println!("   Base 36:      ERROR: {}", e),
    }

    println!("\n🔄 Parsing from different bases:");
    println!(
        "   from_hex('FF')     = {}",
        rsb::math::from_hex("FF").unwrap_or(0)
    );
    println!(
        "   from_binary('1000') = {}",
        rsb::math::from_binary("1000").unwrap_or(0)
    );
    println!(
        "   from_octal('100')   = {}",
        rsb::math::from_octal("100").unwrap_or(0)
    );
    println!(
        "   from_base('2s', 36) = {}",
        rsb::math::from_base("2s", 36).unwrap_or(0)
    );

    println!("\n🚀 Direct Base Conversion:");
    let binary_to_hex = rsb::math::base_convert("11111111", 2, 16);
    println!(
        "   Binary '11111111' → Hex '{}'",
        binary_to_hex.unwrap_or("ERROR".to_string())
    );

    println!("\n💻 Programming Examples:");
    let rgb = 192; // RGB value
    println!("   RGB color {}: #{}", rgb, rsb::math::to_hex_upper(rgb));

    let permissions = rsb::math::from_octal("755").unwrap_or(0);
    println!("   Unix permissions '755' = {} (decimal)", permissions);
}

#[test]
fn uat_math_percentage_operations_demo() {
    println!("\n=== UAT: Percentage Operations ===\n");

    println!("💰 Financial Calculations:");
    let salary = 75000.0;
    let raise_percent = 8.5;
    let raise_amount = rsb::math::percent_of(salary, raise_percent);
    let new_salary = salary + raise_amount;

    println!("   Original salary:     ${:.2}", salary);
    println!("   Raise percentage:    {}%", raise_percent);
    println!("   Raise amount:        ${:.2}", raise_amount);
    println!("   New salary:          ${:.2}", new_salary);

    let change = rsb::math::percent_change(salary, new_salary);
    println!("   Percentage change:   {:.1}%", change);

    println!("\n📊 Ratio Calculations:");
    let ratio = rsb::math::ratio(16.0, 9.0);
    match ratio {
        Ok(r) => println!("   16:9 aspect ratio    = {:.3}", r),
        Err(e) => println!("   16:9 aspect ratio    = ERROR: {}", e),
    }

    let ratio2 = rsb::math::ratio(4.0, 3.0);
    match ratio2 {
        Ok(r) => println!("   4:3 aspect ratio     = {:.3}", r),
        Err(e) => println!("   4:3 aspect ratio     = ERROR: {}", e),
    }

    println!("\n🔄 Percentage Conversions:");
    println!(
        "   50% to decimal       = {}",
        rsb::math::percentage_to_decimal(50.0)
    );
    println!(
        "   0.25 to percentage   = {}%",
        rsb::math::decimal_to_percentage(0.25)
    );
    println!(
        "   0.125 to percentage  = {}%",
        rsb::math::decimal_to_percentage(0.125)
    );

    println!("\n🛒 Shopping Examples:");
    let price = 120.0;
    let discount = 25.0;
    let discount_amount = rsb::math::percent_of(price, discount);
    let final_price = price - discount_amount;
    println!("   Original price:      ${:.2}", price);
    println!("   Discount ({}%):      ${:.2}", discount, discount_amount);
    println!("   Final price:         ${:.2}", final_price);
}

#[test]
fn uat_math_predicates_demo() {
    println!("\n=== UAT: Mathematical Predicates ===\n");

    println!("🔢 Number Properties:");
    let numbers = [8, 7, 0, -5, 42];
    for num in numbers {
        println!(
            "   {} → even: {}, odd: {}, sign: {}",
            num,
            rsb::math::is_even(num),
            rsb::math::is_odd(num),
            rsb::math::sign(num as f64) as i32
        );
    }

    println!("\n🧮 Modulo Operations:");
    println!(
        "   10 mod 3          = {}",
        rsb::math::modulo(10, 3).unwrap_or(0)
    );
    println!(
        "   15 mod 4          = {}",
        rsb::math::modulo(15, 4).unwrap_or(0)
    );
    println!(
        "   7 mod 2           = {}",
        rsb::math::modulo(7, 2).unwrap_or(0)
    );

    println!("\n➕➖ Sign Comparisons:");
    let pairs = [(5.0, 3.0), (-5.0, 3.0), (-2.0, -8.0), (0.0, 0.0)];
    for (a, b) in pairs {
        println!(
            "   same_sign({}, {}) = {}",
            a,
            b,
            rsb::math::same_sign(a, b)
        );
    }

    println!("\n🎯 Sign Tests:");
    let test_vals = [5.0, -3.0, 0.0, 42.7, -0.0];
    for val in test_vals {
        println!(
            "   {} → positive: {}, negative: {}, zero: {}",
            val,
            rsb::math::is_positive(val),
            rsb::math::is_negative(val),
            rsb::math::is_zero(val)
        );
    }
}

#[test]
fn uat_math_aggregation_demo() {
    println!("\n=== UAT: List Aggregation Operations ===\n");

    let test_scores = vec![85.5, 92.0, 78.5, 96.5, 88.0, 91.5, 84.0];

    println!("📊 Statistical Analysis:");
    println!("   Test scores: {:?}", test_scores);
    println!("   Count:       {}", test_scores.len());
    println!("   Sum:         {:.1}", rsb::math::sum_list(&test_scores));
    println!(
        "   Average:     {:.2}",
        rsb::math::avg(&test_scores).unwrap_or(0.0)
    );
    println!(
        "   Mean:        {:.2}",
        rsb::math::mean(&test_scores).unwrap_or(0.0)
    );
    println!(
        "   Median:      {:.2}",
        rsb::math::median(&test_scores).unwrap_or(0.0)
    );
    println!(
        "   Minimum:     {:.1}",
        rsb::math::min_list(&test_scores).unwrap_or(0.0)
    );
    println!(
        "   Maximum:     {:.1}",
        rsb::math::max_list(&test_scores).unwrap_or(0.0)
    );
    println!(
        "   Range:       {:.1}",
        rsb::math::max_list(&test_scores).unwrap_or(0.0)
            - rsb::math::min_list(&test_scores).unwrap_or(0.0)
    );

    println!("\n💰 Financial Data Example:");
    let daily_sales = vec![1250.0, 980.0, 1450.0, 1100.0, 1320.0, 1875.0, 1650.0];
    println!("   Daily sales: {:?}", daily_sales);
    println!("   Weekly total: ${:.2}", rsb::math::sum_list(&daily_sales));
    println!(
        "   Daily average: ${:.2}",
        rsb::math::avg(&daily_sales).unwrap_or(0.0)
    );
    println!(
        "   Best day: ${:.2}",
        rsb::math::max_list(&daily_sales).unwrap_or(0.0)
    );
    println!(
        "   Worst day: ${:.2}",
        rsb::math::min_list(&daily_sales).unwrap_or(0.0)
    );

    println!("\n❌ Empty List Handling:");
    let empty: Vec<f64> = vec![];
    match rsb::math::avg(&empty) {
        Some(_) => println!("   Unexpected success on empty list!"),
        None => println!("   Empty list average → ERROR (properly handled)"),
    }
}

#[test]
fn uat_math_random_operations_demo() {
    println!("\n=== UAT: Random Number Generation ===\n");

    println!("🎲 Random Number Examples:");
    for i in 1..=5 {
        let random_float = rsb::gx::rand::random_range(1.0, 10.0);
        let random_int = rsb::gx::rand::random_int_range(1, 100);
        println!(
            "   Sample {}: float={:.3}, int={}",
            i, random_float, random_int
        );
    }

    println!("\n📊 Random List Generation:");
    let float_list = rsb::gx::rand::random_list_float(5, 0.0, 1.0);
    println!("   5 floats (0-1): {:?}", float_list);

    let int_list = rsb::gx::rand::random_list_int(5, 1, 10);
    println!("   5 ints (1-10):  {:?}", int_list);

    let bool_list = rsb::gx::rand::random_list_bool(8);
    println!("   8 booleans:     {:?}", bool_list);

    println!("\n🎯 String-Based Random Generation (RSB Style):");
    let random_ints = rsb::gx::rand::random_list_string("int", 5, Some("1:100"));
    match random_ints {
        Ok(s) => println!("   Random ints:    '{}'", s),
        Err(e) => println!("   Random ints:    ERROR: {}", e),
    }

    let random_floats = rsb::gx::rand::random_list_string("float", 4, Some("0.0:1.0"));
    match random_floats {
        Ok(s) => println!("   Random floats:  '{}'", s),
        Err(e) => println!("   Random floats:  ERROR: {}", e),
    }

    let random_bools = rsb::gx::rand::random_list_string("bool", 6, None);
    match random_bools {
        Ok(s) => println!("   Random bools:   '{}'", s),
        Err(e) => println!("   Random bools:   ERROR: {}", e),
    }

    println!("\n🎮 Gaming/Simulation Applications:");
    println!("   Dice roll (1-6): {}", rsb::gx::rand::random_int_range(1, 6));
    println!(
        "   Coin flip:       {}",
        if rsb::gx::rand::random_range(0.0, 1.0) > 0.5 {
            "Heads"
        } else {
            "Tails"
        }
    );

    let damage_roll = rsb::gx::rand::random_list_string("int", 3, Some("1:6"));
    match damage_roll {
        Ok(s) => println!("   3d6 damage:      {} (sum would need parsing)", s),
        Err(e) => println!("   3d6 damage:      ERROR: {}", e),
    }
}

#[test]
fn uat_math_string_based_calculations_demo() {
    println!("\n=== UAT: String-Based Calculations (RSB Style) ===\n");

    println!("🧮 Basic Calculator Functions:");
    println!(
        "   calc('add', '5', '3')      = '{}'",
        rsb::math::calc("add", "5", "3")
    );
    println!(
        "   calc('subtract', '10', '3') = '{}'",
        rsb::math::calc("subtract", "10", "3")
    );
    println!(
        "   calc('multiply', '4', '5')  = '{}'",
        rsb::math::calc("multiply", "4", "5")
    );
    println!(
        "   calc('divide', '10', '2')   = '{}'",
        rsb::math::calc("divide", "10", "2")
    );
    println!(
        "   calc('power', '2', '8')     = '{}'",
        rsb::math::calc("power", "2", "8")
    );

    println!("\n🔢 Integer Calculator:");
    println!(
        "   int_calc('add', '42', '58')    = '{}'",
        rsb::math::int_calc("add", "42", "58")
    );
    println!(
        "   int_calc('gcd', '48', '18')    = '{}'",
        rsb::math::int_calc("gcd", "48", "18")
    );
    println!(
        "   int_calc('multiply', '7', '8') = '{}'",
        rsb::math::int_calc("multiply", "7", "8")
    );
    println!(
        "   int_calc('factorial', '5', '') = '{}'",
        rsb::math::int_calc("factorial", "5", "")
    );

    println!("\n❌ Error Handling:");
    println!(
        "   calc('invalid', '5', '3')   = '{}'",
        rsb::math::calc("invalid", "5", "3")
    );
    println!(
        "   calc('divide', '5', '0')    = '{}'",
        rsb::math::calc("divide", "5", "0")
    );
    println!(
        "   int_calc('parse', 'abc', '') = '{}'",
        rsb::math::int_calc("parse", "abc", "")
    );

    println!("\n🔧 Number Parsing:");
    match rsb::math::parse_number("42.5") {
        Ok(val) => println!("   parse_number('42.5')        = {}", val),
        Err(e) => println!("   parse_number('42.5')        = ERROR: {}", e),
    }

    match rsb::math::parse_number("invalid") {
        Ok(val) => println!("   parse_number('invalid')     = {} (unexpected!)", val),
        Err(e) => println!("   parse_number('invalid')     = ERROR: {}", e),
    }
}

#[test]
fn uat_math_expression_evaluation_demo() {
    println!("\n=== UAT: Expression Evaluation (Advanced) ===\n");

    println!("🧮 Mathematical Expression Parser:");

    // Set up some variables for expressions
    set_var("radius", "7.5");
    set_var("pi", "3.14159");
    set_var("x", "10");
    set_var("y", "5");

    println!("   Variables set:");
    println!("     radius = {}", get_var("radius"));
    println!("     pi = {}", get_var("pi"));
    println!("     x = {}", get_var("x"));
    println!("     y = {}", get_var("y"));

    println!("\n🎯 Expression Examples:");
    let expressions = vec!["x + y", "x * y", "pi * radius", "x + y * 2", "(x + y) * 2"];

    for expr in expressions {
        let result = rsb::math::evaluate_expression(expr);
        match result {
            Ok(val) => println!("   '{}' = {}", expr, val),
            Err(e) => println!("   '{}' = ERROR: {}", expr, e),
        }
    }

    println!("\n🔬 Advanced Features (if supported):");
    let advanced_expressions = vec![
        "area = pi * radius ** 2",
        "counter = x + 10",
        "result = x ** 2 + y ** 2",
    ];

    for expr in advanced_expressions {
        let result = rsb::math::evaluate_expression(expr);
        match result {
            Ok(val) => println!("   '{}' = {}", expr, val),
            Err(e) => println!("   '{}' = ERROR: {}", expr, e),
        }
    }

    println!("\n✅ Math module provides comprehensive mathematical functionality!");
    println!("   → Basic arithmetic with error handling");
    println!("   → Integer operations with overflow protection");
    println!("   → Base conversions for programming tasks");
    println!("   → Statistical aggregations for data analysis");
    println!("   → Random number generation for simulations");
    println!("   → String-first API design for RSB integration");
    println!("   → Expression evaluation for complex calculations");
}
