# Token Namespace Concept

⚠️ **CRITICAL AMENDMENT**: This document contains **architectural errors** discovered during implementation. See **[TOKEN_CONCEPT_AMENDMENT.md](./TOKEN_CONCEPT_AMENDMENT.md)** for critical corrections to the Token/Meteor/MeteorShower hierarchy before using this specification.

## Overview

This document defines the unified namespace and key strategy pattern for RSB/XStream token systems, providing a consistent data format across different data access methods while maintaining clear separation of concerns between data transport and semantic validation.

## Pattern Structure

### Full Pattern: `ctx:namespace:key=value`

```rust
// Complete meteor data format
"app:ui.widgets:button[0]=submit"
"user:preferences:theme=dark"
"system:env:PATH=/usr/bin"
"file1:config.database:host=localhost"

#general pattern
CONTEXT:NAMESPACE:KEY=VALUE
```

### Components

1. **Context** (`ctx`): Origin/source identifier
2. **Namespace** (`namespace`): Hierarchical organization within context
3. **Key** (`key`): Individual data identifier with optional indexing

### MeteorShower Organization

MeteorShower provides the primary storage with cross-context indexing:

```rust
// Input meteor stream:
"app:ui.widgets:button[0]=submit,list[]=item1; app:config:theme=dark"

// MeteorShower structure:
MeteorShower {
    meteors: [
        Meteor {
            context: "app",
            tokens: [
                (ui.widgets, button[0]=submit),
                (ui.widgets, list[]=item1),
                (config, theme=dark)
            ]
        }
    ],
    // Storage indexing for fast access:
    contexts: {
        "app" → {
            "ui.widgets" → {
                "button__i_0" → "submit",
                "list__i_APPEND" → "item1"
            },
            "config" → {
                "theme" → "dark"
            }
        }
    }
}

// Full meteor data format examples:
"app:ui.widgets:button__i_0=submit"
"app:ui.widgets:list__i_APPEND=item1"
"app:config:theme=dark"
```

### Multi-Context Example
```rust
// Multiple contexts stored in MeteorShower:
AppContext {
    context: "app",
    data: { "ui.widgets" → { "button__i_0" → "submit" } }
}

UserContext {
    context: "user",
    data: { "preferences" → { "theme" → "dark" } }
}

SystemContext {
    context: "system",
    data: { "env" → { "PATH" → "/usr/bin" } }
}

// All stored together in single MeteorShower with cross-context indexing
```

## Context Rules

### Default Context: `app`
```rust
// These are equivalent:
"ui.widgets:button=submit"         // Implied app context
"app:ui.widgets:button=submit"     // Explicit app context
```

### Context Assignment
- **Implicit**: Token streams default to `app` context
- **Explicit**: `ctx=context_name` declares context for entire stream
- **Scope**: One context per token stream (no mixing within stream)

### Context Examples
```rust
// Local application context (default)
"app:ui.layout:grid[2,3]=cell"

// User-owned data/preferences
"user:settings:theme=dark"

// System/environment data
"system:env:HOME=/home/user"

// File-sourced data
"file1:config:database.host=localhost"
"file2:layout:widgets[0]=menu"

// Remote system data (sandboxed)
"remote1:metrics:cpu=85"
"remote2:status:connected=true"
```

## Namespace Rules

### Hierarchical Structure
- **Delimiter**: Dot notation (`.`) for hierarchy
- **Depth Warning**: Compiler warnings beyond 3 levels (`a.b.c`)
- **Depth Limit**: 4 levels maximum (`a.b.c.d`) before rejection

### Namespace Examples
```rust
// Recommended depth (1-3 levels)
"app:ui:button=submit"              // 2 levels
"app:ui.widgets:list[0]=item"       // 3 levels
"user:config.theme:dark=true"       // 3 levels

// Warning level (4 levels)
"app:ui.widgets.buttons:primary=blue"  // ⚠️ Warning

// Error level (5+ levels)
"app:ui.widgets.buttons.styles:hover=red"  // ❌ Error
```

### Namespace Switching in Streams
```rust
// Within single context, namespace can switch
"ctx=app; ns=ui.widgets; button[0]=submit; list[1]=item; ns=config; theme=dark;"

// Resolves to:
"app:ui.widgets:button[0]=submit"
"app:ui.widgets:list[1]=item"
"app:config:theme=dark"
```

## Key Rules

### Basic Key Patterns
```rust
// Simple assignment
"key=value"                    // No indexing

// Named indexing
"user[name]=alice"             // → "user__name=alice"
"config[database]=host"        // → "config__database=host"

// Numeric indexing
"list[0]=item1"                // → "list__i_0=item1"
"grid[2,3]=cell"               // → "grid__i_2_3=cell"

// Append operation
"queue[]=task"                 // → "queue__i_APPEND=task"
```

### Bracket Transformation Rules

#### Empty Brackets: Append Semantics
```rust
"list[]=item"     → "list__i_APPEND=item"
"queue[]=task"    → "queue__i_APPEND=task"
```

#### Single Bracket: Named or Numeric
```rust
// String content → named indexing (no hint)
"user[name]=alice"    → "user__name=alice"
"field[email]=test"   → "field__email=test"

// Numeric content → indexed access (with hint)
"items[0]=first"      → "items__i_0=first"
"rows[42]=data"       → "rows__i_42=data"
```

#### Multi-Bracket: Coordinate System
```rust
// 2D coordinates
"grid[2,3]=cell"      → "grid__i_2_3=cell"
"board[x,y]=piece"    → "grid__i_x_y=piece"

// 3D coordinates (warning level)
"matrix[1,2,3]=val"   → "matrix__i_1_2_3=val"  // ⚠️ Warning

// 4D+ coordinates (ugly but allowed)
"tensor[a,b,c,d]=data" → "tensor__i_a_b_c_d=data"  // 😬 Ugly
```

