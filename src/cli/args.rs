use crate::cli::options::{OptionsContext, OptionsStrategy};
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
        Args {
            args: args.to_vec(),
            processed: HashSet::new(),
        }
    }

    /// Create Args from &str slices (convenience for tests and simple cases)
    pub fn from_strs(args: &[&str]) -> Self {
        Args {
            args: args.iter().map(|s| s.to_string()).collect(),
            processed: HashSet::new(),
        }
    }

    /// Create Args from a command line string using SimpleParser
    ///
    /// Parses the line with quote-aware tokenization and pattern preservation.
    /// Useful for REPL command processing.
    ///
    /// # Example
    /// ```rust,ignore
    /// let args = Args::from_line("build --output=dist \"my file\"");
    /// assert_eq!(args.get(1), "build");
    /// assert_eq!(args.get(2), "--output=dist");
    /// assert_eq!(args.get(3), "my file");
    /// ```
    pub fn from_line(line: &str) -> Self {
        use crate::repl::parser::{ReplParser, SimpleParser};
        let parser = SimpleParser;
        let tokens = parser.parse(line);
        Args {
            args: tokens,
            processed: HashSet::new(),
        }
    }

    fn is_program_index(&self, index: usize) -> bool {
        if self.args.is_empty() || index != 0 {
            return false;
        }
        if self.args.len() == 1 && self.args[0].starts_with('-') {
            return false;
        }
        true
    }

    fn positional_iter(&self) -> impl Iterator<Item = (usize, &String)> {
        self.args.iter().enumerate().filter_map(|(i, value)| {
            if self.is_program_index(i) || self.processed.contains(&i) {
                None
            } else {
                Some((i, value))
            }
        })
    }

    /// Get the n-th unprocessed positional argument (1-indexed).
    pub fn get(&self, n: usize) -> String {
        if n == 0 {
            return String::new();
        }
        self.positional_iter()
            .nth(n - 1)
            .map(|(_, value)| value.clone())
            .unwrap_or_default()
    }

    pub fn get_or(&self, n: usize, default: &str) -> String {
        let v = self.get(n);
        if v.is_empty() {
            default.to_string()
        } else {
            v
        }
    }

    pub fn has(&self, flag: &str) -> bool {
        self.args
            .iter()
            .enumerate()
            .any(|(i, a)| a == flag && !self.processed.contains(&i) && !self.is_program_index(i))
    }

    pub fn has_pop(&mut self, flag: &str) -> bool {
        let mut to_mark = None;
        for (i, arg) in self.args.iter().enumerate() {
            if arg == flag && !self.processed.contains(&i) && !self.is_program_index(i) {
                to_mark = Some(i);
                break;
            }
        }
        if let Some(index) = to_mark {
            self.processed.insert(index);
            true
        } else {
            false
        }
    }

    /// Support both `--flag=value` and `--flag value` forms.
    pub fn has_val(&mut self, flag: &str) -> Option<String> {
        for (i, arg) in self.args.iter().enumerate() {
            if self.is_program_index(i) || self.processed.contains(&i) {
                continue;
            }
            if let Some(value) = arg.strip_prefix(&format!("{}=", flag)) {
                self.processed.insert(i);
                return Some(value.to_string());
            }
        }
        for (i, arg) in self.args.iter().enumerate() {
            if self.is_program_index(i) || self.processed.contains(&i) {
                continue;
            }
            if arg == flag {
                let value_index = i + 1;
                if value_index < self.args.len() && !self.processed.contains(&value_index) {
                    self.processed.insert(i);
                    self.processed.insert(value_index);
                    return Some(self.args[value_index].clone());
                }
            }
        }
        None
    }

    pub fn get_kv(&mut self, key: &str) -> Option<String> {
        for (i, arg) in self.args.iter().enumerate() {
            if self.is_program_index(i) || self.processed.contains(&i) {
                continue;
            }
            if let Some(v) = arg.strip_prefix(&format!("{}=", key)) {
                self.processed.insert(i);
                return Some(v.to_string());
            }
            if let Some(v) = arg.strip_prefix(&format!("{}:", key)) {
                self.processed.insert(i);
                return Some(v.to_string());
            }
        }
        None
    }

    pub fn get_array(&mut self, key: &str) -> Option<Vec<String>> {
        self.get_kv(key)
            .map(|v| v.split(',').map(|s| s.trim().to_string()).collect())
    }

    pub fn remaining(&self) -> Vec<String> {
        self.positional_iter().map(|(_, a)| a.clone()).collect()
    }
    pub fn all(&self) -> &[String] {
        &self.args
    }
    pub fn join(&self, sep: &str) -> String {
        self.remaining().join(sep)
    }
    pub fn len(&self) -> usize {
        self.args.len()
    }

    /// Expand `$1..$N`, `$@`, `$#`, then Global vars.
    pub fn expand(&self, template: &str) -> String {
        let mut out = template.to_string();
        let rem = self.remaining();
        for (i, a) in rem.iter().enumerate().rev() {
            out = out.replace(&format!("${}", i + 1), a);
        }
        out = out.replace("$@", &rem.join(" "));
        out = out.replace("$#", &rem.len().to_string());
        expand_vars(&out)
    }

    /// Apply options strategy after processing options
    pub fn apply_options_strategy(&mut self, strategy: OptionsStrategy, context: &OptionsContext) {
        // First validate we haven't consumed positional args as flag values
        if context.has_boundary_issues {
            eprintln!("Warning: Potential flag/value boundary issue detected");
        }

        match strategy {
            OptionsStrategy::Sort => self.sort_flags_last(),
            OptionsStrategy::Remove => self.remove_flags(&context.processed_flags),
            OptionsStrategy::Default => {},
        }
    }

    /// Sort flags to the end of the argument list
    fn sort_flags_last(&mut self) {
        let (mut positionals, mut flags): (Vec<_>, Vec<_>) = self.args
            .drain(..)
            .partition(|arg| !arg.starts_with('-'));

        // Keep program name at the beginning if present
        if !positionals.is_empty() && self.is_program_index(0) {
            let prog = positionals.remove(0);
            self.args.push(prog);
        }

        self.args.append(&mut positionals);
        self.args.append(&mut flags);
    }

    /// Remove processed flags from the argument list
    fn remove_flags(&mut self, processed: &[String]) {
        self.args.retain(|arg| {
            !processed.iter().any(|flag| {
                arg == flag || arg.starts_with(&format!("{}=", flag))
            })
        });
    }
}
