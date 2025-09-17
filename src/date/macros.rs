// --- Date Macros (module-owned) ---

#[macro_export]
macro_rules! date {
    () => {
        chrono::Local::now().to_string()
    };
    (iso) => {
        chrono::Local::now().to_rfc3339()
    };
    (epoch) => {
        chrono::Local::now().timestamp().to_string()
    };
    (epoch_ms) => {
        chrono::Local::now().timestamp_millis().to_string()
    };
    (utc_iso) => {
        chrono::Utc::now().to_rfc3339()
    };
    (human) => {
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    };
    ($format:expr) => {
        chrono::Local::now().format($format).to_string()
    };
}
