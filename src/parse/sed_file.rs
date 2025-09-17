//! parse::sed_file — file-based sed helpers (owned by parse)
//!
//! These helpers implement file-oriented sed-like operations by composing
//! `streams::Stream` with minimal FS helpers (read/write). This avoids parse
//! depending on FS-provided sed functions while still reusing IO.

pub fn sed_lines_file(path: &str, start_line: usize, end_line: usize) -> String {
    use crate::streams::Stream;
    let content = crate::fs::read_file(path);
    Stream::from_string(&content)
        .sed_lines(start_line, end_line)
        .to_string()
}

pub fn sed_around_file(path: &str, pattern: &str, context: usize) -> String {
    use crate::streams::Stream;
    let content = crate::fs::read_file(path);
    Stream::from_string(&content)
        .sed_around(pattern, context)
        .to_string()
}

pub fn sed_insert_file(path: &str, content: &str, sentinel: &str) -> Result<(), String> {
    use crate::streams::Stream;
    let file_content = crate::fs::read_file(path);
    let result_stream = Stream::from_string(&file_content).sed_insert(content, sentinel)?;
    crate::fs::write_file(path, &result_stream.to_string());
    Ok(())
}

pub fn sed_template_file(path: &str, content: &str, sentinel: &str) {
    use crate::streams::Stream;
    let file_content = crate::fs::read_file(path);
    let result = Stream::from_string(&file_content)
        .sed_template(content, sentinel)
        .to_string();
    crate::fs::write_file(path, &result);
}

/// Read lines from a file, bounded by first and last line.
///
/// If `start` is None, begins at the first line. If `end` is None,
/// ends at the last line. Indices are 1-indexed and inclusive.
pub fn sed_read(path: &str, start: Option<usize>, end: Option<usize>) -> String {
    let content = crate::fs::read_file(path);
    let lines: Vec<&str> = content.lines().collect();
    let total = lines.len();
    if total == 0 {
        return String::new();
    }

    let s = start.unwrap_or(1).max(1);
    let e = end.unwrap_or(total).min(total);
    if s > e || s > total {
        return String::new();
    }
    let from = s - 1;
    let to = e; // end is inclusive for slicing upper bound
    lines[from..to].join("\n")
}
