use crate::global::expand_vars;
use std::collections::HashSet;

/// Bash-like argument wrapper for CLI usage.
#[derive(Debug, Clone)]
pub struct Args {
    args: Vec<String>,
    processed: HashSet<usize>,
}

impl Args {
    pub fn new(args: &[String]) -> Self {
        Args { args: args.to_vec(), processed: HashSet::new() }
    }

    /// Get the n-th unprocessed positional argument (1-indexed).
    pub fn get(&self, n: usize) -> String {
        if n == 0 { return "".to_string(); }
        self.remaining().get(n).cloned().unwrap_or_default()
    }

    pub fn get_or(&self, n: usize, default: &str) -> String {
        let v = self.get(n);
        if v.is_empty() { default.to_string() } else { v }
    }

    pub fn has(&self, flag: &str) -> bool { self.args.iter().any(|a| a == flag) }

    pub fn has_pop(&mut self, flag: &str) -> bool {
        if let Some(pos) = self.args.iter().position(|a| a == flag) {
            self.processed.insert(pos); true
        } else { false }
    }

    /// Support both `--flag=value` and `--flag value` forms.
    pub fn has_val(&mut self, flag: &str) -> Option<String> {
        for (i, arg) in self.args.iter().enumerate() {
            if let Some(value) = arg.strip_prefix(&format!("{}=", flag)) {
                if !self.processed.contains(&i) { self.processed.insert(i); return Some(value.to_string()); }
            }
        }
        if let Some(pos) = self.args.iter().position(|a| a == flag) {
            if !self.processed.contains(&pos) && pos + 1 < self.args.len() {
                self.processed.insert(pos); self.processed.insert(pos + 1);
                return Some(self.args[pos + 1].clone());
            }
        }
        None
    }

    pub fn get_kv(&mut self, key: &str) -> Option<String> {
        for (i, arg) in self.args.iter().enumerate() {
            if self.processed.contains(&i) { continue; }
            if let Some(v) = arg.strip_prefix(&format!("{}=", key)) { self.processed.insert(i); return Some(v.to_string()); }
            if let Some(v) = arg.strip_prefix(&format!("{}:", key)) { self.processed.insert(i); return Some(v.to_string()); }
        }
        None
    }

    pub fn get_array(&mut self, key: &str) -> Option<Vec<String>> {
        self.get_kv(key).map(|v| v.split(',').map(|s| s.trim().to_string()).collect())
    }

    pub fn remaining(&self) -> Vec<String> {
        self.args.iter().enumerate().filter(|(i, _)| !self.processed.contains(i)).map(|(_, a)| a.clone()).collect()
    }
    pub fn all(&self) -> &[String] { &self.args }
    pub fn join(&self, sep: &str) -> String { self.remaining().join(sep) }
    pub fn len(&self) -> usize { self.remaining().len() }

    /// Expand `$1..$N`, `$@`, `$#`, then Global vars.
    pub fn expand(&self, template: &str) -> String {
        let mut out = template.to_string();
        let rem = self.remaining();
        for (i, a) in rem.iter().enumerate() { out = out.replace(&format!("${}", i + 1), a); }
        out = out.replace("$@", &self.join(" "));
        out = out.replace("$#", &self.len().to_string());
        expand_vars(&out)
    }
}

