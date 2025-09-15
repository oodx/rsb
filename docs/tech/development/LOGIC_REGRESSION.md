# Logic Regression Advisory (Boolean Semantics)

Date: 2025-09-15

Scope
- This note documents a behavioral change in RSB boolean semantics (truth values) and provides guidance for migrating existing tools.

Change Summary
- Truth values now follow REBEL semantics everywhere:
  - `0` = true (success)
  - `1` = false (failure)
- String forms used in the Global store: `"0"` = true, `"1"` = false.
- Detectors and several utilities that previously emitted textual `"true"/"false"` now emit `"0"/"1"`.
- CLI options now set flags to `"0"` when present and `"1"` when negated.

Impact
- Any consumer code comparing against textual `"true"` or `"false"` will break.
- Scripts expecting `opt_*` to be `"1"` for true will need to update to `"0"`.

Mitigation & Migration
- Use macros for checks instead of string comparisons:
  - `is_true!(var: "KEY")` or `is_true!(value)`
  - `is_false!(var: "KEY")` or `is_false!(value)`
- If textual output is required at system boundaries, map values explicitly:
  - `"0" => "true"`, `"1" => "false"`
- Update CLI parsing expectations:
  - `--flag` → `opt_flag = "0"`
  - `--not-flag` → `opt_flag = "1"`
  - Multi-flag `--multi=dq!ts` → set `opt_d=0`, `opt_q=0`, `opt_t=1`, `opt_s=1`.

Compatibility Helpers
- `rsb::com` provides conversions and aliases:
  - `com::TRUE`/`FALSE` numeric, `com::TRUE_STR`/`FALSE_STR` strings
  - `is_true_val`, `is_false_val`, `is_true`, `is_false`
  - `ToRSBBool` trait + `is_true_any`/`is_false_any` for generic conversion

Validation
- All test lanes pass with updated semantics:
  - cargo test (default), visuals, progress, dev-pty
  - runner smoke lane (`./bin/test.sh run smoke`)

Recommended Checklist
- [ ] Replace string equality checks (`== "true"`) with `is_true!` or `is_false!`
- [ ] Update option flag expectations to `"0"` for enabled, `"1"` for disabled
- [ ] Verify detectors usage; adapt consumers to interpret `"0"/"1"`
- [ ] For external outputs, add a small mapping layer if textual booleans are required

Links
- See `docs/tech/features/FEATURES_TRUTH.md` for the canonical spec.

Course‑Correction Note
- The strict enforcement of REBEL numeric booleans across all surfaces is being re‑evaluated. For downstream tools, prefer `is_true!` / `is_false!` and `com` helpers to abstract representation. If we adjust back to a mixed representation (e.g., textual in some places), these helpers will preserve behavior for consumers.
