# RSB Global (Store, Expansion, Config, Introspection)

Updated: 2025-09-12

Purpose
- Provide a simple, bash-like global store for strings used across your app.
- Offer variable expansion, config file parse/save/export helpers, and lightweight introspection (function registry + call stack).
- Keep orchestration (env/paths/bootstrap) out of Global; those live in host/cli layers.

Imports
```rust
use rsb::prelude::*;        // re-exports rsb::global::*
// or explicitly:
use rsb::global::{ Global, set_var, get_var, has_var, unset_var, expand_vars };
```

Core API (String-first)
- Store
  - `set_var(key, value)` — sets a string value
  - `get_var(key) -> String` — returns value or ""
  - `has_var(key) -> bool`
  - `unset_var(key)`
  - `get_all_vars() -> HashMap<String,String>`
- Expansion
  - `expand_vars("$VAR and ${OTHER}") -> String`
  - Bash-like default/alt syntax is intentionally NOT supported here; use `param!` for rich expansions.
- Booleans (integer semantics)
  - `is_true(key)` is `get_var(key) == "1"`
  - `is_false(key)` is `get_var(key) == "0"`

Config Helpers
- Parse content
  - `parse_config_content(&str)` — parses simple `KEY=VALUE` lines
    - Supports double/single-quoted values
    - Array syntax: `ARRAY=(item1 item2 "item 3")`
      - Sets: `ARRAY`, `ARRAY_LENGTH`, `ARRAY_0`, `ARRAY_1`, ...
- File I/O
  - `load_config_file(path)` — reads file then `parse_config_content`
  - `save_config_file(path, keys: &[&str])` — writes selected keys to file, quoting when needed
  - `export_vars(path)` — writes `export KEY='VALUE'` lines for all variables
  - All paths accept `$VAR`/`${VAR}` via `expand_vars()`

Introspection
- Function registry
  - `register_function(name, description)`
  - `list_functions() -> Vec<(String,String)>`
- Call stack (see `CallFrame` struct)
  - `push_call(function, args: &[String])`
  - `pop_call() -> Option<CallFrame>`
  - `get_call_stack() -> Vec<CallFrame>`
  - `show_help()`, `show_functions()`, `show_call_stack()` — print-friendly helpers
- Token/validation helpers
  - `is_token_stream(value) -> bool` — quick shape check for "k=v" token sequences (comma/semicolon delimited)

Value Types
- Global stores only strings. Convert as needed at the edges (`parse::<T>()`).
- Arrays are represented via the `ARRAY_LENGTH` + indexed-key convention and `ARRAY` space-joined value.

Integration Patterns
- Environment (bootstrap)
  - `bootstrap!()` triggers CLI+Host bootstrap, importing environment into Global and setting XDG/RSB/script context.
  - After bootstrap, `get_var("HOME")`, `get_var("XDG_HOME")`, etc., are available.
- Config files
  - `src!("path.conf")` macro delegates to `global::load_config_file`; parsed keys land in Global.
  - `export!()` macro delegates to `global::export_vars`.
  - `load_config!("file1", "file2")` macro chain-calls `global::load_config_file` for batch hydration.
- Colors (optional, future split)
  - Color/glyph registries live under `rsb::global::registry`. The RSB_COLORS env split to host/global is backlogged; for now, visuals configure via runtime APIs (see FEATURES_COLORS.md).

Typical Use Cases
- Central app/site settings: paths, flags, small lists.
- Quick variable expansion for composing file paths and messages.
- Loading/saving `.conf` files without a heavy config crate.
- Lightweight CLI help/stack display (paired with `dispatch!`).

Examples
```rust
use rsb::prelude::*;

fn main() {
    let args = bootstrap!();
    options!(&args);

    // Set and read values
    let mut ctx = Global::new();
    ctx.set("PROJECT", "rsb");
    set_var("PROJECT", "rsb");
    set_var("HOME", "/home/user");
    println!("{}", expand_vars("$PROJECT at ${HOME}/src"));

    // Config
    parse_config_content("API_URL=\"https://example.com\"\nFEATURES=(a b \"c d\")\n");
    println!("features: {}", get_var("FEATURES"));
    save_config_file("$XDG_CONFIG_HOME/rsb/app.conf", &["API_URL","FEATURES"]);

    // Introspection
    register_function("process", "Process data");
    push_call("process", &args.all());
    show_functions();
    show_call_stack();
    let _ = pop_call();
}
```

Testing & UAT
- Sanity/core tests: `tests/global_core.rs`, `tests/features/global/core.rs`.
- UAT (visible): `tests/uat_global.rs` — demonstrates store, expansion, booleans, token stream check, config parse/save/export, and call stack.

Notes
- String-first by design; keep it simple and predictable.
- For rich parameter expansions (defaults/alternates/patterns), use `param!` rather than `expand_vars()`.
- Orchestration (env discovery, XDG paths, script awareness) will migrate to `rsb::host` and `rsb::cli` per the migration plan.

