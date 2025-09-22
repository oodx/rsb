# RSB Token Processing

**Status**: ✅ Complete
**Module**: `rsb::token`
**Ported From**: XStream token system
**Safety**: UNICODE-SAFE (with ASCII-first validation rules)

## Overview

The token module provides generic key=value token processing with optional namespace support. This is the low-level, namespace-agnostic foundation that was extracted from XStream to enable broader reuse.

## Core Concepts

### Token Format
Tokens follow a strict format: `key=value` or `namespace:key=value`

- **Semicolon-separated**: Multiple tokens separated by `;`
- **Quote stripping**: Values can be quoted with `"` or `'` - quotes are automatically stripped
- **Namespace support**: Hierarchical namespaces using dot notation (e.g., `db.config:host="localhost"`)
- **Strict validation**: No spaces around `=` or before `;`; no spaces in keys or namespaces

### Examples
```rust
use rsb::token::{tokenize_string, Token};

// Basic tokens
let tokens = tokenize_string(r#"host="localhost"; port="8080";"#)?;

// With namespaces
let tokens = tokenize_string(r#"db:user="admin"; db:pass="secret";"#)?;

// Hierarchical namespaces
let tokens = tokenize_string(r#"config.db:host="localhost"; auth.session:token="xyz";"#)?;
```

## API Surface

### Core Functions

#### `tokenize_string(input: &str) -> TokenResult<Vec<Token>>`
Parse a semicolon-separated token string into individual tokens.

**Examples**:
```rust
let tokens = tokenize_string(r#"host="localhost"; db:user="admin";"#)?;
assert_eq!(tokens.len(), 2);
assert_eq!(tokens[0].key, "host");
assert_eq!(tokens[0].value, "localhost"); // quotes stripped
```

#### `is_token_streamable(input: &str) -> bool`
Validate if a string can be successfully tokenized (more efficient than full parsing).

**Examples**:
```rust
assert!(is_token_streamable(r#"valid="token";"#));
assert!(!is_token_streamable("invalid token")); // missing =
```

### Types

#### `Token`
Represents a key=value pair with optional namespace.

**Fields**:
- `namespace: Option<Namespace>` - Optional hierarchical namespace
- `key: String` - The key part
- `value: String` - The value part (quotes stripped)

**Methods**:
- `Token::simple(key, value)` - Create without namespace
- `Token::with_namespace(ns, key, value)` - Create with namespace
- `to_string()` - Convert back to token format

#### `Namespace`
Represents hierarchical namespaces with dot separation.

**Methods**:
- `Namespace::from_string("db.config")` - Parse from string
- `to_string()` - Convert back to string representation

#### `TokenStreamable` trait
Trait for types that can be converted to/from token streams.

**Implemented for**: `str`, `String`

```rust
let input = r#"host="localhost";"#;
let tokens = input.tokenize()?;
assert!(input.validate().is_ok());
```

### Token Collections (`rsb::token::bucket`)

#### `TokenBucket`
Organizes tokens by namespace with different access patterns.

**Modes**:
- `BucketMode::Flat` - Simple HashMap: namespace -> key -> value
- `BucketMode::Tree` - Hierarchical tree with parent-child relationships
- `BucketMode::Hybrid` - Both flat data and tree index for maximum flexibility

**Methods**:
- `TokenBucket::from_str(input, mode)` - Parse and collect in one step
- `TokenBucket::from_tokens(tokens, mode)` - Collect pre-parsed tokens
- `TokenBucket::from_str_with_delimiter(input, delim, mode)` - Same as `from_str` but lets you override the token separator (`,` for CSV-style streams, etc.)
- `TokenBucket::insert(ns, key, value)` - Insert/update a value while preserving the active namespace semantics
- `get_namespace(ns)` - Get key-value map for namespace
- `get_children(ns)` - Get direct child namespaces (Tree/Hybrid)
- `get_siblings(ns)` - Get namespaces at same level
- `get_all_under(prefix)` - Get all descendant namespaces

#### `collect_tokens(tokens, mode) -> TokenBucket`
Collects tokens with namespace switching logic:
- `ns=namespace` tokens switch active namespace for subsequent tokens
- Explicit namespaces (`ns:key=value`) override active namespace
- Default namespace is "global"

**Example**:
```rust
let bucket = TokenBucket::from_str("item=val1; ns=animals; dog=fido; cat=fluffy;", BucketMode::Flat)?;
// item goes to "global", dog and cat go to "animals" namespace
```

### Format Utilities (`rsb::token::format`)

#### Quoting and Escaping
- `quote_token(value)` - Add quotes if not already quoted
- `unquote_token(value)` - Remove quotes if present
- `escape_token(value)` - Escape special characters (newlines, quotes, etc.)
- `unescape_token(value)` - Reverse escaping
- `normalize_token(value)` - Trim whitespace and unquote

