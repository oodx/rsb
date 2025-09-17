use rsb::prelude::*;

#[test]
fn sanity_global_basic_store() {
    // Test basic variable storage and retrieval
    set_var("TEST_KEY", "test_value");
    assert_eq!(get_var("TEST_KEY"), "test_value");
    assert!(has_var("TEST_KEY"));

    // Test empty key
    assert_eq!(get_var("NONEXISTENT"), "");
    assert!(!has_var("NONEXISTENT"));

    // Test unset
    unset_var("TEST_KEY");
    assert!(!has_var("TEST_KEY"));
    assert_eq!(get_var("TEST_KEY"), "");
}

#[test]
fn sanity_global_variable_expansion() {
    // Test variable expansion
    set_var("NAME", "RSB");
    set_var("VERSION", "1.0");

    let expanded = expand_vars("$NAME version $VERSION");
    assert!(expanded.contains("RSB"));
    assert!(expanded.contains("1.0"));

    let expanded_braces = expand_vars("${NAME} v${VERSION}");
    assert!(expanded_braces.contains("RSB"));
    assert!(expanded_braces.contains("1.0"));

    // Test missing variable expansion
    let with_missing = expand_vars("$MISSING_VAR");
    assert_eq!(with_missing, "");
}

#[test]
fn sanity_global_boolean_operations() {
    // Test boolean semantics
    set_var("TRUE_FLAG", "1");
    set_var("FALSE_FLAG", "0");
    set_var("OTHER_VALUE", "something");

    assert!(is_true("TRUE_FLAG"));
    assert!(!is_false("TRUE_FLAG"));

    assert!(is_false("FALSE_FLAG"));
    assert!(!is_true("FALSE_FLAG"));

    // Non-boolean values
    assert!(!is_true("OTHER_VALUE"));
    assert!(!is_false("OTHER_VALUE"));

    // Missing values
    assert!(!is_true("MISSING"));
    assert!(!is_false("MISSING"));
}

#[test]
fn sanity_global_config_parsing() {
    // Test config content parsing
    let config_content = r#"
KEY1=value1
KEY2="quoted value"
KEY3='single quoted'
# Comment line
ARRAY=(item1 item2 "item 3")
"#;

    parse_config_content(config_content);

    assert_eq!(get_var("KEY1"), "value1");
    assert_eq!(get_var("KEY2"), "quoted value");
    assert_eq!(get_var("KEY3"), "single quoted");

    // Test array parsing
    assert!(has_var("ARRAY"));
    assert!(has_var("ARRAY_LENGTH"));
    assert!(has_var("ARRAY_0"));
    assert!(has_var("ARRAY_1"));
    assert!(has_var("ARRAY_2"));
}

#[test]
fn sanity_global_function_registry() {
    // Test function registration and listing
    register_function("test_func", "Test function description");

    let functions = list_functions();
    let test_func_found = functions.iter().any(|(name, desc)| {
        name == "test_func" && desc == "Test function description"
    });
    assert!(test_func_found);
}

#[test]
fn sanity_global_call_stack() {
    // Test call stack operations
    let args = vec!["arg1".to_string(), "arg2".to_string()];

    push_call("test_function", &args);

    let stack = get_call_stack();
    assert!(!stack.is_empty());

    let popped = pop_call();
    assert!(popped.is_some());

    let frame = popped.unwrap();
    assert_eq!(frame.function, "test_function");
    assert_eq!(frame.args, args);
}

#[test]
fn sanity_global_get_all_vars() {
    // Test getting all variables
    set_var("TEST_A", "value_a");
    set_var("TEST_B", "value_b");

    let all_vars = get_all_vars();
    assert!(all_vars.contains_key("TEST_A"));
    assert!(all_vars.contains_key("TEST_B"));
    assert_eq!(all_vars.get("TEST_A"), Some(&"value_a".to_string()));
    assert_eq!(all_vars.get("TEST_B"), Some(&"value_b".to_string()));

    // Cleanup
    unset_var("TEST_A");
    unset_var("TEST_B");
}