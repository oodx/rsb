# Feature Gating Plan (Modular Concepts, Basic/Advanced)

Goal
- Make submodules opt-in via Cargo features per concept, with umbrella, basic, and advanced tiers.
- Express dependencies (e.g., many concepts depend on `global`).
- Keep default profile minimal and predictable (core only).

## Proposed Feature Map

Core (always on)
- `core`: umbrella for essential internals (global, cli base, streams, utils). Remains enabled implicitly.

Concept Features (opt-in)
- `strings-basic`, `strings-advanced`, `strings` (= basic + advanced)
- `params-basic`, `params-advanced`, `params` (depends on: `strings-basic`, `global`)
- `date-basic`, `date-advanced`, `date`
- `math-basic`, `math-advanced`, `math`
- `tokens-basic`, `tokens-advanced`, `tokens`
- `host` (env + paths)
- `threads`
- `bash`
- `cli-advanced` (optional helpers beyond core dispatch/options)

Visuals (existing)
- `visual` base; `colors-simple`, `colors-status`, `colors-named`, `colors-all`, `glyphs`, `prompts`, `visuals` umbrella.

Umbrella Sets
- `features-min`: `core`
- `features-cli`: `core`, `params-basic`
- `features-std`: `core`, `strings`, `params`, `date`, `tokens`, `host`
- `features-all`: everything except visuals
- `features-all+visuals`: `features-all` + `visuals`

## Cargo.toml Sketch

```toml
[features]
# keep default minimal
default = ["core"]

# core (always-on internals)
core = []

# concept features (examples)
strings-basic = []
strings-advanced = ["strings-basic"]
strings = ["strings-basic", "strings-advanced"]

params-basic = ["core", "strings-basic"]
params-advanced = ["params-basic"]
params = ["params-basic", "params-advanced"]

host = ["core"]
threads = ["core"]
bash = ["core"]

# visuals (existing)
visual = []
colors-simple = ["visual"]
colors-named = ["visual", "colors-simple"]
colors-status = ["visual"]
colors-all = ["visual", "colors-simple", "colors-named", "colors-status"]
visuals = ["visual", "colors-simple", "colors-named", "colors-status", "glyphs", "prompts"]
glyphs = ["visual"]
prompts = ["visual", "colors-simple"]
```

## Module Gating (examples)

lib.rs
```rust
// Core always-on
pub mod prelude;
pub mod global;
pub mod cli; // minimal/dispatch only

// Opt-in concepts
#[cfg(feature = "strings-basic")]
pub mod string;
#[cfg(feature = "date-basic")]
pub mod date;
#[cfg(feature = "params-basic")]
pub mod param;
#[cfg(feature = "tokens-basic")]
pub mod token;
#[cfg(feature = "math-basic")]
pub mod math;
#[cfg(feature = "threads")]
pub mod threads;
#[cfg(feature = "bash")]
pub mod bash;
#[cfg(feature = "host")]
pub mod hosts;

// Visuals existing gates remain
#[cfg(feature = "visual")]
pub mod visual;
```

Within modules
- Gate advanced helpers/macros under `#[cfg(feature = "<concept>-advanced")]`.
- Keep curated surfaces minimal in basic tier; re-export advanced in umbrella.

## Tests
- Annotate wrappers with `#![cfg(feature = "...")]` or `required-features` in examples.
- Add a small matrix in CI: `features-min`, `features-std`, `features-all`, and `visuals`.

## Migration Steps
1) Introduce features with default-on back-compat (temp: include concept features in default).
2) Add `#[cfg(...)]` around module exports and internal uses. Ensure prelude exports are guarded.
3) Mark tests with required-features; adjust `bin/test.sh` to pass feature sets where needed.
4) Flip defaults to `default = ["core"]` once parity is confirmed.
5) Document `Features` section in README and docs/tech/INDEX.md; add examples per set.

## Dependencies
- `params-*` → requires `strings-basic` and `global`.
- `math-*` → may rely on `global` for side-channel logging; keep core-only logging to avoid visuals.
- `prompts` → requires `colors-simple` under `visual`.
- `cli-advanced` → optional helper UX beyond dispatch/options.

## Notes
- Keep the prelude policy intact: no visuals/loggers in prelude; opt-in per feature.
- Avoid breaking public APIs; prefer deprecations then removal.
- Document feature guards in module docs and feature guides.

