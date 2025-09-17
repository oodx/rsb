//! Token bucket functionality - ported from XStream.
//!
//! Provides TokenBucket for organizing tokens into namespace-aware collections
//! with different organizational modes (Flat, Tree, Hybrid).

use super::error::{TokenBucketError, TokenBucketResult};
use super::types::{Namespace, Token, TokenStreamable};
use std::collections::HashMap;

/// Bucket organization modes for different use cases.
#[derive(Debug, Clone)]
pub enum BucketMode {
    /// Simple flat HashMap structure: namespace -> key -> value
    Flat,
    /// Nested tree structure only (parent-child relationships)
    Tree,
    /// Both flat data and tree index for maximum flexibility
    Hybrid,
}

/// A collection of tokens organized by namespaces with different access patterns.
///
/// TokenBucket provides structured access to token data with support for:
/// - Flat key-value access by namespace
/// - Hierarchical tree navigation (Tree/Hybrid modes)
/// - Namespace switching via `ns=` tokens
/// - Parent-child and sibling relationship queries
#[derive(Debug)]
pub struct TokenBucket {
    /// The organizational mode for this bucket
    pub mode: BucketMode,
    /// Flat storage: namespace_name -> key -> value
    pub data: HashMap<String, HashMap<String, String>>,
    /// Tree index: parent_namespace -> [child_namespaces] (Tree/Hybrid modes only)
    pub tree: Option<HashMap<String, Vec<String>>>,
}

impl TokenBucket {
    /// Create a new empty token bucket with the specified mode.
    pub fn new(mode: BucketMode) -> Self {
        TokenBucket {
            mode: mode.clone(),
            data: HashMap::new(),
            tree: match mode {
                BucketMode::Flat => None,
                _ => Some(HashMap::new()),
            },
        }
    }

    /// Create a token bucket from a collection of tokens.
    ///
    /// # Examples
    /// ```
    /// use rsb::token::{Token, TokenBucket, BucketMode};
    ///
    /// let tokens = vec![
    ///     Token::simple("host", "localhost"),
    ///     Token::with_namespace("db".parse().unwrap(), "user".to_string(), "admin".to_string()),
    /// ];
    /// let bucket = TokenBucket::from_tokens(&tokens, BucketMode::Flat);
    /// ```
    pub fn from_tokens(tokens: &[Token], mode: BucketMode) -> Self {
        collect_tokens(tokens, mode)
    }

    /// Create a token bucket by parsing a token stream string.
    ///
    /// This combines parsing and bucket collection in one step.
    ///
    /// # Examples
    /// ```
    /// use rsb::token::{TokenBucket, BucketMode};
    ///
    /// let bucket = TokenBucket::from_str(r#"host="localhost"; db:user="admin";"#, BucketMode::Flat).unwrap();
    /// ```
    pub fn from_str(input: &str, mode: BucketMode) -> TokenBucketResult<Self> {
        if input.trim().is_empty() {
            return Err(TokenBucketError::EmptyInput);
        }

        let tokens = input
            .tokenize()
            .map_err(|e| TokenBucketError::ParseError(e))?;

        if tokens.is_empty() {
            return Err(TokenBucketError::ParseError(
                "No valid tokens found in input".to_string(),
            ));
        }

        Ok(Self::from_tokens(&tokens, mode))
    }

    /// Insert a token into the bucket at the specified namespace.
    ///
    /// Updates both flat data storage and tree index (if applicable).
    pub fn insert(&mut self, namespace: &Namespace, key: String, value: String) {
        let ns_key = namespace.to_string();

        // Always store flat data
        self.data
            .entry(ns_key.clone())
            .or_insert_with(HashMap::new)
            .insert(key, value);

        // Build tree if needed
        match self.mode {
            BucketMode::Flat => {}
            BucketMode::Tree | BucketMode::Hybrid => {
                self.build_tree_index(namespace);
            }
        }
    }

    /// Internal method to build tree index for hierarchical namespace access.
    fn build_tree_index(&mut self, namespace: &Namespace) {
        if let Some(ref mut tree) = self.tree {
            let _full_path = namespace.to_string();

            // Build parent-child relationships for all levels
            let mut current_path = String::new();
            for (i, part) in namespace.parts.iter().enumerate() {
                if i > 0 {
                    let parent_path = current_path.clone();
                    current_path.push(namespace.delimiter);
                    current_path.push_str(part);

                    // Add current path to parent's children
                    tree.entry(parent_path)
                        .or_insert_with(Vec::new)
                        .push(current_path.clone());
                } else {
                    current_path = part.clone();
                    // Root level entries
                    tree.entry("".to_string())
                        .or_insert_with(Vec::new)
                        .push(current_path.clone());
                }
            }
        }
    }

