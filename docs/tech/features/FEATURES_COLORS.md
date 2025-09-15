# RSB Visual Colors (MODERN + SPEC_ALIGNED)

Updated: 2025-09-15

Purpose
- Provide an optional, runtime‑configurable color system with minimal compile‑time surface.
- Keep visuals opt‑in and out of the prelude to preserve core build lean‑ness.
- Expose a curated, string‑first API aligned with MODULE_SPEC and HOWTO_TEST.

Module State (SPEC Alignment)
- MODERN: Colors live under `rsb::visual::colors` with focused subpackages and a registry.
- SPEC_ALIGNED:
  - Curated API only (see “Public API” below); no wildcard prelude exports.
  - Feature‑gated modules and additive “packages” (simple, status, named).
  - Tests present: sanity + UAT under feature gates (see HOWTO_TEST section below).
  - Docs present: this file (FEATURES_COLORS.md); HOWTO/INDEX link to it.

Feature Flags (Cargo.toml)
- Base: `visual` — enables the visual module; required by all visual features.
- Packages:
  - `colors-simple` — 8/16 color set and control codes; depends on `visual`.
  - `colors-status` — status palette (magic/trace/note/silly, success/warn/error…); depends on `visual`.
  - `colors-named` — extended named palette; depends on `visual`, `colors-simple`.
  - `colors-all` — convenience alias for all color packages (simple + named + status).
  - `colors` — convenience for baseline colors (alias: `visual` + `colors-simple`).
- Other visuals:
  - `glyphs` — optional Unicode glyphs for inline tags; depends on `visual`.
  - `prompts` — interactive prompts; depends on `visual`, `colors-simple`.
- Umbrella:
  - `visuals` — everything visual in one go: colors (simple, named, status) + glyphs + prompts.

Public API (curated; not in prelude)
```rust
use rsb::visual::colors::{
  color_mode, color_enable_with, color_enable,
  color, get_color, bg, colorize, colorize_bg,
  colored, get_all_colors,
};
// Note: `colored!` is a macro exported at crate root: `use rsb::colored;`
```

Runtime Model
- Modes: `color_mode("auto" | "always" | "never")` (respects NO_COLOR and RSB_COLOR env).
- Enable sets:
  - `color_enable()` — uses context/env: `opt_colors` or `RSB_COLORS`.
  - `color_enable_with("simple,status,named[,bg][,glyphs]")` — explicit.
- Backgrounds: add `bg` (or `background`, `backgrounds`, `on`) to enable background codes.
- Glyphs: add `glyphs` (only effective when compiled with the `glyphs` feature).
- Inline tags: `colored("{red}{bg:amber} ok {reset}")` supports `{g:key}` when `glyphs` compiled.

Behavior
- If colors are disabled or a name is unknown:
  - `color(name)` returns `""` and `colorize(text, name)` returns `text` unchanged.
  - `bg(name)` returns `""` unless backgrounds are enabled by spec or context.
- Case‑insensitive lookups: `RED == red`.

Examples
```rust
use rsb::visual::colors::{color_mode, color_enable_with, color, bg, colorize};
use rsb::colored;

color_mode("always");
color_enable_with("simple,status,named,bg");
println!("{}hello{}", color("red"), color("reset"));
println!("{}hello{}", bg("amber"), color("reset"));
println!("{}", colorize("hi", "magic"));
println!("{}", colored("{bg:emerald}{black} OK {reset}"));
```

Testing (HOWTO_TEST)
- Default core tests exclude visuals: `cargo test`.
- Visual suites and doctests: `cargo test --features visuals`.
- Test runner lanes:
  - `./bin/test.sh run colors` — feature tests
  - `./bin/test.sh run uat` — user‑facing demos
  - `./bin/test.sh run all` — full matrix (enables visuals where needed)

Module Notes
- Nothing visual is re‑exported by the prelude (prelude policy).
- Color registry is runtime‑driven (HashMap). Keep names as strings for flexibility;
  do not convert to enums.
- Logging: use `utils::stderrx(level, msg)` for core fallbacks; visual macros are optional.

Migration Tips
- Prefer `visuals` umbrella for apps and UATs that need colors + glyphs + prompts.
- Use granular flags for libraries that want tighter footprint.
- For plain builds, inline tags are stripped by `utils::expand_colors_unified` so output stays clean.
