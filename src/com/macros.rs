#[macro_export]
macro_rules! is_true {
    (var: $key:expr) => {
        $crate::com::is_true($key)
    };
    (true) => {
        true
    };
    (false) => {
        false
    };
    ($v:expr) => {
        $crate::com::is_true_any(&$v)
    };
}

#[macro_export]
macro_rules! is_false {
    (var: $key:expr) => {
        $crate::com::is_false($key)
    };
    (true) => {
        false
    };
    (false) => {
        true
    };
    ($v:expr) => {
        $crate::com::is_false_any(&$v)
    };
}