    /// Get direct children of a namespace (Tree/Hybrid modes only).
    ///
    /// # Examples
    /// ```
    /// use rsb::token::{TokenBucket, BucketMode};
    ///
    /// let bucket = TokenBucket::from_str("a:k1=v1; a.b:k2=v2; a.c:k3=v3;", BucketMode::Tree).unwrap();
    /// let children = bucket.get_children("a");
    /// assert!(children.contains(&"a.b".to_string()));
    /// ```
    pub fn get_children(&self, namespace: &str) -> Vec<String> {
        if let Some(ref tree) = self.tree {
            tree.get(namespace).cloned().unwrap_or_default()
        } else {
            Vec::new()
        }
    }

    /// Get all namespaces under a given prefix (descendants).
    ///
    /// Works in all modes by filtering the flat data keys.
    pub fn get_all_under(&self, prefix: &str) -> Vec<String> {
        self.data
            .keys()
            .filter(|ns| ns.starts_with(prefix) && ns.as_str() != prefix)
            .cloned()
            .collect()
    }

    /// Get namespace data by exact match.
    ///
    /// Returns the key-value map for the specified namespace, if it exists.
    pub fn get_namespace(&self, namespace: &str) -> Option<&HashMap<String, String>> {
        self.data.get(namespace)
    }

    /// Get sibling namespaces (namespaces at the same hierarchical level).
    ///
    /// # Examples
    /// ```
    /// use rsb::token::{TokenBucket, BucketMode};
    ///
    /// let bucket = TokenBucket::from_str("a.b:k1=v1; a.c:k2=v2; a.d:k3=v3;", BucketMode::Tree).unwrap();
    /// let siblings = bucket.get_siblings("a.b");
    /// assert!(siblings.contains(&"a.c".to_string()));
    /// assert!(siblings.contains(&"a.d".to_string()));
    /// ```
    pub fn get_siblings(&self, namespace: &str) -> Vec<String> {
        if let Some(parent) = self.get_parent(namespace) {
            self.get_children(&parent)
                .into_iter()
                .filter(|ns| ns != namespace)
                .collect()
        } else {
            // Root level siblings
            self.get_children("")
                .into_iter()
                .filter(|ns| ns != namespace)
                .collect()
        }
    }

    /// Get the parent namespace of the given namespace.
    fn get_parent(&self, namespace: &str) -> Option<String> {
        if let Some(last_dot) = namespace.rfind('.') {
            Some(namespace[..last_dot].to_string())
        } else {
            None
        }
    }
}

