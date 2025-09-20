

This document teaches you how to contribute to development of the RSB Framework, and points you to correct documentation to understand key patterns

## Developers & Contributors

All reference and technical documentation live in `./docs/tech`, work session status information will live in a particular folder within `./sesssion/<session_name>`. All notes and status information go in one of these two locations. The project root must always be kept clean.

I. HOWTO -> understand the ARCHITECTURE principles, patterns and policies


  In `./docs/tech/reference`

    Fundamental:
    ---
    * REBEL Philosophy -> REBEL.md
      - explains the over-arching philosophy and principles that RSB implements
        in its architecture and framework

    * RSB Architecture / Framework -> RSB_ARCH.md
      - architecture standards, patterns, requirements etc.

    * RSB Quick Reference -> RSB_QUICK_REFERENCE.md
      - rapid guide showing features and patterns from a high level
        this document may drift as RSB undergoes rapid development

    Deep Drive:
    ---
    * BashFX Architecture v3 -> BASHFX-v3.md
      - the spiritual predecessor to REBEL/RSB, rich and exact archiecture
        for creating "Legendary Bash Scripts", with various general unix-aligned
        constraints and guidelines (XDG+, Function Ordinality, CLI architecture). 
        This document is heavy; recommend searching for key segments or words, and otherwise only read the whole thing if your task requires in depth understanding. 

    * BashFX and RSB Alignment -> RSB_BASHFX_ALIGN.md
      - high level document showing some of the obvious places where RSB is aligned with  
        BashFX.
    

II. HOWTO -> understand the MODULES and their concepts, features and patterns

  RSB is undergoing a major refactor project to migrate legacy functions and macros from `src/macros.rs` and flat files in `src/<name>.rs` to explicit module packages `src/<name/`; modules that have the own folder under src are generally considered to be refactored. It's important to understand if a module is LEGACY or MODERN. And some modules can be in a chaotic flux between both states.

  For MODERN featursets/concepts, a completed module will have its own `FEATURE_<NAME>.md` file that explains its concepts, api and some usage patterns, found in `./docs/tech/features`

  Further MODERN modules have two sub-states, SPEC_ALIGNED or SPEC_UNALIGNED. This happened because our `MODULE_SPEC.md` (in docs/tech/development) evolved from emerging paterns and some modules were migrated before this spec existed. This spec includes clear requirements on Testing, Preludes, Macros, etc.

  An RSB Module is considered complete if it is both MODERN and SPEC_ALIGNED.

  In `src/_todo` we have stubbed out modules for planned LEGACY to MODERN migration, wehre comments map out potential functions. Additionally some modules are rollbacks from other children projects. 

  Examples.
  * `src/progress` from the `rust/prods/padlokk/cage` project. (MODERN but SPEC_UNALIGNED, e.g. not fully migrated)

  * `src/tokens`  from the `rust/oodx/xstream` project. (MODERN and partially SPEC_ALIGNED)
  * `src/xcls` also from xstream, a legacy port. (MODERN but not SPEC_ALIGNED)

  Why? the MODULE_SPEC defines module completeness and product readiness for usage in other projects. 

  Also to help not make RSB a kitchen-sink framework, we've slowly begun making modules opt-in via Cargo.toml. The MODULE_SPEC pattern helps drive this.

II. HOWTO -> understand the requirements and pattersn for correctly contributing to RSB

  In `./docs/tech/development`

    Fundamental:
    ---
    * Module Specification (MODERN) -> MODULE_SPEC.md

  ## 5.1: Minimal Checklist for Changes
  - [ ] Respect prelude policy (no optional exports in prelude)
  - [ ] Use progressive helpers (`::<module>::basic`) for complex macros
  - [ ] Gate optional features; keep default lean
  - [ ] Add/adjust integration tests via wrappers (sanity/features/uat)
  - [ ] Ensure `bin/test.sh list` shows your new wrappers
  - [ ] Run: `cargo test` (default) and `cargo test --features visuals` if applicable
  - [ ] Run: `./bin/test.sh run smoke` and `./bin/test.sh run all`
  - [ ] Update `prelude::macros` if adding new module‑owned macros
  - [ ] Update feature guides (`FEATURES_<NAME>.md`) if behavior changes



