# RSB AST/IR Considerations

This document captures goals, scope, trade‑offs, and references for introducing an internal AST/IR to RSB that cleanly supports command wrapping and structured text/terminal formatting while preserving RSB’s string‑first ergonomics.

## Goals
- Safe, composable representation of commands and pipelines (no fragile string concatenation).
- Structured text/UI representation for colors, wrapping, boxes, and alignment.
- Keep current macros ergonomic; AST is opt‑in and renders to strings at the edges.
- Unify color/theme handling and variable expansion across outputs.
- Improve testability with deterministic planning and mocks.

## Non‑Goals (for now)
- Rewriting all macros to require AST.
- Full‑blown TUI framework; only the primitives we need for formatting and boxes.
- Perfect cross‑platform terminal emulation.

## Primary Use Cases
- Command composition: safely build pipelines, redirections, timeouts, and backgrounds.
- Rich stderr/info output: consistent colors, glyphs, and themes with width‑aware wrapping.
- Terminal boxes: draw framed sections (titles, padding, borders) around content.
- Deterministic planning: preview “what would run” (dry‑run) and snapshot results.

## Integration Strategy (High‑Level)
- Feature gate: `ast` (off by default). Zero overhead when not enabled.
- Keep macros string‑first. Add parallel builder APIs for AST nodes.
- Bridge methods: `to_string()`, `to_stream()`, `execute()`; use existing OS module under the hood.
- Variable expansion performed at render/exec time via current Context rules.

## Command IR (Concept)
- Node types:
  - `CommandSpec { program, args, env, cwd, stdin, stdout, stderr, flags }`
  - `Pipeline(Vec<CommandSpec>)`
  - `Redirection { from, to, mode }`
- Flags: `{ background, timeout, retries }`
- Safety: arguments are stored unescaped; renderer handles shell quoting if needed.
- Execution: maps to existing `os::run_cmd_with_status`, `shell_exec`, respecting mocks.

## Text/UI IR (Concept)
- Node types:
  - `Text(content)`, `Styled(style, children)`, `Box(style, options, children)`, `Block(children)`
- Style: symbolic color names mapped via `COLORS` and `GLYPHS`; supports on/off/auto color modes.
- Wrapping: width‑aware (grapheme clusters), ANSI‑safe; optional alignment (left/center/right).
- Boxes: border styles (plain/round/double), titles, padding/margins.

## Rendering Pipeline
1. Expand: apply context variable expansion and optional globbing.
2. Layout: compute wrapping and boxes based on terminal width (fallback to configured width).
3. Paint: resolve style tokens to ANSI (or strip if colors disabled), concatenate.
4. Emit: return string or write to stream/file.

## Testing & Mocking
- Use `mock_cmd!` for command execution tests; allow AST pipelines to use the same mocks.
- Snapshot testing for rendered text/UI; structural testing for node trees.
- Dry‑run: render Pipeline to a plan string without executing.

## Performance & Footprint
- Feature gate: `ast` to keep binary slim by default.
- Lazy rendering: build nodes cheaply; render only at sinks.
- Avoid allocations in hot paths; prefer borrowing where possible.

## Open Questions / Design Choices
- Where to anchor builders? `rsb::ast::{cmd(), pipe()}` vs. `rsb::cmd_node!` macros.
- Should we provide a quasi‑quote style mini‑DSL for commands? (Optional, later.)
- Error strategy for AST building (panic vs. Result) — likely `Result` for public API.
- How to unify themes with existing env‑driven `RSB_COLORS` and GLYPHS — propose “theme providers”.
- Do we want a JSON/YAML serialization of plans for external tooling?

## References To Review (design and implementation cues)
- Command composition / pipelines:
  - duct (command pipelines in Rust)
  - cmd_lib (shell‑like command macros)
  - nushell (structured pipeline/AST concepts)
- Terminal colors & ANSI management:
  - anstyle, anstream, termcolor, owo‑colors, yansi
  - strip‑ansi‑escapes (for length calculations)
- Text wrapping and widths:
  - unicode‑segmentation (graphemes), unicode‑width
  - console crate (style + measure), textwrap crate
- TUI layout/boxes (for minimal box primitives):
  - ratatui (tui‑rs), crossterm (back‑end)
- UX examples:
  - bat (rich theming), ripgrep (color/TTY detection), indicatif (progress style)

Focus for review:
- ANSI‑safe wrapping strategies and width calculations with styles applied.
- Robust quoting/escaping strategies for commands across platforms.
- Theme switching and color on/off/auto flows.
- How these libraries structure builder APIs and error types.

## MVP Acceptance Criteria (Command IR)
- Build `CommandSpec` and `Pipeline` with env, cwd, args, redirections.
- Render to string and execute through OS layer; integrates with `mock_cmd!`.
- Context variable expansion at render time; tests for quoting and escaping.
- Basic docs and examples; gated behind `ast` feature.

## Phase 2 Acceptance (Text/UI IR)
- StyleSpan with symbolic colors; renderer respecting color mode and theme.
- Width‑aware wrapping with ANSI preservation; simple Box with title/padding.
- Integration with existing `colored!` and stderr formatting.
- Examples and tests for layout and wrapping.

## Migration & Exposure
- Keep `rsb::prelude::*` unchanged; expose AST under `rsb::ast` (feature‑gated).
- Provide conversion helpers between AST nodes and existing macros where it adds value.

## Next Steps
1. Align on nomenclature and module path (`rsb::ast` vs `rsb::ir`).
2. Confirm MVP scope and feature flagging.
3. Spike: prototype `CommandSpec` + render/exec + tests.
4. Draft API docs and examples before broadening to Text/UI IR.