#### Joining and Formatting
- `join_tokens(values, separator)` - Join multiple values
- `join_quoted_tokens(values, separator)` - Quote each value and join
- `format_token(token)` - Format token as key=value string
- `format_token_table(tokens)` - Create aligned table display
- `token_to_string(token)` / `tokens_to_string(&[Token])` - Render helpers used internally by the join/format routines

#### Display Utilities
- `trim_token(value)` - Remove leading/trailing whitespace
- `pad_token(value, width, char)` - Pad to specified width
- `truncate_token(value, max_len)` - Truncate with ellipsis

### Core Utilities (`rsb::token::utils`)

#### Helper Functions
- `make_token(key, value)` - Create simple token
- `make_namespaced_token(namespace, key, value)` - Create namespaced token
- `extract_namespace_tokens(tokens, namespace)` - Filter tokens by namespace
- `get_namespace_names(tokens)` - Extract unique namespace names
- `tokens_to_string(tokens)` - Convert tokens back to string format

### Macro Helpers (`rsb::token::macros`)
- `tokenize!(expr)` — convenience front door around `tokenize_string`; panics on error (perfect for demos/tests).
- `token_validate!(expr)` — asserts a token stream is well-formed, exiting with clear diagnostics otherwise.

## Validation Rules

### Format Requirements
- **Tokens separated by semicolons**: `token1; token2; token3`
- **Key=value format**: Each token must contain exactly one `=`
- **No spaces around equals**: `key=value` ✅, `key = value` ❌
- **No trailing spaces**: `token;` ✅, `token ;` ❌
- **Spaces allowed after semicolon**: `token1; token2` ✅

### Key and Namespace Rules
- **No spaces in keys**: `valid_key` ✅, `invalid key` ❌
- **No spaces in namespaces**: `db.config` ✅, `db config` ❌
- **Hierarchical namespaces**: Use dots (`.`) as separators
- **Namespace separator**: Use colon (`:`) between namespace and key

### Value Rules
- **Quoted values**: Quotes are stripped automatically
- **Mixed quoting**: `"double"` and `'single'` both supported
- **Spaces allowed in values**: `key="value with spaces"` ✅
- **Empty values allowed**: `key=""` or `key=` both valid

## Error Handling

### `TokenError` enum
- `EmptyInput` - Input is empty or whitespace-only
- `MalformedToken { token, reason }` - Specific formatting error
- `ParseError { reason }` - General parsing failure

### Comprehensive Error Messages
```rust
// Examples of validation errors:
tokenize_string("bad_token")               // "missing '=' separator"
tokenize_string("key = value")             // "space before '=' not allowed"
tokenize_string("my namespace:key=val")    // "spaces not allowed in namespace"
```

## Integration Examples

### Basic Usage
```rust
use rsb::token::{tokenize_string, TokenStreamable};

// Parse from string
let tokens = tokenize_string(r#"host="localhost"; port="8080";"#)?;

// Use trait methods
let input = r#"db:user="admin"; db:pass="secret";"#;
let tokens = input.tokenize()?;
assert!(input.validate().is_ok());
```

### Namespace Filtering
```rust
use rsb::token::utils::{tokenize_string, extract_namespace_tokens};

let tokens = tokenize_string(r#"
    host="localhost";
    db:user="admin";
    db:pass="secret";
    auth:token="xyz";
"#)?;

// Extract database config
let db_tokens = extract_namespace_tokens(&tokens, Some("db"));
assert_eq!(db_tokens.len(), 2);

// Extract global config (no namespace)
let global_tokens = extract_namespace_tokens(&tokens, None);
assert_eq!(global_tokens.len(), 1);
```

### TokenBucket Organization
```rust
use rsb::token::{TokenBucket, BucketMode};

// Namespace switching example
let bucket = TokenBucket::from_str("item=val1; ns=animals; dog=fido; cat=fluffy;", BucketMode::Flat)?;

// Access organized data
assert_eq!(bucket.data["global"]["item"], "val1");
assert_eq!(bucket.data["animals"]["dog"], "fido");
assert_eq!(bucket.data["animals"]["cat"], "fluffy");

// Hierarchical navigation (Tree mode)
let bucket = TokenBucket::from_str("a:k1=v1; a.b:k2=v2; a.b.c:k3=v3;", BucketMode::Tree)?;
let children = bucket.get_children("a.b");
assert!(children.contains(&"a.b.c".to_string()));
```

### Format Helpers
```rust
use rsb::token::format::{quote_token, escape_token, format_token_table};

// Quote handling
assert_eq!(quote_token("hello world"), "\"hello world\"");
assert_eq!(quote_token("\"already quoted\""), "\"already quoted\"");

// Escape special characters
assert_eq!(escape_token("line1\nline2"), "line1\\nline2");

// Table formatting
let tokens = vec![Token::simple("host", "localhost")];
let table = format_token_table(&tokens);
// Creates aligned table output
```

