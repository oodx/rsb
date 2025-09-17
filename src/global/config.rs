//! Global configuration parsing and I/O

use std::path::Path;

use crate::global::{expand_vars, get_var, has_var, set_var};

pub fn parse_config_content(content: &str) {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            let value = value
                .strip_prefix('"')
                .and_then(|v| v.strip_suffix('"'))
                .unwrap_or(value);
            let value = value
                .strip_prefix('\'')
                .and_then(|v| v.strip_suffix('\''))
                .unwrap_or(value);
            if value.starts_with('(') && value.ends_with(')') {
                let array_content = &value[1..value.len() - 1];
                let mut items = Vec::new();
                let mut current_item = String::new();
                let mut in_quotes = false;
                for ch in array_content.chars() {
                    match ch {
                        '"' => in_quotes = !in_quotes,
                        ' ' if !in_quotes => {
                            if !current_item.is_empty() {
                                items.push(current_item.clone());
                                current_item.clear();
                            }
                        }
                        _ => current_item.push(ch),
                    }
                }
                if !current_item.is_empty() {
                    items.push(current_item);
                }
                set_var(&format!("{}_LENGTH", key), &items.len().to_string());
                for (i, item) in items.iter().enumerate() {
                    set_var(&format!("{}_{}", key, i), item);
                }
                set_var(key, &items.join(" "));
            } else {
                set_var(key, value);
            }
        }
    }
}

pub fn load_config_file(path: &str) {
    let expanded_path = expand_vars(path);
    if let Ok(content) = std::fs::read_to_string(&expanded_path) {
        parse_config_content(&content);
    }
}

pub fn save_config_file(path: &str, keys: &[&str]) {
    let expanded_path = expand_vars(path);
    let mut content = String::new();
    content.push_str("# RSB Configuration File\n");
    content.push_str(&format!(
        "# Generated on {}\n\n",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    ));
    for key in keys {
        if has_var(key) {
            let value = get_var(key);
            if value.contains(' ') {
                content.push_str(&format!("{}=\"{}\"\n", key, value));
            } else {
                content.push_str(&format!("{}={}\n", key, value));
            }
        }
    }
    if let Some(parent) = Path::new(&expanded_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(&expanded_path, &content);
}

pub fn export_vars(path: &str) {
    let expanded_path = expand_vars(path);
    let all_vars = crate::global::get_all_vars();
    let mut content = String::new();
    for (key, value) in all_vars.iter() {
        content.push_str(&format!("export {}='{}'\n", key, value));
    }
    if let Some(parent) = Path::new(&expanded_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(&expanded_path, &content);
}
