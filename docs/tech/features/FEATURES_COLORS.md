# RSB Visual Colors (MODERN + SPEC_ALIGNED)

Updated: 2025-09-21

Purpose
- Provide an optional, runtime‑configurable color system with minimal compile‑time surface.
- Keep visuals opt‑in and out of the prelude to preserve core build lean‑ness.
- Expose a curated, string‑first API aligned with MODULE_SPEC and HOWTO_TEST.

Module State (SPEC Alignment)
- MODERN: Colors live under `rsb::colors` with focused subpackages and a registry.
- SPEC_ALIGNED:
  - Curated API only (see “Public API” below); no wildcard prelude exports.
  - Feature‑gated modules and additive “packages” (simple, status, named).
  - Tests present: sanity + UAT under feature gates (see HOWTO_TEST section below).
  - Docs present: this file (FEATURES_COLORS.md); HOWTO/INDEX link to it.

Feature Flags (Cargo.toml)
- Base: `colors-core` — shared runtime registry; required by all color packages.
- Packages:
  - `colors-simple` — 8/16 color set and control codes; depends on `colors-core`.
  - `colors-status` — status palette (magic/trace/note/silly, success/warn/error…); depends on `colors-core`.
  - `colors-named` — extended named palette; depends on `colors-core`, `colors-simple`.
  - `colors-all` — convenience alias for all color packages (simple + named + status).
  - `colors` — convenience alias for the baseline stack (`colors-core` + `colors-simple`).
- Visual helpers:
  - `visual` — macros + prompts wiring; enables `colors-core` automatically.
  - `glyphs` — optional Unicode glyphs; depends on `visual`.
  - `prompts` — interactive prompts; depends on `visual`, `colors-simple`.
- Umbrella:
  - `visuals` — everything visual in one go: colors (simple, named, status) + glyphs + prompts.

