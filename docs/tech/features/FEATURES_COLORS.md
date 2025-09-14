# RSB Visual Colors (Feature Flags) — Quick Stub

Purpose
- Document optional “visual” color subsystem for tools not using paintbox. Visuals are opt‑in and never exported via the prelude by default.

Feature Flags
- `visual` — base switch for the visuals package
- `colors-simple` — basic 8–16 colors (red/green/… + control like reset/bold)
- `colors-status` — status palette (magic/trace/note/silly, success/warn/error, etc.)
- `colors-named` — extended named palette (crimson/emerald/azure/…)
- `colors-all` — convenience alias for all color packages
- `glyphs` — optional Unicode glyphs for inline tags
- `prompts` — interactive prompts (depends on `colors-simple`)
- `visuals` — umbrella: simple + status + named + glyphs + prompts

Imports (explicit; not in prelude)
```rust
use rsb::visual::colors::{ color_mode, color_enable_with, color, colorize, bg, colorize_bg };
use rsb::colored; // macro lives at crate root
```

Runtime Enablement
- `color_mode("auto" | "always" | "never")`
- `color_enable_with("simple,status,named,bg[,glyphs]")`
- Lookup is case‑insensitive: `RED == red`
- Backgrounds require `bg` in the enable spec

Cheat Sheet
- `color(name) -> &str` — returns ANSI or ""
- `bg(name) -> String` — background ANSI or "" (when enabled)
- `colorize(text, name) -> String`
- `colorize_bg(text, name) -> String`
- `colored("{red}{bg:amber} hi {reset}") -> String` — inline tag expansion

Notes
- Named colors are stored in the global registry (HashMap). Do not convert them to enums.
- Visual logging macros (info!/warn!/…/colored!) are opt‑in; import explicitly from `rsb::{...}`.
- Nothing visual is re‑exported by the prelude.

Examples
```rust
color_mode("always");
color_enable_with("simple,status,named,bg");
println!("{}hello{}", color("red"), color("reset"));
println!("{}hello{}", bg("amber"), color("reset"));
println!("{}", colorize("hi", "magic"));
println!("{}", colored("{bg:emerald}{black} OK {reset}"));
```

Tests
- Visual UATs: `cargo test --tests --features visuals`

TBD (for full docs later)
- Short table of common names and categories
- Prompt examples and behavior matrix (opt_yes/opt_quiet/non‑TTY)
- CI matrix notes and example configuration

