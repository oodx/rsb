# RSB Documentation Strategy

**Date:** 2025-09-30
**Status:** PROPOSED
**Context:** Macro migration complete, documentation consolidation needed

## Vision

Create `rsbdoc` - a self-hosted RSB CLI tool that provides instant access to all RSB and development documentation from the terminal.

## Motivation

**Problem:**
- Documentation scattered across repos
- Need to navigate directories to find guides
- Context switching breaks flow
- No unified way to access RSB features, architecture, and process docs

**Solution:**
- Single CLI: `rsbdoc <topic> <doc>`
- Built WITH RSB (dogfooding the prelude!)
- Reads from canonical locations
- Beautiful terminal output (like test.sh)

## Command Structure

### Core Pattern
```bash
rsbdoc <topic> <document>
```

**Short names, lowercase, easy to type:**

```bash
# Features (RSB-specific)
rsbdoc feat bash           # FEATURES_BASH.md
rsbdoc feat global         # FEATURES_GLOBAL.md
rsbdoc feat list           # List all features

# Architecture
rsbdoc arch rebel          # REBEL architecture
rsbdoc arch bashfx         # BashFX standards
rsbdoc arch rsb            # RSB_ARCH.md

# Process documentation
rsbdoc proc modules        # MODULE_SPEC.md
rsbdoc proc hub            # HOWTO_HUB.md
rsbdoc proc testing        # Testing guides

# Concepts
rsbdoc concept meteor      # Meteor pattern docs
rsbdoc concept strings     # String handling concepts
rsbdoc concept testsh      # test.sh concepts

# Meta/discovery
rsbdoc list                # List all topics
rsbdoc arch list           # List arch docs
rsbdoc proc list           # List process docs
```

## Documentation Sources

### Primary: Brain (Canonical Home)
```
$BRAIN_HOME/dev/
├── architecture/     → rsbdoc arch <doc>
├── concepts/         → rsbdoc concept <doc>
├── proccess/         → rsbdoc proc <doc>
└── projects/         → rsbdoc project <doc>
```

**Default:** `BRAIN_HOME=~/repos/docs/brain`

### Secondary: RSB Repository
```
$RSB_HOME/docs/tech/features/
└── FEATURES_*.md     → rsbdoc feat <name>
```

**Default:** `RSB_HOME=<detected from rsbdoc binary location>`

## Implementation Strategy

### Phase 1: Core CLI (bin/rsbdoc.rs)
- Single binary using `rsb::prelude::*`
- Demonstrates RSB's own CLI features:
  - `Args` parsing
  - `dispatch!` or pattern matching
  - `colored!` output
  - `read_file()` from fs module
  - Global variable expansion
- Command routing based on directory structure
- Auto-discovery of available docs

### Phase 2: Deployment (bin/deploy-rsbdoc.sh)
- Adapted from `bin/deploy.sh` (boxy pattern)
- Build: `cargo build --release --bin rsbdoc`
- Deploy to: `~/.local/lib/odx/rsbdoc` (binary)
- Symlink: `~/.local/bin/odx/rsbdoc` → lib
- XDG-compliant installation

### Phase 3: Live Reading (No Embedding)
- Read directly from filesystem
- Supports instant doc updates (no rebuild)
- Expects:
  - `$BRAIN_HOME/dev/` for canonical docs
  - `$RSB_HOME/docs/tech/features/` for RSB features
- Graceful fallback if paths missing

## Technical Details

### File Discovery
```rust
// Pseudo-code pattern
fn find_doc(topic: &str, doc: &str) -> Option<String> {
    let brain = env::var("BRAIN_HOME").unwrap_or_else(|| format!("{}/repos/docs/brain", home_dir()));
    let rsb = env::var("RSB_HOME").unwrap_or_else(|| detect_rsb_home());

    match topic {
        "feat" => find_in(&format!("{}/docs/tech/features", rsb), &format!("FEATURES_{}.md", doc.to_uppercase())),
        "arch" => find_in(&format!("{}/dev/architecture", brain), doc),
        "proc" => find_in(&format!("{}/dev/proccess", brain), doc),
        "concept" => find_in(&format!("{}/dev/concepts", brain), doc),
        _ => None
    }
}
```

### Display Format
- Colorized markdown rendering (subset)
- Headers in bold/color
- Code blocks with syntax highlighting (optional)
- Bullet lists with glyphs
- Similar ceremony to `test.sh docs` output

### Auto-completion Support
- Generate shell completions for discovered docs
- `rsbdoc --completions bash > ~/.local/share/bash-completion/completions/rsbdoc`

## Benefits

1. **Self-Hosting:** RSB CLI built WITH RSB
2. **Dogfooding:** Real-world usage of prelude features
3. **Live Updates:** Edit doc → instantly available
4. **Canonical Source:** Single location for all dev docs
5. **Fast Access:** No browser, no file navigation
6. **Beautiful Output:** Terminal ceremony like test.sh
7. **Extensible:** Easy to add new topics/docs

## Migration Path

**Current State:**
- Docs in `rsb/docs/tech/{features,reference,development}`
- Brain docs in `~/repos/docs/brain/dev/`

**Target State:**
- Brain becomes canonical home for architecture/process/concepts
- RSB repo keeps FEATURES_*.md (project-specific)
- `rsbdoc` reads from both locations seamlessly

**Migration:**
- Move general docs to Brain
- Keep RSB-specific features in repo
- Link with symlinks if needed during transition

## Future Enhancements

- **Search:** `rsbdoc search "validation macros"`
- **Recent:** `rsbdoc recent` (show recently viewed)
- **Pager Integration:** Pipe to less/bat automatically
- **Web Export:** `rsbdoc serve` for local web view
- **Multi-repo:** Support multiple project feature docs
- **Diff:** `rsbdoc diff feat bash` (compare versions)

## Success Metrics

- [ ] Single command to access any RSB doc
- [ ] Zero context switching to browser/editor
- [ ] Beautiful terminal output
- [ ] Built entirely with RSB prelude
- [ ] Deployed to XDG bin directory
- [ ] Auto-discovers new docs without code changes

## References

- `bin/test.sh docs` - Existing doc viewer pattern
- `bin/deploy.sh` - XDG deployment pattern (boxy)
- Brain structure: `~/repos/docs/brain/dev/`
- RSB features: `docs/tech/features/FEATURES_*.md`

---

**Next Steps:**
1. Create `bin/rsbdoc.rs` with core routing
2. Implement doc discovery and rendering
3. Create `bin/deploy-rsbdoc.sh` deployment script
4. Test with existing documentation
5. Iterate on UX based on usage
