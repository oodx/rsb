use assert_fs::TempDir;
use rsb::prelude::*;

#[test]
fn host_xdg_and_rsb_paths_and_dirs() {
    // Use a temp HOME to avoid touching real user dirs
    let tmp_home = TempDir::new().unwrap();
    let home_str = tmp_home.path().to_string_lossy().to_string();
    std::env::set_var("HOME", &home_str);
    // Clear XDG to force defaults
    for k in [
        "XDG_CONFIG_HOME",
        "XDG_CACHE_HOME",
        "XDG_DATA_HOME",
        "XDG_HOME",
        "XDG_LIB_HOME",
        "XDG_ETC_HOME",
        "XDG_BIN_HOME",
        "XDG_TMP",
    ] {
        std::env::remove_var(k);
    }

    // Set up XDG and ensure directories
    rsb::hosts::setup_xdg_paths();
    rsb::hosts::ensure_xdg_directories();

    // Validate Global keys
    assert_eq!(
        rsb::global::get_var("XDG_CONFIG_HOME"),
        format!("{}/.config", home_str)
    );
    assert_eq!(
        rsb::global::get_var("XDG_CACHE_HOME"),
        format!("{}/.cache", home_str)
    );
    assert_eq!(
        rsb::global::get_var("XDG_HOME"),
        format!("{}/.local", home_str)
    );
    assert_eq!(
        rsb::global::get_var("XDG_LIB_HOME"),
        format!("{}/.local/lib", home_str)
    );
    assert_eq!(
        rsb::global::get_var("XDG_ETC_HOME"),
        format!("{}/.local/etc", home_str)
    );
    assert_eq!(
        rsb::global::get_var("XDG_BIN_HOME"),
        format!("{}/.local/bin", home_str)
    );
    // BashFX override: data -> $XDG_HOME/data
    assert_eq!(
        rsb::global::get_var("XDG_DATA_HOME"),
        format!("{}/.local/data", home_str)
    );
    assert_eq!(
        rsb::global::get_var("XDG_TMP_HOME"),
        format!("{}/.cache/tmp", home_str)
    );
    // Back-compat alias also set
    assert_eq!(
        rsb::global::get_var("XDG_TMP"),
        rsb::global::get_var("XDG_TMP_HOME")
    );

    // Validate directories exist
    for p in [
        rsb::global::get_var("XDG_LIB_HOME"),
        rsb::global::get_var("XDG_ETC_HOME"),
        rsb::global::get_var("XDG_BIN_HOME"),
        rsb::global::get_var("XDG_DATA_HOME"),
        rsb::global::get_var("XDG_TMP"),
    ] {
        assert!(
            std::path::Path::new(&p).is_dir(),
            "expected dir to exist: {}",
            p
        );
    }

    // Now RSB paths
    rsb::hosts::setup_rsb_paths();
    assert_eq!(
        rsb::global::get_var("RSB_LIB_HOME"),
        format!("{}/.local/lib/rsb", home_str)
    );
    assert_eq!(
        rsb::global::get_var("RSB_ETC_HOME"),
        format!("{}/.local/etc", home_str)
    );
    assert_eq!(
        rsb::global::get_var("RSB_DATA_HOME"),
        format!("{}/.local/data/rsb", home_str)
    );
    assert_eq!(
        rsb::global::get_var("RSB_BIN_HOME"),
        format!("{}/.local/bin/rsb", home_str)
    );
}

#[test]
fn host_script_execution_context() {
    let tmp_dir = TempDir::new().unwrap();
    let tmp_path = tmp_dir.path();
    // Change cwd to tmp for predictable PWD
    std::env::set_current_dir(tmp_path).unwrap();
    let script_path = tmp_path.join("demo_script.sh");
    std::fs::write(&script_path, b"echo ok").unwrap();

    let args = vec![script_path.to_string_lossy().to_string()];
    rsb::hosts::setup_execution_context(&args);

    assert_eq!(rsb::global::get_var("SCRIPT_NAME"), "demo_script.sh");
    assert_eq!(
        rsb::global::get_var("SCRIPT_PATH"),
        script_path.to_string_lossy().to_string()
    );
    assert_eq!(
        rsb::global::get_var("SCRIPT_DIR"),
        tmp_path.to_string_lossy().to_string()
    );
    assert_eq!(
        rsb::global::get_var("PWD"),
        tmp_path.to_string_lossy().to_string()
    );
}
