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
use rsb::global::{ set_var, get_var, has_var, unset_var, expand_vars };
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
- Call stack
  - `push_call(function, args: &[String])`
  - `pop_call() -> Option<CallFrame>`
  - `get_call_stack() -> Vec<CallFrame>`
  - `show_help()`, `show_functions()`, `show_call_stack()` — print-friendly helpers

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