Adapters
- Simple env-only (no host dependency):
  - `apply_env_simple()` / `import_env_simple()` — mirror `std::env::vars()` into Global without setting modes or paths.
  - `hydrate_simple(&[&str])` — simple env import then `apply_config_files`.
- Host-enhanced env (uses host):
  - `apply_env()` — imports env and sets `*_MODE` flags from env (DEBUG/DEV/QUIET/TRACE).
  - `apply_config_files(&[&str])` — loads config files in order.
  - `hydrate_env_and_files(&[&str])` — host-enhanced env + config.

Guidance
- Use the simple adapter when you only need env values in Global and want zero coupling to host bootstrap.
- Use the host adapter when you also want standard mode flags and, later, XDG/script awareness and colors env parsing.
- Namespacing
- Global is flat by design. Use helper functions to simulate namespaces:
  - Dunder style: `NS__KEY`
  - Colon style: `NS::KEY`
  - Set: `ns_set(ns, key, value)` (dunder), `ns_set_cc(ns, key, value)` (colon)
  - Get: `ns_get(ns, key)` checks both styles; `ns_get_with_style(ns, key, style)` forces one
  - List: `ns_get_all(ns)` merges both styles (dunder preferred on conflicts)
  - Overlay: `ns_overlay_to_plain(ns)` copies namespaced keys → plain, `ns_overlay_plain_to_ns(ns, keys, style)` copies selected plain keys → namespaced
  - Enum helpers: pass `NsStyle::Dunder` or `NsStyle::Colon` when you need explicit style control.

Macros
- Config/IO
  - `export!()` — write all global variables (or `$RSB_EXPORT`) as shell exports.
  - `load_config!("path1", "path2")` — convenience front-end for `global::load_config_file`.
  - `src!(...)` — alias for `load_config!` maintained for legacy call sites.
- Validation/requirements (paired with Global + `error!` reporting)
  - `validate!(condition, message...)` — fail-fast guard (exit in prod, panic in tests).
  - `test!("name" => { ... })` — lightweight macro to wrap doc-style tests without pulling in the test harness.
  - `require_file!`, `require_dir!`, `require_command!`, `require_var!` — specialized `validate!` wrappers.
  - Iteration helpers such as `for_in!`, `file_in!`, and `case!` expose loop variables through Global for scripted validations.

<!-- feat:global -->

_Generated by bin/feat.py --update-doc._

* `src/global/adapter.rs`
  - fn apply_env (line 6)
  - fn apply_config_files (line 12)
  - fn hydrate_env_and_files (line 19)
  - fn import_env_simple (line 25)
  - fn apply_env_simple (line 32)
  - fn hydrate_simple (line 37)

* `src/global/config.rs`
  - fn parse_config_content (line 7)
  - fn load_config_file (line 56)
  - fn save_config_file (line 63)
  - fn export_vars (line 90)

* `src/global/mod.rs`
  - pub use store::* (line 8)
  - pub use utils::* (line 11)
  - pub use config::* (line 14)
  - pub use adapter::* (line 17)
  - pub use ns::* (line 20)
  - pub use registry::* (line 22)

* `src/global/ns.rs`
  - enum NsStyle (line 10)
  - fn ns_set (line 23)
  - fn ns_set_cc (line 28)
  - fn ns_get (line 33)
  - fn ns_get_with_style (line 44)
  - fn ns_get_all (line 49)
  - fn ns_overlay_to_plain (line 67)
  - fn ns_overlay_plain_to_ns (line 76)

* `src/global/registry.rs`
  - struct CallFrame (line 8)
  - fn register_function (line 50)
  - fn list_functions (line 57)
  - fn push_call (line 68)
  - fn pop_call (line 78)
  - fn get_call_stack (line 82)
  - fn show_help (line 86)
  - fn show_functions (line 129)
  - fn show_call_stack (line 140)

* `src/global/store.rs`
  - struct Global (line 9)
  - fn new (line 14)
  - fn set (line 19)
  - fn get (line 22)
  - fn has (line 25)
  - fn expand (line 28)
  - fn get_all_vars (line 46)
  - fn set_var (line 55)
  - fn get_var (line 58)
  - fn has_var (line 61)
  - fn unset_var (line 64)
  - fn expand_vars (line 67)
  - fn get_all_vars (line 70)

* `src/global/utils.rs`
  - pub use crate::com::{FALSE_STR, TRUE_STR} (line 4)
  - pub use crate::com::{is_false, is_false_val, is_true, is_true_val} (line 12)
  - fn is_token_stream (line 14)

* `src/macros/control_validation.rs`
  - macro for_in! (line 8)
  - macro test! (line 26)
  - macro case! (line 93)
  - macro file_in! (line 112)
  - macro export! (line 145)
  - macro src! (line 155)
  - macro load_config! (line 160)
  - macro validate! (line 166)
  - macro require_file! (line 196)
  - macro require_dir! (line 203)
  - macro require_command! (line 214)
  - macro require_var! (line 221)

<!-- /feat:global -->
