//! RSB-Specific Paths and Namespacing
//!
//! Host-side setup of RSB path namespace derived from XDG paths.

/// Populate RSB_* paths in Global using XDG_* as base.
pub fn setup_rsb_paths() {
    crate::global::set_var("RSB_LIB_HOME", &crate::global::expand_vars("$XDG_LIB_HOME/rsb"));
    crate::global::set_var("RSB_ETC_HOME", &crate::global::expand_vars("$XDG_ETC_HOME"));
    crate::global::set_var("RSB_DATA_HOME", &crate::global::expand_vars("$XDG_DATA_HOME/rsb"));
    // BashFX-style bin flattening for rsb tools
    crate::global::set_var("RSB_BIN_HOME", &crate::global::expand_vars("$XDG_BIN_HOME/rsb"));
}

/// Convenience helper: path to a tool binary under RSB bin namespace.
pub fn rsb_tool_path(tool_name: &str) -> String {
    let base = crate::global::get_var("RSB_BIN_HOME");
    if base.is_empty() { return tool_name.to_string(); }
    format!("{}/{}", base, tool_name)
}

/// Convenience helper: default config file path for a tool.
pub fn rsb_config_path(tool_name: &str) -> String {
    let base = crate::global::get_var("RSB_ETC_HOME");
    if base.is_empty() { return format!("{}.conf", tool_name); }
    format!("{}/{}.conf", base, tool_name)
}

/// Convenience helper: data directory for a tool.
pub fn rsb_data_path(tool_name: &str) -> String {
    let base = crate::global::get_var("RSB_DATA_HOME");
    if base.is_empty() { return tool_name.to_string(); }
    format!("{}/{}", base, tool_name)
}
