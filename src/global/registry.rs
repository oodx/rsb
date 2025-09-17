//! Global registries: functions, call stack, and visual registries

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub function: String,
    pub args: Vec<String>,
    pub timestamp: std::time::SystemTime,
    pub context_snapshot: HashMap<String, String>,
}

lazy_static! {
    pub(crate) static ref FUNCTION_REGISTRY: Arc<Mutex<HashMap<String, String>>> =
        Arc::new(Mutex::new(HashMap::new()));
    pub(crate) static ref CALL_STACK: Arc<Mutex<Vec<CallFrame>>> = Arc::new(Mutex::new(Vec::new()));
    pub(crate) static ref COLORS: Arc<Mutex<HashMap<String, String>>> =
        { Arc::new(Mutex::new(initialize_default_colors())) };
    pub(crate) static ref GLYPHS: Arc<Mutex<HashMap<String, String>>> = {
        let mut m = HashMap::new();
        m.insert("info".to_string(), "ℹ".to_string());
        m.insert("okay".to_string(), "✓".to_string());
        m.insert("warn".to_string(), "⚠".to_string());
        m.insert("error".to_string(), "✗".to_string());
        m.insert("fatal".to_string(), "💀".to_string());
        m.insert("debug".to_string(), "🔍".to_string());
        m.insert("trace".to_string(), "👁".to_string());
        Arc::new(Mutex::new(m))
    };
}

#[allow(dead_code)]
fn initialize_default_colors() -> HashMap<String, String> {
    let mut colors = HashMap::new();
    colors.insert("red".to_string(), "\x1B[31m".to_string());
    colors.insert("green".to_string(), "\x1B[32m".to_string());
    colors.insert("yellow".to_string(), "\x1B[33m".to_string());
    colors.insert("blue".to_string(), "\x1B[34m".to_string());
    colors.insert("white".to_string(), "\x1B[37m".to_string());
    colors.insert("reset".to_string(), "\x1B[0m".to_string());
    colors.insert("error".to_string(), "\x1B[91m".to_string());
    colors.insert("success".to_string(), "\x1B[92m".to_string());
    colors.insert("warning".to_string(), "\x1B[93m".to_string());
    colors.insert("info".to_string(), "\x1B[94m".to_string());
    colors
}

pub fn register_function(name: &str, description: &str) {
    FUNCTION_REGISTRY
        .lock()
        .unwrap()
        .insert(name.to_string(), description.to_string());
}

pub fn list_functions() -> Vec<(String, String)> {
    let mut funcs: Vec<_> = FUNCTION_REGISTRY
        .lock()
        .unwrap()
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    funcs.sort_by(|a, b| a.0.cmp(&b.0));
    funcs
}

pub fn push_call(function: &str, args: &[String]) {
    let frame = CallFrame {
        function: function.to_string(),
        args: args.to_vec(),
        timestamp: std::time::SystemTime::now(),
        context_snapshot: crate::global::get_all_vars(),
    };
    CALL_STACK.lock().unwrap().push(frame);
}

pub fn pop_call() -> Option<CallFrame> {
    CALL_STACK.lock().unwrap().pop()
}

pub fn get_call_stack() -> Vec<CallFrame> {
    CALL_STACK.lock().unwrap().clone()
}

pub fn show_help() {
    // Header + usage
    let header = crate::global::expand_vars(&format!(
        "{{bold}}{{blue}}{}{{reset}}\n\n{{bold}}USAGE:{{reset}}\n  {} <command> [options]\n\n{{bold}}COMMANDS:{{reset}}",
        crate::global::get_var("SCRIPT_NAME"),
        crate::global::get_var("SCRIPT_NAME")
    ));
    println!("{}", crate::utils::expand_colors_unified(&header));

    // Registered functions
    for (name, desc) in list_functions() {
        let line = format!("  {{cyan}}{:<15}{{reset}} {}", name, desc);
        println!("{}", crate::utils::expand_colors_unified(&line));
    }

    // Built-in commands
    println!(
        "{}",
        crate::utils::expand_colors_unified("\n{bold}BUILT-IN COMMANDS:{reset}")
    );
    println!(
        "{}",
        crate::utils::expand_colors_unified(&format!(
            "  {{green}}{:<15}{{reset}} Show this help message",
            "help"
        ))
    );
    println!(
        "{}",
        crate::utils::expand_colors_unified(&format!(
            "  {{green}}{:<15}{{reset}} List all available functions",
            "inspect"
        ))
    );
    println!(
        "{}",
        crate::utils::expand_colors_unified(&format!(
            "  {{green}}{:<15}{{reset}} Show the current call stack",
            "stack"
        ))
    );
}

pub fn show_functions() {
    println!(
        "{}",
        crate::utils::expand_colors_unified("{bold}Available functions:{reset}")
    );
    for (name, desc) in list_functions() {
        let line = format!("  {{cyan}}{:<20}{{reset}} {}", name, desc);
        println!("{}", crate::utils::expand_colors_unified(&line));
    }
}

pub fn show_call_stack() {
    let stack = get_call_stack();
    if stack.is_empty() {
        println!("Call stack is empty");
        return;
    }
    println!(
        "{}",
        crate::utils::expand_colors_unified("{bold}Call stack (most recent first):{reset}")
    );
    for (i, frame) in stack.iter().rev().enumerate() {
        let elapsed = frame
            .timestamp
            .elapsed()
            .map(|d| format!("{}ms", d.as_millis()))
            .unwrap_or_else(|_| "?".to_string());
        let line = format!(
            "  {}: {{yellow}}{}{{reset}} {} ({{grey}}{} ago{{reset}})",
            i,
            frame.function,
            frame.args.join(" "),
            elapsed
        );
        println!("{}", crate::utils::expand_colors_unified(&line));
    }
}
