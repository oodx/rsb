# Date and Time Utilities (FEATURES_DATE)

Updated: 2025-09-12

Scope
- Provides date/time macros and helpers. Replaces the old `time` module.
- String‑first APIs suitable for scripting workflows.

Macros
- `date!()` — Local now as string (debug format)
- `date!(iso)` — Local time in RFC3339
- `date!(utc_iso)` — UTC in RFC3339
- `date!(epoch)` — Unix timestamp seconds
- `date!(epoch_ms)` — Unix timestamp milliseconds
- `date!("%Y-%m-%d")` — Local time with custom strftime format

Helpers
- `current_timestamp() -> i64` — seconds since epoch (UTC)
- `current_timestamp_millis() -> i64` — milliseconds since epoch (UTC)
- `format_time(ts: i64, fmt: &str) -> String` — format local time
- `format_time_utc(ts: i64, fmt: &str) -> String` — format UTC time
- `parse_time(s: &str, fmt: &str) -> Result<i64, DateError>` — parse with strftime format into UTC seconds
- `time_diff(start: &str, end: &str) -> String` — humanized difference (e.g., `1d 2h 3m 4s`)
- `human_date(s: &str) -> String` — "… ago" or "in …" for input date
- `time_until(s: &str) -> String` — "in …" or "0s" if already past

Parsing behavior
- Tries RFC3339, RFC2822, "%Y-%m-%d %H:%M:%S %z", "%Y-%m-%d %H:%M:%S%.3f %z".
- Falls back to naive local time (no tz) with "%Y-%m-%d %H:%M:%S" and "%Y-%m-%d %H:%M:%S%.3f".
- `parse_time` returns a typed `DateError` for clearer error handling.

Duration formatting
- Output is compact: `Xd Xh Xm Xs` (seconds included when others absent).
- Future enhancements may add verbose/compact modes and weeks.

Testing
- Feature tests: `tests/features_date.rs` wrapper → `tests/features/date/date_test.rs`.
- UAT: `tests/uat_date.rs` prints macro and helper outputs.

Migration notes
- The `date!` macro moved from `src/macros/time_math.rs` to `src/date/macros.rs`.
- Previous `time.rs` helpers were migrated into `src/date/helpers.rs` and extended with UTC/millis and typed errors.

