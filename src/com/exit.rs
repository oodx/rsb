// Exit code modeling and bridges
use std::process::ExitCode;

/// Canonical RSB exit kinds mapped onto process exit codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitKind {
    Success,       // 0
    Failure,       // 1 - General failure
    SystemFailure, // 2 - System/environment failure
    LogicFailure,  // 3 - Logic/programming error
    UserFailure,   // 4 - User input/usage error
}

impl ExitKind {
    #[inline]
    pub fn code(self) -> u8 {
        match self {
            ExitKind::Success => 0,
            ExitKind::Failure => 1,
            ExitKind::SystemFailure => 2,
            ExitKind::LogicFailure => 3,
            ExitKind::UserFailure => 4,
        }
    }

    #[inline]
    pub fn as_exit(self) -> ExitCode {
        ExitCode::from(self.code())
    }
}

/// Trait that converts values into process `ExitCode`.
pub trait AsExit {
    fn as_exit(self) -> ExitCode;
}

impl AsExit for bool {
    #[inline]
    fn as_exit(self) -> ExitCode {
        if self {
            ExitCode::SUCCESS
        } else {
            ExitCode::from(1)
        }
    }
}

impl AsExit for ExitKind {
    #[inline]
    fn as_exit(self) -> ExitCode {
        ExitCode::from(self.code())
    }
}

// Helpers to classify exit codes from integer values
#[inline]
pub fn is_success(code: i32) -> bool {
    code == 0
}
#[inline]
pub fn is_fail(code: i32) -> bool {
    code != 0
}

// Import ToBool trait from bool module to add ExitKind implementation
use super::bool::ToBool;

impl ToBool for ExitKind {
    fn to_bool(&self) -> bool {
        matches!(self, ExitKind::Success)
    }
}

impl ToString for ExitKind {
    fn to_string(&self) -> String {
        match self {
            ExitKind::Success => "success".to_string(),
            ExitKind::Failure => "error".to_string(),
            ExitKind::SystemFailure => "error".to_string(),
            ExitKind::LogicFailure => "error".to_string(),
            ExitKind::UserFailure => "error".to_string(),
        }
    }
}
