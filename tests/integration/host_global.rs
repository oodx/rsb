use rsb::prelude::*;

#[test]
fn host_global_hydrate_and_ns() {
    // Prepare env + config file
    std::env::set_var("APPX_URL", "https://example.com");
    let tmp = std::env::temp_dir().join("host_global_test.conf");
    std::fs::write(&tmp, b"CFG=ok\n").unwrap();

    // Hydrate env+modes + config
    rsb::hosts::global::hydrate_env_and_configs(&[tmp.to_str().unwrap()]);
    assert_eq!(rsb::global::get_var("APPX_URL"), "https://example.com");
    assert_eq!(rsb::global::get_var("CFG"), "ok");

    // Prefix import
    std::env::set_var("MYAPP_FOO", "bar");
    std::env::set_var("MYAPP_NUM", "7");
    rsb::hosts::global::import_env_with_prefix("MYAPP_", true);
    assert_eq!(rsb::global::get_var("FOO"), "bar");
    assert_eq!(rsb::global::get_var("NUM"), "7");

    // Namespacing helpers
    rsb::hosts::global::ns_set("ns1", "name", "value");
    assert_eq!(rsb::hosts::global::ns_get("ns1", "name"), "value");
    let all = rsb::hosts::global::ns_get_all("ns1");
    assert!(all.iter().any(|(k, v)| k == "name" && v == "value"));
}
