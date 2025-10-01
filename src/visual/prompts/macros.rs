//! Prompt macros for the prompts feature.
//!
//! These macros wrap the visual prompts subsystem and require the `prompts` feature.

// Prompt macros require the `prompts` feature.
#[cfg(feature = "prompts")]
#[macro_export]
macro_rules! confirm {
    ($msg:expr) => {
        $crate::visual::prompts::confirm($msg)
    };
}

#[cfg(feature = "prompts")]
#[macro_export]
macro_rules! confirm_default {
    ($msg:expr, $default:expr) => {
        $crate::visual::prompts::confirm_default($msg, $default)
    };
}

#[cfg(feature = "prompts")]
#[macro_export]
macro_rules! ask {
    ($msg:expr) => {
        $crate::visual::prompts::ask($msg, None)
    };
    ($msg:expr, $default:expr) => {
        $crate::visual::prompts::ask($msg, Some($default))
    };
}

#[cfg(feature = "prompts")]
#[macro_export]
macro_rules! select {
    ($msg:expr, $options:expr) => {
        $crate::visual::prompts::select($msg, $options, None)
    };
    ($msg:expr, $options:expr, $default_idx:expr) => {
        $crate::visual::prompts::select($msg, $options, Some($default_idx))
    };
}

#[cfg(feature = "prompts")]
#[macro_export]
macro_rules! prompt {
    ("confirm", $msg:expr) => {
        $crate::visual::prompts::confirm($msg)
    };
    ("ask", $msg:expr) => {
        $crate::visual::prompts::ask($msg, None)
    };
    ("ask", $msg:expr, $default:expr) => {
        $crate::visual::prompts::ask($msg, Some($default))
    };
    ("select", $msg:expr, $options:expr) => {
        $crate::visual::prompts::select($msg, $options, None)
    };
    ("select", $msg:expr, $options:expr, $default_idx:expr) => {
        $crate::visual::prompts::select($msg, $options, Some($default_idx))
    };
}

#[cfg(feature = "prompts")]
#[macro_export]
macro_rules! confirm_timeout {
    ($msg:expr) => {
        $crate::visual::utils::confirm_with_timeout($msg, None, false)
    };
    ($msg:expr, $timeout:expr) => {
        $crate::visual::utils::confirm_with_timeout($msg, Some($timeout), false)
    };
    ($msg:expr, $timeout:expr, $default:expr) => {
        $crate::visual::utils::confirm_with_timeout($msg, Some($timeout), $default)
    };
}

#[cfg(feature = "prompts")]
#[macro_export]
macro_rules! ask_timeout {
    ($msg:expr) => {
        $crate::visual::utils::ask_with_timeout($msg, None, None)
    };
    ($msg:expr, $default:expr) => {
        $crate::visual::utils::ask_with_timeout($msg, Some($default), None)
    };
    ($msg:expr, $default:expr, $timeout:expr) => {
        $crate::visual::utils::ask_with_timeout($msg, Some($default), Some($timeout))
    };
}

#[cfg(feature = "prompts")]
#[macro_export]
macro_rules! select_timeout {
    ($msg:expr, $options:expr) => {
        $crate::visual::utils::select_with_timeout($msg, $options, None, None)
    };
    ($msg:expr, $options:expr, $default_idx:expr) => {
        $crate::visual::utils::select_with_timeout($msg, $options, Some($default_idx), None)
    };
    ($msg:expr, $options:expr, $default_idx:expr, $timeout:expr) => {
        $crate::visual::utils::select_with_timeout(
            $msg,
            $options,
            Some($default_idx),
            Some($timeout),
        )
    };
}

#[cfg(feature = "prompts")]
#[macro_export]
macro_rules! prompt_timeout {
    ("confirm", $msg:expr) => {
        $crate::visual::utils::confirm_with_timeout($msg, None, false)
    };
    ("confirm", $msg:expr, $timeout:expr) => {
        $crate::visual::utils::confirm_with_timeout($msg, Some($timeout), false)
    };
    ("ask", $msg:expr) => {
        $crate::visual::utils::ask_with_timeout($msg, None, None)
    };
    ("ask", $msg:expr, $default:expr) => {
        $crate::visual::utils::ask_with_timeout($msg, Some($default), None)
    };
    ("ask", $msg:expr, $default:expr, $timeout:expr) => {
        $crate::visual::utils::ask_with_timeout($msg, Some($default), Some($timeout))
    };
    ("select", $msg:expr, $options:expr) => {
        $crate::visual::utils::select_with_timeout($msg, $options, None, None)
    };
    ("select", $msg:expr, $options:expr, $default_idx:expr) => {
        $crate::visual::utils::select_with_timeout($msg, $options, Some($default_idx), None)
    };
    ("select", $msg:expr, $options:expr, $default_idx:expr, $timeout:expr) => {
        $crate::visual::utils::select_with_timeout(
            $msg,
            $options,
            Some($default_idx),
            Some($timeout),
        )
    };
}
