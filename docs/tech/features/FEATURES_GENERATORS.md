# RSB Generators (MODERN + SPEC_ALIGNED)

Updated: 2025-09-15

Note
- Internally named “GX” (Generators and eXtensions). In docs we use Generators for clarity.

Purpose
- Provide ergonomic generators for strings, IDs, and selections.
- Keep logic in module helpers; expose thin macros for scripting ergonomics.
- Use consumer-owned adapters to integrate with other modules (math, fs) per MODULE_SPEC.

Module Layout (SPEC)
- `src/gx/mod.rs` — Orchestrator; re-exports curated surface and adapters.
- `src/gx/string/{mod.rs,helpers.rs,constants.rs}` — string generators.
- `src/gx/id/{mod.rs,helpers.rs}` — ID generators.
- `src/gx/collection/{mod.rs,helpers.rs}` — collection helpers.
- `src/gx/macros.rs` — module-owned macros for generators and dict helpers.
- Adapters: `src/gx/gx_math_adapter.rs`, `src/gx/gx_fs_adapter.rs` — integrate math/fs without circular deps.
- Sample data: `src/gx/data/dict/*.txt` for dict demonstrations.

Public API (curated)
- Functions (via gx::*):
  - `string::get_rand_alnum(n)`, `get_rand_alpha(n)`, `get_rand_hex(n)`, `get_rand_string(n)`
  - `id::get_rand_uuid()`
  - `collection::get_rand_from_slice(&[String]) -> Option<String>`
  - Adapters: `rand_usize_inclusive(min,max)`, `load_dict_file(path) -> Vec<String>`, `rand_from_dict_file(path)`
- Macros:
  - Random strings/IDs: `rand_alnum!(n)`, `rand_alpha!(n)`, `rand_hex!(n)`, `rand_string!(n)`, `rand_uuid!()`
  - Random range: `rand_range!(min, max)` (inclusive; uses gx_math_adapter)
  - Dict helpers: `rand_dict!("ARR_NAME")`, `rand_dict!("ARR_NAME", n [, delim])`, `gen_dict!(alnum|alpha|hex|string, n, into: "ARR_NAME")`

Design Notes
- Adapters live under the consumer module (gx) to reuse math and fs safely.
- Macros are thin; all logic in helpers/adapters.
- Sample dict files allow easy demos and tests without side effects.

Testing
- Sanity tests: `rsb/tests/generators_sanity.rs` cover macros and adapters.
- Visual/UAT: use `examples/showcase.rs` to demo usage interactively.

Status
- MODERN: Yes — helpers/adapters under gx, macros thin.
- SPEC_ALIGNED: Yes — orchestrator-only `mod.rs`, adapters in consumer, tests + sample data.

