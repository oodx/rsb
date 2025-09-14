
# RSB - Next Features & Ideas

This document tracks potential features and improvements for future versions of the RSB library, based on ideas and feedback gathered during development.

## High Priority

### 1. Rich Date/Time Utilities
- **Description:** A full suite of `chrono`-based time and date utilities.
- **Features:**
  - `time_diff(start, end)`: Calculate the difference between two date/time strings.
  - `time_until(date)`: Produce a human-readable string like "in 2 hours". [SELECTED]
  - `human_date(date)`: Format dates nicely (e.g., "3 days ago").
  - `benchmark! { ... }`: A macro to measure the execution time of a block of code.
  - `job!(timeout: 10s, ...)`: Add timeout support to the job control system.
  - Timezone support for date conversions.

### 2. Advanced `sed` with Block Operations
- **Description:** Enhance the `sed` stream operation to work on blocks of text delimited by patterns or sentinels, not just line-by-line. This would allow for complex, in-place editing of structured files or code.
- **Example:** `stream.sed_block("/start_pattern/", "/end_pattern/", "s/old/new/g")`

### 3. Configurable Stderr
- **Description:** Allow full user configuration of `stderr` output colors and glyphs via the `RSB_COLORS` environment variable.
- **Format:** `RSB_COLORS="info:[color,glyph],error:[color],fatal:[glyph]"`
- **Requires:** Refactoring the `COLORS` and `GLYPHS` statics to be fully mutable and parsing this variable at bootstrap.

### 4. Robust Job Control
- **Description:** The `job!(wait: ...)` implementation is currently a placeholder. A fully robust, thread-safe implementation for waiting on and retrieving results from background jobs is needed.
- **Possible Solution:** Investigate using channels (`std::sync::mpsc`) or a different concurrency primitive to safely manage `JoinHandle`s.

## Medium Priority

### 5. More Powerful String Utilities
- **Description:** Add more ergonomic string manipulation macros.
- **Features:**
    - `str_explode!(string, on: delim, into: arr_name)`: Split a string into an RSB array.
    - `str_in!(needle in haystack)`: A clean, readable macro for substring checks.
    - `str_trim!(var)`: Macro to trim whitespace from a variable.
    - `str_len!(var)`: Macro to get the length of a variable.

### 6. Stream from Array / Delimited String
- **Description:** Add constructors to `Stream` to easily create a stream from an existing `Vec<String>` or a delimited string.
- **Example:** `Stream::from_vec(&my_vec)`, `stream!(from_array: &my_vec)`

### 7. Glob Support in Parameter Expansion
- **Description:** The `param!` macro's prefix/suffix removal (`#`, `##`, `%`, `%%`) currently uses simple string matching. It should support shell-style glob patterns.
- **Example:** `param!("FILENAME", suffix: "*.log")`

### 8. `colored!` Macro [SELECTED]
- **Description:** A dedicated macro for color/glyph parsing, allowing users to easily format their own strings without necessarily printing to `stderr`.
- **Example:** `let formatted = colored!("{red}My custom error{reset}");`

## Low Priority / Ideas

### 9. Native Windows Support
- **Description:** Currently, command execution relies on a `sh`-compatible shell. Add support for native Windows `cmd.exe` or `PowerShell` to improve portability. This would likely involve conditional compilation (`#[cfg(windows)]`). The `libc` dependency for signal handling would also need a Windows equivalent (`winapi`).

### 10. Official Testing Framework / Patterns
- **Description:** Document official patterns and provide helper functions for testing RSB scripts. A potential implementation could include:
  - **`rsb_test!` macro**: A test harness that creates an isolated RSB context for each test.
  - **`mock_fs!` macro**: A way to define a virtual filesystem for a test's duration, avoiding disk I/O and improving test speed and reliability.
    ```rust
    rsb_test!("test file processing" => {
        mock_fs!({
            "data/users.csv" => "id,name\n1,alice\n2,bob"
        });
        // ... test logic ...
    });
    ```
  - **`mock_cmd!` macro**: A way to mock the output of `cmd!` and `shell!` calls to prevent running real, slow, or destructive commands. [SELECTED]
    ```rust
    rsb_test!("test git status" => {
        mock_cmd!({
            "git status --porcelain" => " M src/lib.rs"
        });
        // ... test logic ...
    });
    ```
  - **Fluent Assertions**: A set of assertion macros that feel native to RSB, like `assert_var!("VAR", ==, "value")` or `assert_file_contains!("file.txt", "content")`.

### 11. Robust `cp -r` Fallback
- **Description:** The current `cp_r` fallback is very basic. A more robust, native Rust implementation could be provided for systems that don't have a `cp` command.
=======
# RSB - Next Features Roadmap

This document tracks the next set of features planned for the RSB library.

## Core Feature Set

### 1. Advanced Bash Parity Features
- **`math!` Macro**: A powerful, floating-point aware macro for shell-style arithmetic.
  - `math!("VAR = (OTHER_VAR * 1.05) + 2")`
  - `math!("COUNTER += 1")`
  - Should support: `+`, `-`, `*`, `/`, `%`, `**` (power), and shorthand assignments.
- **`cap_stream!` / `subst!` Macro**: A macro to support process substitution by capturing a stream's output to a temporary file and returning the path.
  - `let path = cap_stream!(cat!("file.txt").sort());`
  - `cmd!("diff {} /some/other/file.txt", path);`
  - Should use `$XDG_TMP` and have automatic cleanup on script exit.
- **`trap on ERR`**: Enhance `cmd!` and `shell!` to emit a `COMMAND_ERROR` event on non-zero exit codes, allowing for robust, script-wide error handling.
- **Robust `cp -r` Fallback**: Provide a more robust native Rust implementation for recursive copying to improve portability on systems without a standard `cp` command.

### 2. Foundational Utilities
- **`tmp!` Macro**: A macro to generate temporary file paths in a configurable temporary directory (`$XDG_TMP`).
  - `tmp!()` or `tmp!(random)`
  - `tmp!(pid)`
  - `tmp!(timestamp)`
- **Random Data Macros**: A suite of macros for generating random data.
  - `rand_alnum!(n)`
  - `rand_alpha!(n)`
  - `rand_hex!(n)`
  - `rand_string!(n)`
  - `rand_uuid!`
- **Dictionary Macros**: Utilities for working with word lists.
  - `dict!(<filepath>)`: Loads a newline or space-delimited file into an RSB array.
  - `rand_dict!(<array_name>)`: Selects a random word from an RSB array.
  - `rand_dict!(<array_name>, n, <delim>)`: Creates a delimited string of `n` random words.
  - `gen_dict!(<type>, n)`: Generates an array of `n` random words of a given type (e.g., `alnum`, `hex`).

### 3. Quality of Life
- **`stderr!` Macro**: A macro for formatting strings with color codes without printing them to stderr.
  - `let my_str = stderr!("{red}Error!{reset}");`
