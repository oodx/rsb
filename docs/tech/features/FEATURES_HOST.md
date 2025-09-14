# RSB Host (Environment, XDG/RSB Paths, Script Context)

Updated: 2025-09-12

Purpose
- Discover and standardize host environment for RSB apps (env → Global).
- Provide XDG+ (BashFX v3-aligned) directory keys and ensure required folders exist.
- Surface RSB path namespace derived from XDG+.
- Derive script execution context (SCRIPT_NAME, PATH, DIR, PWD).
- Offer a single bootstrap that sequences all the above.

Imports
```rust
use rsb::hosts;                 // host surfaces
use rsb::global;                // global store/expand/config
use rsb::prelude::*;            // convenient re-exports
```

Core API
- Environment
  - `hosts::import_environment()` — mirror `std::env::vars()` into Global.
  - `hosts::setup_standard_modes()` — map `DEBUG|DEV|QUIET|TRACE` → `*_MODE=1`.
  - `hosts::env_bootstrap()` — import + modes.
  - `hosts::env_to_global()` / `hosts::global_to_env()` — sync helpers.
- XDG+ (BashFX v3)
  - `hosts::setup_xdg_paths()` — sets:
    - XDG(0): `XDG_CONFIG_HOME`, `XDG_CACHE_HOME` (respects env or uses `$HOME/.config|.cache`).
    - XDG(1): `XDG_HOME` (prefers env `XDG_HOME`, else `$HOME/.local`), then
      `XDG_LIB_HOME`, `XDG_ETC_HOME`, `XDG_BIN_HOME`, and overrides `XDG_DATA_HOME` → `$XDG_HOME/data`.
    - TMP: `XDG_TMP_HOME` (preferred) and back‑compat alias `XDG_TMP` → `$HOME/.cache/tmp` by default.
  - `hosts::ensure_xdg_directories()` — creates `XDG_LIB_HOME`, `XDG_ETC_HOME`, `XDG_BIN_HOME`, `XDG_DATA_HOME`, `XDG_TMP_HOME`.
- RSB paths
  - `hosts::setup_rsb_paths()` — derives `RSB_LIB_HOME`, `RSB_ETC_HOME`, `RSB_DATA_HOME`, `RSB_BIN_HOME` from XDG+.
  - Helpers: `hosts::rsb_tool_path(name)`, `hosts::rsb_config_path(name)`, `hosts::rsb_data_path(name)`.
- Script context
  - `hosts::setup_execution_context(args)` — sets `SCRIPT_NAME`, `SCRIPT_PATH`, `SCRIPT_DIR`, `PWD` from `argv[0]` and `cwd`.
- Bootstrap
  - `hosts::bootstrap(args)` — sequences: env → xdg → rsb → mkdir → modes → script → args context (`ARGC`, `ARGV_n`).
  - `hosts::bootstrap_from_env()` — convenience wrapper using `std::env::args()`.
- Host→Global composition
  - `hosts::global::hydrate_env_and_configs(&[paths])` — env bootstrap + load config files.
  - `hosts::global::import_env_with_prefix(prefix, strip)` — import only prefixed env vars.
  - Namespace helpers proxy: `hosts::global::{ns_set, ns_get, ns_get_all}`.

Examples
```rust
use rsb::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    rsb::hosts::bootstrap(&args);

    // XDG/RSB
    println!("XDG_HOME={}", global::get_var("XDG_HOME"));
    println!("RSB_BIN_HOME={}", global::get_var("RSB_BIN_HOME"));

    // Script + args context
    println!("SCRIPT_NAME={}", global::get_var("SCRIPT_NAME"));
    println!("ARGC={}", global::get_var("ARGC"));
}
```

Testing & UAT
- Sanity: `tests/sanity/host_env.rs`, `tests/sanity/host_paths.rs` (visible prints).
- Unit: `tests/host_env.rs`, `tests/host_paths.rs`, `tests/host_global.rs`.
- UAT: `tests/uat/host_env.rs`, `tests/uat/host_paths.rs`; runner aliases: `./bin/test.sh run host-env`, `./bin/test.sh run host-paths`.

Notes
- Follows BashFX v3 XDG(1) layout; see `BASHFX-v3.md` and `RSB_BASHFX_ALIGN.md`.
- For virtualization/sandboxes prefer `XDG_HOME` instead of mutating `HOME`.
- Host focuses on discovery and context; business logic and UI belong to CLI/app layers.

