//! Token processing module for RSB - ported from XStream.
//!
//! Provides generic key=value token processing with optional namespace support.
//! This is the low-level, namespace-agnostic foundation that XStream builds upon.
//!
//! # Architecture
//!
//! - **Types**: Core Token and Namespace types with parsing support
//! - **Parsing**: Robust tokenization with validation and error handling
//! - **Utils**: Curated helper functions for token manipulation
//!
//! # Token Format
//!
//! Tokens follow the format: `key=value` or `namespace:key=value`
//!
//! - Semicolon-separated: `host="localhost"; db:user="admin";`
//! - Quote stripping: `"quoted"` → `quoted`
//! - Namespace support: `db:host="localhost"`
//! - Strict validation: no spaces around `=` or before `;`
//!
//! # Examples
//!
//! ## Basic Tokenization
//! ```
//! use rsb::token::tokenize_string;
//!
//! let tokens = tokenize_string(r#"host="localhost"; port="8080";"#).unwrap();
//! assert_eq!(tokens.len(), 2);
//! assert_eq!(tokens[0].key, "host");
//! assert_eq!(tokens[0].value, "localhost"); // quotes stripped
//! ```
//!
//! ## Namespace Support
//! ```
//! use rsb::token::{tokenize_string, utils::extract_namespace_tokens};
//!
//! let tokens = tokenize_string(r#"host="localhost"; db:user="admin"; db:pass="secret";"#).unwrap();
//! let db_tokens = extract_namespace_tokens(&tokens, Some("db"));
//! assert_eq!(db_tokens.len(), 2);
//! ```
//!
//! ## Validation
//! ```
//! use rsb::token::{is_token_streamable, TokenStreamable};
//!
//! assert!(is_token_streamable(r#"valid="token";"#));
//! assert!(!is_token_streamable("invalid token")); // no =
//!
//! // Trait-based validation
//! let input = r#"host="localhost";"#;
//! assert!(input.validate().is_ok());
//! ```

// Internal modules
mod error;
mod helpers;
mod parse;
mod types;

// Public modules following MODULE_SPEC
pub mod bucket;
pub mod format;
pub mod utils;

// Re-export main API surface (orchestrator pattern)
pub use bucket::{collect_tokens, BucketMode, TokenBucket};
pub use error::{TokenBucketError, TokenBucketResult};
pub use parse::{is_token_streamable, tokenize_string};
pub use types::*;

// Keep module-owned macros compiled/included
pub mod macros;
