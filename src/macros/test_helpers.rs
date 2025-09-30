// --- Test Helpers ---
// Namespaced re-exports for selective imports
pub use crate::mock_cmd;
#[macro_export]
macro_rules! mock_cmd {
    ({ $($cmd:expr => $out:expr),* $(,)? }) => {{
        let pairs: &[(&str, &str)] = &[ $( ($cmd, $out) ),* ];
        $crate::hosts::command::set_mock_cmds(pairs);
    }};
    (clear) => {{
        $crate::hosts::command::clear_mock_cmds();
    }};
}