### Round-trip Processing
```rust
use rsb::token::utils::{make_token, make_namespaced_token, tokens_to_string};

// Create tokens programmatically
let tokens = vec![
    make_token("host", "localhost"),
    make_namespaced_token("db", "user", "admin"),
    make_namespaced_token("db", "pass", "secret"),
];

// Convert back to string
let output = tokens_to_string(&tokens);
// Result: "host=localhost; db:user=admin; db:pass=secret"

// Parse again to verify round-trip
let parsed = tokenize_string(&output)?;
assert_eq!(parsed.len(), 3);
```

## Design Notes

### Relationship to XStream
This module provides the **generic foundation** that XStream builds upon:
- **RSB Token**: Generic key=value parsing with namespaces
- **XStream TokenBucket**: Adds bucket semantics, stream operations, and visualization

### Safety Characteristics
- **UNICODE-SAFE**: Handles Unicode content within quoted values
- **ASCII-FIRST**: Keys and namespaces validated with ASCII-only rules
- **Fail-fast**: Strict validation with comprehensive error messages

### Performance Notes
- `is_token_streamable()` is optimized for validation-only use cases
- Namespace filtering creates borrowed references (no cloning)
- Quote stripping handled efficiently during parsing

## Module Structure (MODULE_SPEC compliant)

- `types.rs` - Core Token, Namespace, and TokenError types
- `parse.rs` - Tokenization and validation logic
- `bucket.rs` - TokenBucket collection with namespace organization
- `error.rs` - TokenBucketError types and results
- `format.rs` - Format utilities (quote, escape, join, pad, etc.)
- `utils.rs` - Curated helper functions for explicit import
- `helpers.rs` - Internal implementation details
- `macros.rs` - Module-owned macros (placeholder)
- `mod.rs` - Orchestrator with public API surface

## Testing

Comprehensive test coverage includes:
- Basic tokenization and quote stripping
- Namespace parsing and validation
- Error cases and edge conditions
- Spacing rule enforcement
- Round-trip processing
- Trait implementation validation

Run tests: `cargo test --lib token`

<!-- feat:tokens -->

_Generated by bin/feat.py --update-doc._

* `src/token/bucket.rs`
  - enum BucketMode (line 12)
  - struct TokenBucket (line 29)
  - fn new (line 40)
  - fn from_tokens (line 63)
  - fn from_str (line 77)
  - fn insert (line 98)
  - fn get_children (line 154)
  - fn get_all_under (line 165)
  - fn get_namespace (line 176)
  - fn get_siblings (line 191)
  - fn collect_tokens (line 235)

* `src/token/error.rs`
  - enum TokenBucketError (line 7)
  - type TokenBucketResult (line 48)

* `src/token/format.rs`
  - fn quote_token (line 20)
  - fn unquote_token (line 43)
  - fn escape_token (line 70)
  - fn unescape_token (line 93)
  - fn join_tokens (line 115)
  - fn join_quoted_tokens (line 130)
  - fn trim_token (line 149)
  - fn normalize_token (line 164)
  - fn pad_token (line 179)
  - fn format_token (line 200)
  - fn format_token_table (line 228)
  - fn truncate_token (line 290)

* `src/token/macros.rs`
  - macro token_validate! (line 24)
  - macro tokenize! (line 39)

* `src/token/mod.rs`
  - pub use bucket::{collect_tokens, BucketMode, TokenBucket} (line 66)
  - pub use error::{TokenBucketError, TokenBucketResult} (line 67)
  - pub use parse::{is_token_streamable, tokenize_string} (line 68)
  - pub use types::* (line 69)

* `src/token/parse.rs`
  - fn tokenize_string (line 40)
  - fn is_token_streamable (line 106)

* `src/token/types.rs`
  - struct Namespace (line 14)
  - fn new (line 24)
  - fn from_string (line 32)
  - fn from_str_with_delimiter (line 37)
  - struct Token (line 66)
  - fn new (line 77)
  - fn with_namespace (line 86)
  - fn simple (line 95)
  - trait TokenStreamable (line 131)
  - enum TokenError (line 140)
  - type TokenResult (line 164)
  - pub use super::error::{TokenBucketError, TokenBucketResult} (line 167)

* `src/token/utils.rs`
  - pub use super::parse::{is_token_streamable, tokenize_string} (line 8)
  - pub use super::types::* (line 9)
  - fn make_token (line 22)
  - fn make_namespaced_token (line 37)
  - fn extract_namespace_tokens (line 55)
  - fn get_namespace_names (line 80)
  - fn token_to_string (line 102)
  - fn tokens_to_string (line 119)

<!-- /feat:tokens -->
