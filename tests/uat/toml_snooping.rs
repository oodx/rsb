//! User Acceptance Tests for TOML Snooping with visual demonstrations

use rsb::toml::*;
use rsb::global::get_var;
use std::fs;
use std::env;

// Helper to create test Cargo.toml in temp directory
fn setup_test_toml(content: &str) -> std::path::PathBuf {
    let temp_dir = env::temp_dir().join(format!("rsb_uat_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).unwrap();
    let cargo_path = temp_dir.join("Cargo.toml");
    fs::write(&cargo_path, content).unwrap();
    temp_dir
}

fn cleanup_test_dir(dir: &std::path::Path) {
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn uat_toml_snooping_basic_demo() {
    println!("\n=== TOML Snooping Basic Demo ===\n");

    let test_toml = r#"
[package]
name = "demo-app"
version = "1.0.0"

[package.metadata.hub]
api_url = "https://api.hub.example.com"
timeout = "30"
max_connections = "100"

[package.metadata.inf]
team_name = "RSB Core"
support_email = "support@rsb.dev"

[package.metadata.rsb]
options_mode = "remove"
debug = true
"#;

    let test_dir = setup_test_toml(test_toml);
    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(&test_dir).unwrap();

    println!("Cargo.toml content:");
    println!("{}", test_toml);

    println!("\nEnabling TOML snooping...");
    enable_toml_snooping();

    println!("\n✓ TOML snooping enabled!\n");

    println!("Hub configuration (hub_*):");
    println!("  hub_api_url = {}", get_var("hub_api_url"));
    println!("  hub_timeout = {}", get_var("hub_timeout"));
    println!("  hub_max_connections = {}", get_var("hub_max_connections"));

    println!("\nInfrastructure configuration (inf_*):");
    println!("  inf_team_name = {}", get_var("inf_team_name"));
    println!("  inf_support_email = {}", get_var("inf_support_email"));

    println!("\nRSB configuration (rsb_*):");
    println!("  rsb_options_mode = {}", get_var("rsb_options_mode"));
    println!("  rsb_debug = {}", get_var("rsb_debug"));

    println!("\n✨ All metadata sections extracted and stored in global variables!");

    env::set_current_dir(original_dir).unwrap();
    cleanup_test_dir(&test_dir);
}

#[test]
fn uat_toml_snake_case_conversion_demo() {
    println!("\n=== Snake Case Conversion Demo ===\n");

    let test_toml = r#"
[package]
name = "demo-app"
version = "1.0.0"

[package.metadata.hub]
apiUrl = "https://api.test"
maxRetries = "5"
connectTimeout = "10"
"#;

    let test_dir = setup_test_toml(test_toml);
    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(&test_dir).unwrap();

    println!("Original TOML keys (camelCase):");
    println!("  apiUrl");
    println!("  maxRetries");
    println!("  connectTimeout");

    println!("\nEnabling TOML snooping with snake_case conversion...");
    enable_toml_snooping();

    println!("\nConverted keys (snake_case):");
    println!("  hub_api_url = {}", get_var("hub_api_url"));
    println!("  hub_max_retries = {}", get_var("hub_max_retries"));
    println!("  hub_connect_timeout = {}", get_var("hub_connect_timeout"));

    println!("\n✨ All keys automatically converted to snake_case!");

    env::set_current_dir(original_dir).unwrap();
    cleanup_test_dir(&test_dir);
}

#[test]
fn uat_toml_array_storage_demo() {
    println!("\n=== Array Storage Demo ===\n");

    let test_toml = r#"
[package]
name = "demo-app"
version = "1.0.0"

[package.metadata.hub]
features = ["auth", "cache", "metrics", "logging"]
regions = ["us-east", "us-west", "eu-central"]
"#;

    let test_dir = setup_test_toml(test_toml);
    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(&test_dir).unwrap();

    println!("TOML arrays:");
    println!("  features = [\"auth\", \"cache\", \"metrics\", \"logging\"]");
    println!("  regions = [\"us-east\", \"us-west\", \"eu-central\"]");

    println!("\nEnabling TOML snooping with array handling...");
    enable_toml_snooping();

    println!("\nFeatures array (RSB indexed format):");
    let features_len = get_var("hub_features_LENGTH");
    println!("  hub_features_LENGTH = {}", features_len);
    for i in 0..features_len.parse::<usize>().unwrap_or(0) {
        println!("  hub_features_{} = {}", i, get_var(&format!("hub_features_{}", i)));
    }

    println!("\nRegions array (RSB indexed format):");
    let regions_len = get_var("hub_regions_LENGTH");
    println!("  hub_regions_LENGTH = {}", regions_len);
    for i in 0..regions_len.parse::<usize>().unwrap_or(0) {
        println!("  hub_regions_{} = {}", i, get_var(&format!("hub_regions_{}", i)));
    }

    println!("\n✨ Arrays stored with LENGTH + indexed values (RSB convention)!");

    env::set_current_dir(original_dir).unwrap();
    cleanup_test_dir(&test_dir);
}

#[test]
fn uat_toml_value_types_demo() {
    println!("\n=== Value Types Demo ===\n");

    let test_toml = r#"
[package]
name = "demo-app"
version = "1.0.0"

[package.metadata.rsb]
app_name = "My Application"
port = 8080
enabled = true
threshold = 3.14
"#;

    let test_dir = setup_test_toml(test_toml);
    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(&test_dir).unwrap();

    println!("TOML values with different types:");
    println!("  app_name = \"My Application\" (String)");
    println!("  port = 8080 (Integer)");
    println!("  enabled = true (Boolean)");
    println!("  threshold = 3.14 (Float)");

    println!("\nEnabling TOML snooping...");
    enable_toml_snooping();

    println!("\nAll values stored as strings:");
    println!("  rsb_app_name = \"{}\" (String → String)", get_var("rsb_app_name"));
    println!("  rsb_port = \"{}\" (Integer → String)", get_var("rsb_port"));
    println!("  rsb_enabled = \"{}\" (Boolean → String)", get_var("rsb_enabled"));
    println!("  rsb_threshold = \"{}\" (Float → String)", get_var("rsb_threshold"));

    println!("\n✨ All values stored as strings (RSB string-biased philosophy)!");

    env::set_current_dir(original_dir).unwrap();
    cleanup_test_dir(&test_dir);
}

#[test]
fn uat_toml_custom_namespace_demo() {
    println!("\n=== Custom Namespace Demo ===\n");

    let test_toml = r#"
[package]
name = "demo-app"
version = "1.0.0"

[package.metadata.myapp]
custom_setting = "custom_value"
api_key = "secret123"
"#;

    let test_dir = setup_test_toml(test_toml);
    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(&test_dir).unwrap();

    println!("Custom TOML namespace:");
    println!("  [package.metadata.myapp]");
    println!("  custom_setting = \"custom_value\"");
    println!("  api_key = \"secret123\"");

    println!("\nAdding custom namespace 'myapp'...");
    snoop_namespace("myapp");

    println!("Enabling TOML snooping...");
    enable_toml_snooping();

    println!("\nCustom namespace values:");
    println!("  myapp_custom_setting = {}", get_var("myapp_custom_setting"));
    println!("  myapp_api_key = {}", get_var("myapp_api_key"));

    println!("\n✨ Custom namespaces can be added and snooped!");

    env::set_current_dir(original_dir).unwrap();
    cleanup_test_dir(&test_dir);
}

#[test]
fn uat_toml_bootstrap_integration_demo() {
    println!("\n=== Bootstrap Integration Demo ===\n");

    let test_toml = r#"
[package]
name = "demo-app"
version = "1.0.0"

[package.metadata.hub]
api_url = "https://api.hub.example.com"

[package.metadata.rsb]
options_mode = "remove"
"#;

    let test_dir = setup_test_toml(test_toml);
    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(&test_dir).unwrap();

    println!("Bootstrap macro variants:\n");

    println!("1. bootstrap!() - Default (no TOML snooping)");
    println!("   → Use when you don't need Cargo.toml metadata\n");

    println!("2. bootstrap!(toml) - With TOML snooping");
    println!("   → Automatically extracts rsb, hub, inf namespaces");
    println!("   → Example: let args = bootstrap!(toml);\n");

    println!("3. bootstrap!(toml: \"custom\") - With custom namespaces");
    println!("   → Extracts specified namespaces + defaults");
    println!("   → Example: let args = bootstrap!(toml: \"myapp\", \"config\");\n");

    enable_toml_snooping();

    println!("After bootstrap!(toml), global variables available:");
    println!("  hub_api_url = {}", get_var("hub_api_url"));
    println!("  rsb_options_mode = {}", get_var("rsb_options_mode"));

    println!("\n✨ Bootstrap macro seamlessly integrates TOML snooping!");

    env::set_current_dir(original_dir).unwrap();
    cleanup_test_dir(&test_dir);
}

#[cfg(feature = "object")]
#[test]
fn uat_toml_object_integration_demo() {
    use rsb::object::*;

    println!("\n=== Object Integration Demo ===\n");

    let test_toml = r#"
[package]
name = "demo-app"
version = "1.0.0"

[package.metadata.hub]
api_url = "https://api.hub.example.com"
timeout = "30"
retries = "3"
"#;

    let test_dir = setup_test_toml(test_toml);
    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(&test_dir).unwrap();

    println!("Enabling TOML snooping...");
    enable_toml_snooping();

    println!("\nCreating Object from snooped globals...");
    let hub_config = Object::<HubShape>::from_global("hub");

    println!("\nAccessing via Object:");
    println!("  hub_config[\"api_url\"] = {}", &hub_config["api_url"]);
    println!("  hub_config[\"timeout\"] = {}", &hub_config["timeout"]);
    println!("  hub_config[\"retries\"] = {}", &hub_config["retries"]);

    println!("\nIterating over config:");
    for (key, value) in hub_config.as_map() {
        println!("  {} = {}", key, value);
    }

    println!("\n✨ TOML snooping integrates seamlessly with Object<T> system!");

    env::set_current_dir(original_dir).unwrap();
    cleanup_test_dir(&test_dir);
}