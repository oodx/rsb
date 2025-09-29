//! User Acceptance Tests for Object module with visual demonstrations

#[cfg(feature = "object")]
mod object_uat {
    use rsb::object::*;
    use rsb::global::set_var;

    #[test]
    fn uat_object_basic_demo() {
        println!("\n=== Object Creation and Basic Operations Demo ===\n");

        let mut obj = Object::<()>::new("demo");
        println!("Created new Object with namespace: {}", obj.namespace());

        obj.set("name", "RSB Object");
        obj.set("version", "2.0");
        obj.set("status", "active");

        println!("\nObject contents:");
        for (key, value) in obj.as_map() {
            println!("  {} = {}", key, value);
        }

        println!("\nAccessing values:");
        println!("  obj[\"name\"] = {}", &obj["name"]);
        println!("  obj.get(\"version\") = {}", obj.get("version"));
        println!("  obj.get_or(\"missing\", \"default\") = {}", obj.get_or("missing", "default"));
    }

    #[test]
    fn uat_object_key_normalization_demo() {
        println!("\n=== Key Normalization Demo ===\n");

        let mut obj = Object::<()>::new("normalize");

        // Set with various key formats
        obj.set("snake_case", "snake");
        obj.set("dot.notation", "dots");
        obj.set("kebab-case", "kebab");
        obj.set("CamelCase", "camel");

        println!("Setting keys with different formats:");
        println!("  snake_case -> \"snake\"");
        println!("  dot.notation -> \"dots\"");
        println!("  kebab-case -> \"kebab\"");
        println!("  CamelCase -> \"camel\"");

        println!("\nAccessing with normalized keys:");
        println!("  obj[\"snake_case\"] = {}", &obj["snake_case"]);
        println!("  obj[\"dot_notation\"] = {}", &obj["dot_notation"]);
        println!("  obj[\"kebab_case\"] = {}", &obj["kebab_case"]);
        println!("  obj[\"camelcase\"] = {}", &obj["camelcase"]);

        println!("\nAll keys are normalized to snake_case!");
    }

    #[test]
    fn uat_object_global_integration_demo() {
        println!("\n=== Global Store Integration Demo ===\n");

        // Set up some global variables
        set_var("demo_api_url", "https://api.example.com");
        set_var("demo_timeout", "30");
        set_var("demo_retries", "3");

        println!("Global variables set:");
        println!("  demo_api_url = \"https://api.example.com\"");
        println!("  demo_timeout = \"30\"");
        println!("  demo_retries = \"3\"");

        // Load from global
        let config = Object::<()>::from_global("demo");
        println!("\nLoaded Object from global with namespace \"demo\"");

        println!("\nObject contents (stripped prefix):");
        for (key, value) in config.as_map() {
            println!("  {} = {}", key, value);
        }

        // Modify and sync back
        let mut config = config;
        config.set("new_setting", "value");
        config.sync_to_global();

        println!("\nAdded new_setting and synced back to global");
        println!("Global now has: demo_new_setting = \"value\"");
    }

    #[test]
    fn uat_object_phantom_types_demo() {
        println!("\n=== Phantom Type System Demo ===\n");

        // Create typed objects
        let hub: HubConfig = Object::new("hub");
        let inf: InfConfig = Object::new("inf");
        let rsb: RsbConfig = Object::new("rsb");

        println!("Created typed configuration objects:");
        println!("  HubConfig with namespace: {}", hub.namespace());
        println!("  InfConfig with namespace: {}", inf.namespace());
        println!("  RsbConfig with namespace: {}", rsb.namespace());

        // Type conversion
        let mut generic: Object = Object::new("app");
        generic.set("mode", "production");

        println!("\nGeneric Object can be converted to typed:");
        let typed: Object<HubShape> = generic.as_type();
        println!("  Object -> Object<HubShape>");
        println!("  Data preserved: mode = {}", typed.get("mode"));

        println!("\nPhantom types provide compile-time documentation");
        println!("without runtime overhead!");
    }

    #[test]
    fn uat_object_helper_functions_demo() {
        println!("\n=== Helper Functions Demo ===\n");

        // Set up some config data
        set_var("hub_api", "https://hub.example.com");
        set_var("inf_team", "RSB Core");
        set_var("rsb_version", "2.0");

        println!("Using helper functions to get typed objects:");

        let hub = get_hub();
        println!("\n  get_hub() -> Object<HubShape>");
        println!("    namespace: {}", hub.namespace());

        let inf = get_inf();
        println!("\n  get_inf() -> Object<InfShape>");
        println!("    namespace: {}", inf.namespace());

        let rsb = get_rsb();
        println!("\n  get_rsb() -> Object<RsbShape>");
        println!("    namespace: {}", rsb.namespace());

        println!("\nGeneric helper for any namespace:");
        let custom = get_object::<()>("custom");
        println!("  get_object(\"custom\") -> Object");
        println!("    namespace: {}", custom.namespace());
    }

    #[test]
    fn uat_object_javascript_style_demo() {
        println!("\n=== JavaScript-like Object Usage Demo ===\n");

        let mut config = Object::<()>::new("app");

        // JavaScript-style property setting
        config.set("server.host", "localhost");
        config.set("server.port", "8080");
        config.set("database.url", "postgres://localhost/mydb");
        config.set("cache.enabled", "true");

        println!("Setting nested-style properties:");
        println!("  config.set(\"server.host\", \"localhost\")");
        println!("  config.set(\"server.port\", \"8080\")");
        println!("  config.set(\"database.url\", \"postgres://localhost/mydb\")");
        println!("  config.set(\"cache.enabled\", \"true\")");

        println!("\nAccessing with bracket notation:");
        println!("  config[\"server_host\"] = {}", &config["server_host"]);
        println!("  config[\"server_port\"] = {}", &config["server_port"]);
        println!("  config[\"database_url\"] = {}", &config["database_url"]);
        println!("  config[\"cache_enabled\"] = {}", &config["cache_enabled"]);

        println!("\nNote: Dots in keys are normalized to underscores!");
        println!("This provides JavaScript-like ergonomics in Rust!");
    }
}