/// Collect tokens into a bucket, handling namespace switching logic.
///
/// This function implements XStream's namespace switching behavior:
/// - Tokens with explicit namespaces (ns:key=value) use their namespace
/// - `ns=namespace` tokens switch the active namespace for subsequent tokens
/// - Tokens without explicit namespaces use the current active namespace
/// - The default namespace is "global"
///
/// # Examples
/// ```
/// use rsb::token::{tokenize_string, collect_tokens, BucketMode};
///
/// let tokens = tokenize_string("item=val1; ns=animals; dog=fido; cat=fluffy;").unwrap();
/// let bucket = collect_tokens(&tokens, BucketMode::Flat);
///
/// // item goes to global, dog and cat go to animals namespace
/// assert!(bucket.data.contains_key("global"));
/// assert!(bucket.data.contains_key("animals"));
/// ```
pub fn collect_tokens(tokens: &[Token], mode: BucketMode) -> TokenBucket {
    let mut bucket = TokenBucket::new(mode);
    let mut active_namespace = Namespace::from_string("global");

    for token in tokens {
        // Handle ns= tokens for namespace switching
        if token.namespace.is_none() && token.key == "ns" {
            active_namespace = Namespace::from_string(&token.value);
            continue;
        }

        // Use token's namespace if present, otherwise use active namespace
        let namespace = token.namespace.as_ref().unwrap_or(&active_namespace);

        bucket.insert(namespace, token.key.clone(), token.value.clone());
    }

    bucket
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::tokenize_string;

    #[test]
    fn test_bucket_creation() {
        let bucket = TokenBucket::new(BucketMode::Flat);
        assert!(bucket.data.is_empty());
        assert!(bucket.tree.is_none());

        let bucket = TokenBucket::new(BucketMode::Tree);
        assert!(bucket.tree.is_some());
    }

    #[test]
    fn test_from_tokens() {
        let tokens = vec![
            Token::simple("host", "localhost"),
            Token::with_namespace(
                Namespace::from_string("db"),
                "user".to_string(),
                "admin".to_string(),
            ),
        ];

        let bucket = TokenBucket::from_tokens(&tokens, BucketMode::Flat);
        assert_eq!(bucket.data.len(), 2);
        assert!(bucket.data.contains_key("global"));
        assert!(bucket.data.contains_key("db"));
    }

    #[test]
    fn test_from_str() {
        let bucket =
            TokenBucket::from_str(r#"host="localhost"; db:user="admin";"#, BucketMode::Flat)
                .unwrap();
        assert_eq!(bucket.data.len(), 2);
        assert_eq!(bucket.data["global"]["host"], "localhost");
        assert_eq!(bucket.data["db"]["user"], "admin");
    }

    #[test]
    fn test_namespace_switching() {
        let input = "item=value1; ns=animals; dog=fido; cat=fluffy; ns=global; final=done;";
        let tokens = tokenize_string(input).unwrap();
        let bucket = collect_tokens(&tokens, BucketMode::Flat);

        // Check global namespace
        assert!(bucket.data.contains_key("global"));
        assert_eq!(bucket.data["global"]["item"], "value1");
        assert_eq!(bucket.data["global"]["final"], "done");

        // Check animals namespace
        assert!(bucket.data.contains_key("animals"));
        assert_eq!(bucket.data["animals"]["dog"], "fido");
        assert_eq!(bucket.data["animals"]["cat"], "fluffy");

        // ns= tokens should not appear in the bucket data
        assert!(!bucket.data.values().any(|map| map.contains_key("ns")));
    }

    #[test]
    fn test_explicit_namespace_override() {
        let input = "tok1=val1; ns=color; tok2=val2; meta:p=q; sec:user=bob;";
        let tokens = tokenize_string(input).unwrap();
        let bucket = collect_tokens(&tokens, BucketMode::Flat);

        // Check global namespace (tok1)
        assert_eq!(bucket.data["global"]["tok1"], "val1");

        // Check color namespace (tok2 due to ns=color)
        assert_eq!(bucket.data["color"]["tok2"], "val2");

        // Check that prefixed tokens ignore active namespace
        assert_eq!(bucket.data["meta"]["p"], "q");
        assert_eq!(bucket.data["sec"]["user"], "bob");

        // Verify meta:p=q did NOT go to color namespace
        assert!(!bucket.data["color"].contains_key("p"));
    }

    #[test]
    fn test_hierarchical_namespaces() {
        let input = "a:k1=v1; a.b:k2=v2; a.b.c:k3=v3; a.b.d:k4=v4; a.e:k5=v5;";
        let tokens = tokenize_string(input).unwrap();
        let bucket = collect_tokens(&tokens, BucketMode::Tree);

        // Test exact namespace access
        assert!(bucket.get_namespace("a.b.c").is_some());
        assert_eq!(bucket.get_namespace("a.b.c").unwrap()["k3"], "v3");

        // Test hierarchical queries
        let under_a = bucket.get_all_under("a");
        assert!(under_a.contains(&"a.b".to_string()));
        assert!(under_a.contains(&"a.b.c".to_string()));
        assert!(under_a.contains(&"a.e".to_string()));

        let under_ab = bucket.get_all_under("a.b");
        assert!(under_ab.contains(&"a.b.c".to_string()));
        assert!(under_ab.contains(&"a.b.d".to_string()));
        assert!(!under_ab.contains(&"a.e".to_string()));
    }

    #[test]
    fn test_tree_navigation() {
        let input = "a:k1=v1; a.b:k2=v2; a.b.c:k3=v3; a.b.d:k4=v4; a.e:k5=v5;";
        let tokens = tokenize_string(input).unwrap();
        let bucket = collect_tokens(&tokens, BucketMode::Tree);

        // Test parent-child relationships
        let root_children = bucket.get_children("");
        assert!(root_children.contains(&"a".to_string()));

        let a_children = bucket.get_children("a");
        assert!(a_children.contains(&"a.b".to_string()));
        assert!(a_children.contains(&"a.e".to_string()));

        let ab_children = bucket.get_children("a.b");
        assert!(ab_children.contains(&"a.b.c".to_string()));
        assert!(ab_children.contains(&"a.b.d".to_string()));

        // Test siblings
        let ab_siblings = bucket.get_siblings("a.b");
        assert!(ab_siblings.contains(&"a.e".to_string()));
        assert!(!ab_siblings.contains(&"a.b".to_string())); // Should not include self

        let cd_siblings = bucket.get_siblings("a.b.c");
        assert!(cd_siblings.contains(&"a.b.d".to_string()));
    }

    #[test]
    fn test_error_handling() {
        // Test empty input
        let result = TokenBucket::from_str("", BucketMode::Flat);
        assert!(matches!(result, Err(TokenBucketError::EmptyInput)));

        let result = TokenBucket::from_str("   ", BucketMode::Flat);
        assert!(matches!(result, Err(TokenBucketError::EmptyInput)));
    }
}