### Type Hinting in Dunder Pattern
```rust
// Numeric brackets get index hint (__i_)
"list[0]=item"        → "list__i_0=item"
"grid[2,3]=cell"      → "grid__i_2_3=cell"

// String brackets get direct dunder (no hint)
"person[steve]=data"  → "person__steve=data"
"cache[user]=alice"   → "cache__user=alice"
```

### Structured Value Support
```rust
// Simple values
"key=value"                    // String value
"count=42"                     // Numeric string value

// Quoted values (preserve spaces, special chars)
"message='hello world'"        // Single quotes
"path=\"/home/user/file\""     // Double quotes
"json='{\"name\":\"alice\"}''" // Escaped JSON

// Multi-line values (implementation dependent)
"config='line1\nline2\nline3'" // Newline preservation
"script='#!/bin/bash\necho hi'" // Shell script content
```

## Implied Rules

### Context Implications
1. **Default Context**: All tokens without explicit context belong to `app`
2. **Context Isolation**: Different contexts cannot access each other's data directly
3. **Privilege Boundaries**: Context determines access permissions (system > user > app > remote)

### Namespace Implications
1. **Routing**: Namespace determines which consumer/manager handles the data
2. **Hierarchy**: Dot notation implies parent-child relationships
3. **Folding**: Related namespaces can be grouped by consumers

### Key Implications
1. **Storage**: All keys stored as flattened strings in MeteorShower storage
2. **Semantics**: Bracket notation provides hints, consumers enforce meaning
3. **Extensibility**: Dunder pattern allows unlimited extension types

## Deferred Situations

### Consumer Responsibility
The token system provides **data transport**, not **semantic validation**. Consumers are responsible for:

1. **Type Checking**: Ensuring data types match expected schema
2. **Semantic Validation**: Rejecting invalid operations for their domain
3. **Index Management**: Handling append operations and coordinate systems
4. **Access Control**: Enforcing context-based permissions
5. **Value Parsing**: Interpreting quoted strings, JSON, multi-line content

### Mixed Data Scenarios
```rust
// MeteorShower accepts mixed semantics:
"ui.layout:grid[]=item1; ui.layout:grid[2,3]=item2; ui.layout:grid[name]=item3;"

// Consumer (GridLayoutManager) decides:
// - grid[] → Error("Grid requires coordinates")
// - grid[2,3] → OK (valid coordinates)
// - grid[name] → Error("Grid doesn't support named keys")
```

### Cross-Context References
```rust
// Variable expansion allows cross-context access:
"app:config:db_host=${user:settings:database.host}"
"app:ui:theme=${system:env:DEFAULT_THEME}"

// But direct writes are context-isolated:
"system:env:PATH=modified"  // May be rejected by system context handler
```

<!--
### Cross-Context Relationships (Advanced - BACKLOG)
Potential future feature for distributed access control:
- Relationship declarations: app:user:rel=app2,app3
- Permission types: rel[read], rel[write], rel[admin]
- Wildcard access: rel=* for public namespaces
- Access validation through RelationshipManager
Requires deeper investigation of actual use cases.
-->

### Current Extensions (Implemented)
- **BracketTransform Trait**: Extensible bracket notation system with caching
- **MeteorShower Collection**: Object-oriented collection with indexed queries (`by_context()`, `find()`)
- **Inverse Parsing**: Reconstruction of bracket notation from flat keys
- **Storage & Interchange**: Primary storage (MeteorShower) with serialized interchange format (StorageData)

### Future Extension Points
Future extensions may include:
- **Operations**: `counter[++]=1`, `list[--]=item` (conceptual only)
- **Queries**: `cache[?]=key` for existence checks
- **Transformations**: Automatic type conversion hints
- **Validation**: Schema enforcement at namespace level

## Implementation Notes

### Collection Types

#### MeteorShower Storage (Primary)
- Cross-context indexed storage with object-oriented meteor access
- Hybrid storage: flat `HashMap<String, String>` + hierarchical `TreeNode` index
- Flat storage for O(1) key-value access with canonical keys (`namespace:key`)
- Tree index for O(log n) directory operations and path traversal
- Bracket notation transformed to dunder at parse time
- Consumer folding reconstructs semantic structures

#### StorageData Format (Serialized Interchange)
- Vector of fully-qualified Meteor objects
- Indexed lookups by context and namespace for performance
- Query methods: `by_context()`, `by_context_namespace()`, `find()`
- Discovery methods: `contexts()`, `namespaces_in_context()`
- Supports complex meteor data queries

### Parser Transformation
```rust
// Forward transformation pipeline:
"app:ui.widgets:list[0]=item"
→ context="app", namespace="ui.widgets", key="list__i_0", value="item"

// Inverse parsing (reconstruction from flat keys):
"list__i_0" → "list[0]"  // Via BracketTransform trait
"grid__i_2_3" → "grid[2,3]"
"user__name" → "user[name]"
```

### Consumer Pattern
```rust
// Consumers implement domain-specific logic:
impl LayoutManager {
    fn consume_tokens(&mut self, shower: &MeteorShower) -> Result<()> {
        // 1. Check context permissions
        // 2. Route by namespace
        // 3. Parse bracket hints from keys
        // 4. Enforce semantic rules
        // 5. Update internal state
    }
}
```

This design provides a unified data format while maintaining flexibility for different use cases and clear boundaries between data transport and semantic interpretation.