Public API (curated; not in prelude)
```rust
use rsb::colors::{
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
use rsb::colors::{color_mode, color_enable_with, color, bg, colorize};
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

### API Surface Cheat Sheet
- Simple palette: `SimpleColor`, `get_simple_colors()`, `get_simple_color()`, `colorize_simple()`, `is_simple_color()`, `from_name()`.
- Status palette: `StatusColor`, `get_status_colors()`, `get_status_color()`, `get_status_color_categories()`, `colorize_status()`, `is_status_color()`, `from_name()`.
- Named palette: `get_named_colors()`, `get_boxy_extended_colors()`, `get_named_color()`, `colorize_named()`, `is_named_color()`.
- Runtime toggles: `color_mode()`, `set_color_mode()`, `color_enable()`, `color_enable_with()`, `color()`, `get_color()`, `bg()`, `colorize()`, `colorize_bg()`, `colored()`, `get_all_colors()`, `colors_enabled()`, `set_backgrounds_enabled()`, `backgrounds_enabled()`.
- Glyph integration: `glyph_enable()`, `set_glyphs_enabled()`, `glyphs_enabled()`, `get_all_glyphs()`.
- Messaging macros: `info!`, `okay!`, `warn!`, `error!`, `fatal!`, `debug!`, `trace!`, `confirm!`, `confirm_default!`, `ask!`, `select!`, `prompt!`, `confirm_timeout!`, `ask_timeout!`, `select_timeout!`, `prompt_timeout!`.
- Surface overview:
  - Simple palette: `SimpleColor`, `get_simple_colors`, `get_simple_color`, `colorize_simple`, `is_simple_color`, `from_name`.
  - Status palette: `StatusColor`, `get_status_colors`, `get_status_color`, `get_status_color_categories`, `colorize_status`, `is_status_color`, `from_name`.
  - Named palette: `get_named_colors`, `get_boxy_extended_colors`, `get_named_color`, `colorize_named`, `is_named_color`.
  - Runtime toggles: `color_mode`, `set_color_mode`, `color_enable`, `color_enable_with`, `color`, `get_color`, `bg`, `colorize`, `colorize_bg`, `colored`, `get_all_colors`, `colors_enabled`, `set_backgrounds_enabled`, `backgrounds_enabled`.
  - Glyph controls: `glyph_enable`, `set_glyphs_enabled`, `glyphs_enabled`, `get_all_glyphs` (requires the `glyphs` feature).
  - Messaging macros (behind visuals/prompts features): `info!`, `okay!`, `warn!`, `error!`, `fatal!`, `debug!`, `trace!`, `confirm!`, `confirm_default!`, `ask!`, `select!`, `prompt!`, plus timeout variants (`confirm_timeout!`, `ask_timeout!`, `select_timeout!`, `prompt_timeout!`).

Migration Tips
- Prefer `visuals` umbrella for apps and UATs that need colors + glyphs + prompts.
- Use granular flags for libraries that want tighter footprint.
- For plain builds, inline tags are stripped by `utils::expand_colors_unified` so output stays clean.

<!-- feat:colors -->

_Generated by bin/feat.py --update-doc._

* `src/colors/mod.rs`
  - pub use named::* (line 32)
  - pub use simple::* (line 38)
  - pub use {named::*, status::*} (line 41)

* `src/colors/named.rs`
  - pub use super::simple::* (line 7)
  - fn get_named_colors (line 11)
  - fn get_boxy_extended_colors (line 22)
  - fn get_named_color (line 111)
  - fn colorize_named (line 208)
  - fn is_named_color (line 218)

* `src/colors/registry.rs`
  - fn color_enable (line 259)
  - fn color_enable_with (line 264)
  - fn color_mode (line 269)
  - fn color (line 274)
  - fn get_color (line 288)
  - fn colorize (line 293)
  - fn bg (line 329)
  - fn colorize_bg (line 343)
  - fn colored (line 353)
  - fn get_all_colors (line 417)

* `src/colors/simple.rs`
  - enum SimpleColor (line 13)
  - fn from_name (line 131)
  - fn all (line 174)
  - fn get_simple_colors (line 211)
  - fn get_simple_color (line 219)
  - fn colorize_simple (line 226)
  - fn is_simple_color (line 236)

* `src/colors/status.rs`
  - enum StatusColor (line 12)
  - fn from_name (line 155)
  - fn all (line 206)
  - fn get_status_colors (line 250)
  - fn get_status_color (line 258)
  - fn colorize_status (line 265)
  - fn is_status_color (line 275)
  - fn get_status_color_categories (line 280)

* `src/colors/util.rs`
  - fn set_color_mode (line 10)
  - fn colors_enabled (line 18)
  - fn set_backgrounds_enabled (line 30)
  - fn backgrounds_enabled (line 34)

* `src/utils.rs`
  - fn should_print_level (line 8)
  - fn expand_colors_unified (line 27)
  - fn stderrx (line 59)
  - fn glyph_stderr (line 134)
  - pub use crate::string::is_name (line 140)
  - fn num_eq (line 144)
  - fn num_lt (line 151)
  - fn num_gt (line 158)
  - fn array_push (line 167)
  - fn array_get (line 174)
  - fn array_length (line 178)
  - fn array_contains (line 182)
  - fn get_array (line 187)
  - fn set_array (line 208)
  - fn prompt_user (line 219)
  - fn confirm_action (line 244)
  - fn is_true (line 283)
  - fn is_false (line 286)

* `src/visual/macros.rs`
  - macro colored! (line 11)
  - macro info! (line 24)
  - macro okay! (line 26)
  - macro warn! (line 28)
  - macro error! (line 30)
  - macro fatal! (line 32)
  - macro debug! (line 34)
  - macro trace! (line 36)
  - macro confirm! (line 41)
  - macro confirm_default! (line 49)
  - macro ask! (line 57)
  - macro select! (line 68)
  - macro prompt! (line 79)
  - macro confirm_timeout! (line 99)
  - macro ask_timeout! (line 113)
  - macro select_timeout! (line 127)
  - macro prompt_timeout! (line 146)

<!-- /feat:colors -->

