use rsb::prelude::*;

#[test]
fn adapter_apply_env_and_config() {
    // Env piece
    std::env::set_var("ADAPTER_ENV_VAR", "abc");
    rsb::global::apply_env();
    assert_eq!(get_var("ADAPTER_ENV_VAR"), "abc");
    // Modes
    std::env::set_var("TRACE", "1");
    rsb::global::apply_env();
    assert!(rsb::global::is_true("TRACE_MODE"));

    // Config piece
    let tmp = std::env::temp_dir().join("rsb_adapter.conf");
    std::fs::write(&tmp, b"FOO=bar\nVAL=42\n").unwrap();
    rsb::global::apply_config_files(&[tmp.to_str().unwrap()]);
    assert_eq!(get_var("FOO"), "bar");
    assert_eq!(get_var("VAL"), "42");

    // Combined helper
    std::env::set_var("ADAPTER_ENV_2", "xyz");
    let tmp2 = std::env::temp_dir().join("rsb_adapter2.conf");
    std::fs::write(&tmp2, b"K=V\n").unwrap();
    rsb::global::hydrate_env_and_files(&[tmp2.to_str().unwrap()]);
    assert_eq!(get_var("ADAPTER_ENV_2"), "xyz");
    assert_eq!(get_var("K"), "V");
}

