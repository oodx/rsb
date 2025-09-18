# RSB Integration Plan for fx-testsh Legendary Runner

**Audience**: RSB maintainers and release engineers planning to adopt the new `fx-testsh` legendary runner.
**Goal**: Provide a repeatable playbook for re-integrating the generated BashFX legendary runner into the existing RSB repository (which still houses its own legacy `bin/test.sh`).

---

## 1. Prerequisites

1. **Runner Build**
   - Ensure `fx-testsh` is on a clean commit with the latest parts-based build:
     ```bash
     ./bin/build.sh build
     git status  # should be clean
     ```
   - Confirm all lanes, module filters, and doc overlays have been validated (see `TASKS.txt`).

2. **RSB Repository Access**
   - Clone or update the RSB repo side-by-side with `fx-testsh`.
   - Verify you have permission to land changes in `rsb/bin/test.sh` and associated docs.

3. **Tooling**
   - `boxy` available in PATH (fall-back to stderr is acceptable but Boxy makes CI parity smoother).
   - `cargo`, `bash`, and relevant feature flags (`visuals`, `stdopts`) installed for local verification.

---

## 2. File Sync Strategy

RSB already has a legacy `bin/test.sh`. For the legendary runner, adopt the generated output plus its build assets:

1. **Copy Generated Script**
   - From `fx-testsh` repo root:
     ```bash
     cp test.sh /path/to/rsb/bin/test.sh
     chmod +x /path/to/rsb/bin/test.sh
     ```

2. **Optional: Adopt Parts Build**
   - If RSB is ready to maintain the parts-based structure, also copy:
     ```bash
     rsync -av parts/ /path/to/rsb/parts/
     cp bin/build.sh /path/to/rsb/bin/build-testsh.sh  # or reuse naming conventions
     ```
   - Update RSB docs and build scripts to invoke `bin/build-testsh.sh build` before packaging.

3. **Docs**
   - Mirror these references:
     - `docs/reference/RSB_TEST_RUNNER.md`
     - `docs/reference/RSB_TESTSH_INTEGRATION.md` (this file)
     - Any updated `docs/rebel/*.md` that the runner references via overrides (see `_docs_seed_default`).

---

## 3. Environment & Profiles

1. **RSB Profile**
   - The runner ships with `TESTSH_PROFILE=rsb` overlays. Validate the lane map in `test.sh` under `_profile_rsb()`.
   - Ensure RSB’s CI sets either `--profile=rsb` or `TESTSH_PROFILE=rsb` when invoking the runner.

2. **Legacy Flags**
   - The legendary runner exports `RSB_*` env vars (`RSB_TEST_MODE`, `RSB_VERBOSE`, etc.) so downstream tests remain compatible.
   - Strict/override semantics match the legacy script; `--override` now prints Boxy warnings before executing violating lanes.

3. **In-Repo Defaults**
   - If desired, add a wrapper script or CI step that sets `TESTSH_PROFILE=rsb` by default to mimic the legacy behaviour.

---

## 4. Validation Checklist (RSB Repo)

Run these commands *inside the RSB repo* after copying the new runner:

```bash
# Ensure generated script is fresh
./bin/build-testsh.sh build   # optional if parts adopted

# Structure enforcement
./bin/test.sh lint

# Core lanes (RSB profile)
./bin/test.sh --rsb run sanity
./bin/test.sh --rsb run uat
./bin/test.sh --rsb run smoke

# Module filtering spot-check (if legacy wrappers exist)
./bin/test.sh --rsb run uat colors

# Docs overlay
./bin/test.sh --rsb docs runner
./bin/test.sh --rsb docs org

# Adhoc example (if applicable)
./bin/test.sh --rsb run adhoc <existing-demo>
```

Expect Boxy banners for run headers and the concluding run summary. If `boxy` is unavailable on CI, stderr fallback preserved in `_boxy_fallback()` keeps output consistent.

---

## 5. Release & Communication

1. **Versioning**
   - Bump the runner version via `semv` (or the existing RSB release tooling).
   - Mention the BashFX legendary integration in release notes.

2. **Docs Update**
   - Update RSB’s README or developer onboarding docs to explain the new `parts/` build (if adopted) and the Boxy ceremony.
   - Reference this integration guide for future syncs.

3. **Stakeholder Notification**
   - Let RSB maintainers know about the new Boxy dependencies and strict enforcement behavior.
   - Highlight any lanes still pending hydration (e.g., Tina integration) to set expectations.

---

## 6. Post-Integration Follow-ups

- **Tina Integration**: Port the Tina hooks from the legacy script once they are ready.
- **test.conf Consideration**: Decide whether to ship a repo-specific default profile (`TESTSH_PROFILE`) via a config file.
- **CI Enhancements**: Incorporate Boxy logs/artifacts to visualize run summaries in pipelines.

---

## 7. Operator Checklist (Future Me)

When you re-open this project inside `repos/rust/oodx/rsb`, remember:

1. **Rebuild first** – run `./bin/build.sh build`; treat `test.sh` as generated-only.
2. **Set profile** – ensure `TESTSH_PROFILE=rsb` (or pass `--rsb`) before running anything.
3. **Validation sequence** – lint → `run sanity` → `run uat` → `run smoke` → doc overlays → optional module/adhoc spot-check.
4. **Watch for Boxy** – install or expect stderr fallback; override banners indicate when compliance is bypassed.
5. **Communicate** – use semantic commits, update release notes, and point maintainers at this integration guide.

Stick to the checklist and you’ll minimize surprises during reintegration.

---

## Appendix: Quick Reference

| Item                        | Path / Command                                |
|-----------------------------|-----------------------------------------------|
| Runner build script         | `bin/build.sh` (fx-testsh)                    |
| Generated runner            | `test.sh`                                     |
| RSB profile definition      | `_profile_rsb()` in `test.sh`                 |
| Doc overrides               | `_docs_seed_default()` in `parts/60_docs.sh`  |
| Integration guide           | `docs/reference/RSB_TESTSH_INTEGRATION.md`    |
| Sandbox validation script   | `./test.sh lint`, `./test.sh run sanity`      |
