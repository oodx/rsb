//! Feature tests for RSB dispatch system
//!
//! Tests command dispatch, pre_dispatch, argument forwarding,
//! and error handling for unknown commands.

use rsb::cli::{execute_pre_dispatch, CommandHandler};
use rsb::prelude::*;

#[cfg(test)]
mod dispatch_tests {
    use super::*;

    // Example command handlers for testing
    fn cmd_hello(args: Args) -> i32 {
        let name = args.get_or(2, "World");
        println!("Hello, {}!", name);
        0
    }

    fn cmd_add(args: Args) -> i32 {
        let a: i32 = args.get_or(2, "0").parse().unwrap_or(0);
        let b: i32 = args.get_or(3, "0").parse().unwrap_or(0);
        println!("{}", a + b);
        0
    }

    fn cmd_args_test(args: Args) -> i32 {
        println!("Command args: {:?}", args.all());
        println!("Arg count: {}", args.len());
        for (i, arg) in args.all().iter().enumerate() {
            println!("  [{}]: {}", i, arg);
        }
        0
    }

    #[test]
    fn test_pre_dispatch_found_command() {
        // Test pre_dispatch with valid command
        let args = Args::new(&["myapp".to_string(), "hello".to_string(), "RSB".to_string()]);

        let result = execute_pre_dispatch(&args, |command| match command {
            "hello" => Some(cmd_hello),
            "add" => Some(cmd_add),
            _ => None,
        });

        assert!(result, "pre_dispatch should return true for found command");
    }

    #[test]
    fn test_pre_dispatch_not_found_command() {
        // Test pre_dispatch with invalid command
        let args = Args::new(&["myapp".to_string(), "unknown".to_string()]);

        let result = execute_pre_dispatch(&args, |command| match command {
            "hello" => Some(cmd_hello),
            "add" => Some(cmd_add),
            _ => None,
        });

        assert!(
            !result,
            "pre_dispatch should return false for unknown command"
        );
    }

    #[test]
    fn test_arg_forwarding() {
        // Test that arguments are properly forwarded to handlers
        let args = Args::new(&[
            "myapp".to_string(),
            "args-test".to_string(),
            "first".to_string(),
            "second".to_string(),
            "third".to_string(),
        ]);

        let result = execute_pre_dispatch(&args, |command| match command {
            "args-test" => Some(cmd_args_test),
            _ => None,
        });

        assert!(result, "pre_dispatch should handle args-test command");
    }

    #[test]
    fn test_empty_command() {
        // Test behavior with empty command
        let args = Args::new(&["myapp".to_string()]);

        let result = execute_pre_dispatch(&args, |command| {
            match command {
                "hello" => Some(cmd_hello),
                "" => None, // Empty command should not match
                _ => None,
            }
        });

        assert!(
            !result,
            "pre_dispatch should return false for empty command"
        );
    }

    #[test]
    fn test_command_extraction() {
        // Test various argument patterns
        let test_cases = vec![
            (vec!["app"], ""),
            (vec!["app", "hello"], "hello"),
            (vec!["app", "hello", "world"], "hello"),
            (vec!["app", "--help"], "--help"),
        ];

        for (args_vec, expected_cmd) in test_cases {
            let args_strings: Vec<String> = args_vec.into_iter().map(String::from).collect();
            let args = Args::new(&args_strings);
            let command = args.get_or(1, "");
            assert_eq!(
                command, expected_cmd,
                "Command extraction failed for: {:?}",
                args_strings
            );
        }
    }

    #[test]
    fn test_multiple_commands() {
        // Test multiple commands with same handler lookup
        let test_commands = vec!["hello", "add"];

        for cmd in test_commands {
            let args = Args::new(&["myapp".to_string(), cmd.to_string()]);

            let result = execute_pre_dispatch(&args, |command| match command {
                "hello" => Some(cmd_hello),
                "add" => Some(cmd_add),
                _ => None,
            });

            assert!(result, "Command '{}' should be found", cmd);
        }
    }
}

// Integration tests with actual macro usage patterns
#[cfg(test)]
mod dispatch_integration_tests {
    use super::*;

    // These tests demonstrate real dispatch usage patterns

    fn hello_handler(args: Args) -> i32 {
        println!("Hello from handler! Args: {:?}", args.all());
        0
    }

    fn version_handler(_args: Args) -> i32 {
        println!("Version 1.0.0");
        0
    }

    #[test]
    fn test_dispatch_pattern() {
        // Test typical dispatch usage
        std::env::set_var("CARGO_TEST", "1");

        let args = Args::new(&[
            "myapp".to_string(),
            "hello".to_string(),
            "world".to_string(),
        ]);

        let result = execute_pre_dispatch(&args, |command| match command {
            "hello" => Some(hello_handler),
            "version" => Some(version_handler),
            _ => None,
        });

        assert!(result);
    }

    #[test]
    fn test_version_command() {
        std::env::set_var("CARGO_TEST", "1");

        let args = Args::new(&["myapp".to_string(), "version".to_string()]);

        let result = execute_pre_dispatch(&args, |command| match command {
            "hello" => Some(hello_handler),
            "version" => Some(version_handler),
            _ => None,
        });

        assert!(result);
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_unknown_command_behavior() {
        let args = Args::new(&["myapp".to_string(), "nonexistent".to_string()]);

        let result = execute_pre_dispatch(&args, |_command| None);

        assert!(
            !result,
            "Unknown commands should return false from pre_dispatch"
        );
    }
}
