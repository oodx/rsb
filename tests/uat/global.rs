use rsb::prelude::*;

#[test]
fn uat_global_demo() {
    // Visible demo of global store and expansion
    rsb::global::set_var("PROJECT", "rsb");
    rsb::global::set_var("ENV", "dev");
    let msg = rsb::global::expand_vars("Project $PROJECT in ${ENV} mode");

    // Plain visible output (no features required)
    println!("\n=== UAT: Global Demo ===");
    println!("SET PROJECT=rsb");
    println!("SET ENV=dev");
    println!("EXPAND → {}\n", msg);

    // Optional visual styling when feature is enabled
    #[cfg(feature = "visual")]
    {
        // Show colored/glyph stderr messages without affecting stdout data
        rsb::info!("Global UAT (info): {}", msg);
        rsb::okay!("Global UAT (okay): {}", msg);
    }

    // Store basics
    println!("HAS PROJECT? {}", rsb::global::has_var("PROJECT"));
    println!("HAS MISSING? {}", rsb::global::has_var("MISSING"));
    rsb::global::unset_var("PROJECT");
    println!("UNSET PROJECT → HAS? {}", rsb::global::has_var("PROJECT"));
    rsb::global::set_var("PROJECT", "rsb");

    // Expand variables
    rsb::global::set_var("HOME", "/home/test");
    let path1 = rsb::global::expand_vars("$HOME/docs");
    let path2 = rsb::global::expand_vars("Path: ${HOME}/docs");
    println!("EXPAND1 → {}", path1);
    println!("EXPAND2 → {}", path2);

    // Boolean helpers
    rsb::global::set_var("opt_quiet", "true");
    rsb::global::set_var("opt_trace", "false");
    println!("is_true(opt_quiet) → {}", rsb::global::is_true("opt_quiet"));
    println!("is_false(opt_trace) → {}", rsb::global::is_false("opt_trace"));

    // Token stream check
    println!("is_token_stream('a=1,b=2') → {}", rsb::global::is_token_stream("a=1,b=2"));
    println!("is_token_stream('=bad') → {}", rsb::global::is_token_stream("=bad"));

    // Config parsing (content)
    let cfg = r#"
KEY=VALUE
KEY_WITH_SPACES="value with spaces"
ARRAY=(item1 item2 "item 3")
"#;
    rsb::global::parse_config_content(cfg);
    println!("CONFIG KEY → {}", rsb::global::get_var("KEY"));
    println!("CONFIG KEY_WITH_SPACES → {}", rsb::global::get_var("KEY_WITH_SPACES"));
    println!("CONFIG ARRAY → {}", rsb::global::get_var("ARRAY"));
    println!("CONFIG ARRAY_LENGTH → {}", rsb::global::get_var("ARRAY_LENGTH"));
    println!("CONFIG ARRAY_2 → {}", rsb::global::get_var("ARRAY_2"));

    // Config I/O (save → load)
    let tmp_dir = std::env::temp_dir();
    let cfg_path = tmp_dir.join("rsb_global_uat.conf");
    rsb::global::set_var("CONF_A", "alpha");
    rsb::global::set_var("CONF_PATH", "/tmp/demo path");
    rsb::global::save_config_file(cfg_path.to_str().unwrap(), &["CONF_A", "CONF_PATH"]);
    println!("SAVED CONFIG → {}", cfg_path.display());
    // Clear and reload
    rsb::global::unset_var("CONF_A");
    rsb::global::unset_var("CONF_PATH");
    rsb::global::load_config_file(cfg_path.to_str().unwrap());
    println!("LOADED CONF_A → {}", rsb::global::get_var("CONF_A"));
    println!("LOADED CONF_PATH → {}", rsb::global::get_var("CONF_PATH"));

    // Export vars
    let export_path = tmp_dir.join("rsb_global_uat.env");
    rsb::global::export_vars(export_path.to_str().unwrap());
    println!("EXPORTED VARS → {}", export_path.display());

    // Function registry and call stack (visible)
    rsb::global::register_function("demo", "Runs the uat demo");
    let funcs = rsb::global::list_functions();
    println!("FUNCTIONS REGISTERED → {}", funcs.len());
    for (name, desc) in funcs { println!("  - {} : {}", name, desc); }
    // Push/pop one frame
    rsb::global::push_call("demo", &["--flag".into()]);
    rsb::global::show_call_stack();
    let _ = rsb::global::pop_call();

    assert_eq!(msg, "Project rsb in dev mode");
}
