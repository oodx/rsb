//! Namespacing helpers on top of the flat Global store.
//!
//! Supports two key styles:
//! - Dunder:  NS__KEY
//! - Colon:   NS::KEY
//!
//! Getters try both styles by default. Setters have explicit style variants.

#[derive(Copy, Clone)]
pub enum NsStyle { Dunder, Colon }

fn make_key(ns: &str, key: &str, style: NsStyle) -> String {
    match style {
        NsStyle::Dunder => format!("{}__{}", ns, key),
        NsStyle::Colon => format!("{}::{}", ns, key),
    }
}

/// Set a namespaced key (dunder style: NS__KEY)
pub fn ns_set(ns: &str, key: &str, value: &str) { crate::global::set_var(make_key(ns, key, NsStyle::Dunder), value); }

/// Set a namespaced key (colon-colon style: NS::KEY)
pub fn ns_set_cc(ns: &str, key: &str, value: &str) { crate::global::set_var(make_key(ns, key, NsStyle::Colon), value); }

/// Get a namespaced key. Checks NS__KEY first, then NS::KEY.
pub fn ns_get(ns: &str, key: &str) -> String {
    let k_dunder = make_key(ns, key, NsStyle::Dunder);
    let v = crate::global::get_var(&k_dunder);
    if !v.is_empty() { return v; }
    let k_cc = make_key(ns, key, NsStyle::Colon);
    crate::global::get_var(&k_cc)
}

/// Get a namespaced key with explicit style.
pub fn ns_get_with_style(ns: &str, key: &str, style: NsStyle) -> String {
    crate::global::get_var(&make_key(ns, key, style))
}

/// List all keys in a namespace across both styles. Returns (key_without_ns, value).
pub fn ns_get_all(ns: &str) -> Vec<(String, String)> {
    let dunder_prefix = format!("{}__", ns);
    let colon_prefix = format!("{}::", ns);
    let mut map = std::collections::HashMap::new();
    for (k, v) in crate::global::get_all_vars() {
        if let Some(rest) = k.strip_prefix(&dunder_prefix) { map.insert(rest.to_string(), v.clone()); }
        if let Some(rest) = k.strip_prefix(&colon_prefix) { map.entry(rest.to_string()).or_insert(v.clone()); }
    }
    let mut out: Vec<(String, String)> = map.into_iter().collect();
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

/// Overlay all namespaced keys into plain keys (copy NS:key → key). Returns count.
pub fn ns_overlay_to_plain(ns: &str) -> usize {
    let items = ns_get_all(ns);
    for (k, v) in &items { crate::global::set_var(k, v); }
    items.len()
}

/// Overlay selected plain keys into a namespace (copy key → NS:key). Returns count.
pub fn ns_overlay_plain_to_ns(ns: &str, keys: &[&str], style: Option<NsStyle>) -> usize {
    let mut n = 0;
    let style = style.unwrap_or(NsStyle::Dunder);
    for k in keys {
        let v = crate::global::get_var(k);
        if !v.is_empty() {
            crate::global::set_var(make_key(ns, k, style), v);
            n += 1;
        }
    }
    n
}

