# Hybrid Storage Architecture

## Overview

Meteor uses a **hybrid storage model** that combines flat canonical keys with hierarchical tree indexing. This design provides both O(1) direct access and efficient hierarchical queries while maintaining the filesystem-like semantics users expect.

## Architecture Components

### 1. Context Isolation

Each context operates as a completely separate storage system:

```rust
struct MeteorEngine {
    contexts: HashMap<String, ContextStorage>,  // Isolated storage per context
    current_context: Context,                   // Cursor state
}
```

**Context Examples:**
- `app` - Application data, UI state, configuration
- `user` - User preferences, profile, settings
- `system` - System configuration, logs, metadata

**Complete isolation:** No data sharing between contexts.

### 2. Context Storage Structure

Each context contains hybrid storage:

```rust
struct ContextStorage {
    flat_data: HashMap<String, String>,      // Canonical key-value storage
    tree_index: TreeMap<String, TreeNode>,   // Hierarchical navigation index
}
```

### 3. Flat Canonical Keys

All data is stored as flat key-value pairs using canonical addressing:

```rust
// Format: "namespace:path.to.key"
flat_data: {
    "main:user.index" → "jose",              // Default value for user
    "main:user.settings.theme" → "dark",     // Nested hierarchical data
    "main:user.settings.lang" → "en",
    "ui:button.color" → "blue",              // Different namespace
}
```

**Key Properties:**
- **Canonical:** Every piece of data has exactly one address
- **Flat:** No nested data structures, just string keys
- **Efficient:** O(1) direct access by canonical key
- **Atomic:** Each key-value pair is independent

### 4. Tree Navigation Index

The tree index provides hierarchical navigation that points into the flat storage:

```rust
enum TreeNode {
    Directory(HashMap<String, TreeNode>),    // Contains child nodes
    File(String),                           // Points to canonical key in flat_data
}

// Example tree structure:
tree_index: {
    "main": Directory {
        "user": Directory {
            "index": File("main:user.index"),           // → flat_data["main:user.index"]
            "settings": Directory {
                "theme": File("main:user.settings.theme"), // → flat_data["main:user.settings.theme"]
                "lang": File("main:user.settings.lang"),
            }
        }
    },
    "ui": Directory {
        "button": Directory {
            "color": File("ui:button.color"),           // → flat_data["ui:button.color"]
        }
    }
}
```

**Tree Properties:**
- **Navigation:** Efficient hierarchical traversal and queries
- **Metadata:** Tree is pure index, not source of truth
- **Rebuildable:** Can be reconstructed from flat keys if corrupted
- **Filesystem Semantics:** Directories contain, files ARE values

## Filesystem Model

### Files vs Directories

**Files (Leaf Nodes):**
- Contain actual data values
- Cannot have children
- Point to canonical keys in flat storage

**Directories (Branch Nodes):**
- Contain child nodes (files or other directories)
- Cannot have values directly
- Organize hierarchical structure

### The `.index` Pattern

Directories can have default scalar values through the `.index` pattern:

```rust
// Storage:
"main:user.index" → "jose"              // User's default value
"main:user.settings.theme" → "dark"     // User's hierarchical data

// Tree:
"user": Directory {
    "index": File("main:user.index"),         // Default value
    "settings": Directory { ... }             // Hierarchical content
}

// API:
engine.get("user.index")      → "jose"       // Explicit access
engine.get_default("user")    → "jose"       // Convenience method
```

## Query Operations

### Direct Access (O(1))

```rust
// Direct lookup by canonical key
engine.get("user.settings.theme")
// → parse to canonical: "main:user.settings.theme"
// → flat_data["main:user.settings.theme"] → "dark"
```

### Hierarchical Queries (O(log n))

```rust
// Tree traversal for namespace queries
engine.find("user.settings.*")
// → tree_index["main"]["user"]["settings"].children()
// → ["theme", "lang"] → map to canonical keys → lookup values
```

### Existence Checks

```rust
// Check if path exists as directory
engine.is_directory("user.settings") → true

// Check if path exists as file
engine.is_file("user.settings.theme") → true

// Check if path has default value
engine.has_default("user") → true (user.index exists)
```

## Collection and Routing

### Input Processing

Meteor routes incoming data based on path structure:

```rust
// Input: "user.settings.theme=dark"
// 1. Parse path: ["user", "settings", "theme"]
// 2. Generate canonical key: "main:user.settings.theme"
// 3. Store in flat_data: "main:user.settings.theme" → "dark"
// 4. Update tree_index: main/user/settings/theme → File("main:user.settings.theme")
```

### Path Resolution

```rust
// Path types and their canonical forms:
"user"                    → "main:user.index"           (default value)
"user.settings"           → "main:user.settings.index"  (directory default)
"user.settings.theme"     → "main:user.settings.theme"  (explicit file)
```

## Type Safety and Conflicts

### File vs Directory Enforcement

```rust
// Valid:
user.index='jose'              // user as directory with default value
user.settings.theme='dark'     // user has hierarchical content

// Invalid:
user='jose'                    // user as scalar
user.settings.theme='dark'     // ERROR: user is file, cannot have children
```

### Explicit Type Declaration (Future)

Potential notation for explicit type control:

```rust
user{}                    // Explicitly declare as directory/object
user[]                    // Explicitly declare as array
user                      // Scalar/file (no children allowed)
```

## Performance Characteristics

### Storage Efficiency

- **Flat Storage:** O(1) space per key-value pair
- **Tree Index:** O(n) space where n = unique path segments
- **Memory Usage:** Minimal duplication (tree stores only addresses)

### Query Performance

- **Direct Access:** O(1) hash lookup
- **Hierarchical Queries:** O(log n) tree traversal + O(k) result collection
- **Prefix Matching:** O(log n) to find subtree + O(m) to collect results

### Update Performance

- **Insert:** O(log n) to update tree + O(1) to store data
- **Delete:** O(log n) to update tree + O(1) to remove data
- **Batch Operations:** Amortized efficiency for multiple related updates

## Implementation Benefits

### 1. Best of Both Worlds
- **Flat storage:** Simple, efficient, atomic operations
- **Tree indexing:** Hierarchical queries, familiar filesystem semantics

### 2. Data Integrity
- **Single source of truth:** Flat storage is canonical
- **Index consistency:** Tree can be rebuilt if corrupted
- **Atomic operations:** Each key-value pair is independent

### 3. Query Flexibility
- **Direct access:** When you know the exact path
- **Hierarchical traversal:** When you need to explore structure
- **Pattern matching:** Efficient prefix and wildcard queries

### 4. Scalability
- **Context isolation:** Independent scaling per context
- **Efficient indexing:** Tree operations scale logarithmically
- **Memory optimization:** Minimal overhead for sparse hierarchies

## Migration from Previous Architecture

### Deprecated Components
- ~~Simple nested HashMap storage~~
- ~~MeteorShower as primary container~~
- ~~Direct hierarchical value storage~~

### New Components
- ✅ Context-isolated hybrid storage
- ✅ Flat canonical key addressing
- ✅ Tree navigation index
- ✅ Filesystem semantic model

This architecture provides the foundation for efficient, scalable, and intuitive hierarchical data management while maintaining the collection-oriented philosophy of the Meteor system.