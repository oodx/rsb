RSB ↔ BashFX Alignment Notes

Purpose
- Capture how BashFX v3 standards map to the RSB host layer (Rust), to guide consistent behavior across scripts and binaries.

Key Alignments
- XDG layering
  - XDG(0): Respect upstream defaults for `XDG_CONFIG_HOME` (`~/.config`), `XDG_CACHE_HOME` (`~/.cache`), and initial `XDG_DATA_HOME` (`~/.local/share`).
  - XDG(1): Prefer `$HOME/.local` layout and name it `XDG_HOME`. Derive:
    - `XDG_LIB_HOME` = `$XDG_HOME/lib`
    - `XDG_ETC_HOME` = `$XDG_HOME/etc`
    - `XDG_BIN_HOME` = `$XDG_HOME/bin`
    - `XDG_DATA_HOME` = `$XDG_HOME/data` (override for BashFX preference)
  - TMP policy: Prefer `XDG_TMP_HOME` = `$HOME/.cache/tmp`. Keep `XDG_TMP` as a back‑compat alias.
  - Policy: Do not mutate `$HOME` for virtualization; prefer `XDG_HOME` input from the environment when provided.

- RSB path namespace
  - `RSB_LIB_HOME` = `$XDG_LIB_HOME/rsb`
  - `RSB_ETC_HOME` = `$XDG_ETC_HOME`
  - `RSB_DATA_HOME` = `$XDG_DATA_HOME/rsb`
  - `RSB_BIN_HOME` = `$XDG_BIN_HOME/rsb` (lib→bin flattening pattern)

- Modes
  - Convert presence of env flags to integer booleans in Global: `DEBUG` → `DEBUG_MODE=1`, `DEV` → `DEV_MODE=1`, `QUIET` → `QUIET_MODE=1`, `TRACE` → `TRACE_MODE=1`.

- Script context
  - From `argv[0]` and `cwd` derive: `SCRIPT_NAME`, `SCRIPT_PATH`, `SCRIPT_DIR`, `PWD`.
  - These are host responsibilities, separate from high‑level CLI parsing.

Host API Surfaces (implemented)
- `hosts::setup_xdg_paths()` sets XDG(0) + XDG(1) keys in Global with safe fallbacks; prefers env `XDG_HOME`.
- `hosts::ensure_xdg_directories()` creates `XDG_LIB_HOME`, `XDG_ETC_HOME`, `XDG_BIN_HOME`, `XDG_DATA_HOME`, and `XDG_TMP_HOME` if missing.
- `hosts::setup_rsb_paths()` derives RSB_* keys from XDG+.
- `hosts::setup_execution_context(args)` sets script context keys.
- `hosts::import_environment()`, `hosts::setup_standard_modes()` import env and apply mode flags.

New Host Bootstrap (added)
- `hosts::bootstrap(args)` orchestrates:
  1) import env → 2) setup XDG → 3) setup RSB → 4) ensure dirs → 5) mode flags → 6) script context → 7) args context (`ARGC`, `ARGV_*`).
- `hosts::bootstrap_from_env()` collects args from `std::env::args()` and calls `bootstrap`.

Testing Alignment
- Unit + UAT validate keys and directory creation under a temp home.
- Sanity tests print key XDG/RSB values; UAT demo includes script context outputs.

Open Topics (tracked for future alignment)
- CLI: richer bootstrap flags (debug/quiet overrides, config path hooks) and interactive detection.
- Virtualization helpers: standard sandboxes that leverage `XDG_HOME` without mutating `HOME`.
- Colors/glyphs: environment parsing harmonization across BashFX and RSB (beyond current scope).

