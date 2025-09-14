//! XDG Base Directory Specification Paths
//!
//! Host-side functions to populate Global with XDG paths and optionally
//! ensure those directories exist on disk.

/// Set up XDG and XDG+ directory variables in Global.
/// Uses environment when present, with sensible fallbacks:
/// - XDG_CONFIG_HOME: $HOME/.config (unless already set)
/// - XDG_CACHE_HOME:  $HOME/.cache (unless already set)
/// - XDG_DATA_HOME:   $HOME/.local/share initially; overridden to $XDG_HOME/data (BashFX)
/// - XDG_HOME:        $HOME/.local
/// - XDG_LIB_HOME:    $XDG_HOME/lib
/// - XDG_ETC_HOME:    $XDG_HOME/etc
/// - XDG_BIN_HOME:    $XDG_HOME/bin
/// - XDG_TMP:         $HOME/.cache/tmp (unless already set)
pub fn setup_xdg_paths() {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());

    let xdg_config = std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
        format!("{}/.config", home)
    });
    crate::global::set_var("XDG_CONFIG_HOME", &xdg_config);

    let xdg_cache = std::env::var("XDG_CACHE_HOME").unwrap_or_else(|_| {
        format!("{}/.cache", home)
    });
    crate::global::set_var("XDG_CACHE_HOME", &xdg_cache);

    let xdg_data_initial = std::env::var("XDG_DATA_HOME").unwrap_or_else(|_| {
        format!("{}/.local/share", home)
    });
    crate::global::set_var("XDG_DATA_HOME", &xdg_data_initial);

    // XDG+ (BashFX) extensions
    let xdg_home = std::env::var("XDG_HOME").unwrap_or_else(|_| format!("{}/.local", home));
    crate::global::set_var("XDG_HOME", &xdg_home);
    crate::global::set_var("XDG_LIB_HOME", &format!("{}/lib", xdg_home));
    crate::global::set_var("XDG_ETC_HOME", &format!("{}/etc", xdg_home));
    crate::global::set_var("XDG_BIN_HOME", &format!("{}/bin", xdg_home));
    // Override to $XDG_HOME/data for BashFX preference
    crate::global::set_var("XDG_DATA_HOME", &format!("{}/data", xdg_home));

    // TMP: prefer XDG_TMP_HOME, fall back to legacy XDG_TMP, then default
    let xdg_tmp_home = std::env::var("XDG_TMP_HOME")
        .or_else(|_| std::env::var("XDG_TMP"))
        .unwrap_or_else(|_| format!("{}/.cache/tmp", home));
    crate::global::set_var("XDG_TMP_HOME", &xdg_tmp_home);
    // Back-compat alias
    crate::global::set_var("XDG_TMP", &xdg_tmp_home);
}

/// Create the key XDG+ directories if they don’t exist.
pub fn ensure_xdg_directories() {
    let xdg_dirs = [
        "XDG_LIB_HOME",
        "XDG_ETC_HOME",
        "XDG_BIN_HOME",
        "XDG_DATA_HOME",
        "XDG_TMP_HOME",
    ];

    for dir_var in &xdg_dirs {
        let dir_path = crate::global::get_var(dir_var);
        if !dir_path.is_empty() {
            crate::fs::mkdir_p(&dir_path);
        }
    }
}
