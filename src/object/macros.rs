//! Macros for the Object module

/// Get a value from the hub namespace
#[macro_export]
macro_rules! hub_config {
    ($key:expr) => {{
        $crate::global::get_var(&format!("hub_{}", $key))
    }};
}

/// Get a value from the inf namespace
#[macro_export]
macro_rules! inf_config {
    ($key:expr) => {{
        $crate::global::get_var(&format!("inf_{}", $key))
    }};
}

/// Get a value from the rsb namespace
#[macro_export]
macro_rules! rsb_config {
    ($key:expr) => {{
        $crate::global::get_var(&format!("rsb_{}", $key))
    }};
}

/// Get the full hub configuration object
#[macro_export]
macro_rules! hub_object {
    () => {{
        $crate::object::get_hub()
    }};
}

/// Get the full inf configuration object
#[macro_export]
macro_rules! inf_object {
    () => {{
        $crate::object::get_inf()
    }};
}

/// Get the full rsb configuration object
#[macro_export]
macro_rules! rsb_object {
    () => {{
        $crate::object::get_rsb()
    }};
}