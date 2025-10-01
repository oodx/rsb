//! rsbdoc - RSB Documentation CLI
//!
//! Quick access to RSB and development documentation from the terminal.
//!
//! Usage:
//!   rsbdoc <topic> <doc>       # View specific documentation
//!   rsbdoc feat <name>         # View RSB feature docs
//!   rsbdoc list                # List all available topics
//!   rsbdoc <topic> list        # List docs for a topic

use rsb::prelude::*;
use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        show_help();
        return;
    }

    match args[0].as_str() {
        "list" => list_topics(),
        "feat" | "arch" | "proc" | "concept" => {
            if args.len() < 2 {
                list_docs(&args[0]);
            } else if args[1] == "list" {
                list_docs(&args[0]);
            } else {
                show_doc(&args[0], &args[1]);
            }
        }
        _ => {
            stderr!("Unknown topic: {}", args[0]);
            stderr!("Run 'rsbdoc list' to see available topics");
            std::process::exit(1);
        }
    }
}

fn show_help() {
    echo!(r#"
╔════════════════════════════════════════════════╗
║              rsbdoc - Documentation CLI        ║
╚════════════════════════════════════════════════╝

Usage:
  rsbdoc <topic> <doc>       View specific documentation
  rsbdoc <topic> list        List docs for a topic
  rsbdoc list                List all topics

Topics:
  feat       RSB features (from rsb/docs/tech/features/)
  arch       Architecture docs (from $BRAIN_HOME/dev/architecture/)
  proc       Process docs (from $BRAIN_HOME/dev/proccess/)
  concept    Concept docs (from $BRAIN_HOME/dev/concepts/)

Examples:
  rsbdoc feat bash           # View FEATURES_BASH.md
  rsbdoc feat list           # List all feature docs
  rsbdoc arch rebel          # View REBEL architecture
  rsbdoc proc testing        # View testing process docs
  rsbdoc list                # Show all available topics

Environment:
  BRAIN_HOME    Path to brain docs (default: ~/repos/docs/brain)
  RSB_HOME      Path to RSB repo (auto-detected from binary location)
"#);
}

fn list_topics() {
    echo!("Available topics:");
    echo!("  feat       RSB features");
    echo!("  arch       Architecture documentation");
    echo!("  proc       Process documentation");
    echo!("  concept    Concept documentation");
    echo!("");
    echo!("Use 'rsbdoc <topic> list' to see available docs for a topic");
}

fn list_docs(topic: &str) {
    let docs = discover_docs(topic);

    if docs.is_empty() {
        stderr!("No documentation found for topic: {}", topic);
        return;
    }

    echo!("\nAvailable {} docs:", topic);
    for doc in docs {
        echo!("  - {}", doc);
    }
    echo!("");
}

fn show_doc(topic: &str, doc: &str) {
    match find_doc(topic, doc) {
        Some(path) => {
            if test!(-f path.to_str().unwrap()) {
                display_doc(&path);
            } else {
                stderr!("Documentation file not found: {}", path.display());
                std::process::exit(1);
            }
        }
        None => {
            stderr!("Could not locate documentation: {} {}", topic, doc);
            stderr!("Run 'rsbdoc {} list' to see available docs", topic);
            std::process::exit(1);
        }
    }
}

fn find_doc(topic: &str, doc: &str) -> Option<PathBuf> {
    let brain_home = env::var("BRAIN_HOME")
        .unwrap_or_else(|_| format!("{}/repos/docs/brain", env::var("HOME").unwrap_or_default()));
    let rsb_home = env::var("RSB_HOME")
        .unwrap_or_else(|_| detect_rsb_home());

    match topic {
        "feat" => {
            // Try FEATURES_<DOC>.md pattern
            let path = PathBuf::from(&rsb_home)
                .join("docs/tech/features")
                .join(format!("FEATURES_{}.md", doc.to_uppercase()));
            if path.exists() { Some(path) } else { None }
        }
        "arch" => {
            // Try brain/dev/architecture/<doc>/
            let mut path = PathBuf::from(&brain_home)
                .join("dev/architecture")
                .join(doc);
            if path.is_dir() {
                // Look for README.md or <doc>.md inside
                let readme = path.join("README.md");
                if readme.exists() {
                    return Some(readme);
                }
            } else {
                // Try as direct file
                path.set_extension("md");
                if path.exists() {
                    return Some(path);
                }
            }
            None
        }
        "proc" => {
            // Try brain/dev/proccess/<doc>/
            let mut path = PathBuf::from(&brain_home)
                .join("dev/proccess")
                .join(doc);
            if path.is_dir() {
                let readme = path.join("README.md");
                if readme.exists() {
                    return Some(readme);
                }
            } else {
                path.set_extension("md");
                if path.exists() {
                    return Some(path);
                }
            }
            None
        }
        "concept" => {
            // Try brain/dev/concepts/<doc>/
            let mut path = PathBuf::from(&brain_home)
                .join("dev/concepts")
                .join(doc);
            if path.is_dir() {
                let readme = path.join("README.md");
                if readme.exists() {
                    return Some(readme);
                }
            } else {
                path.set_extension("md");
                if path.exists() {
                    return Some(path);
                }
            }
            None
        }
        _ => None,
    }
}

fn discover_docs(topic: &str) -> Vec<String> {
    let mut docs = Vec::new();

    let brain_home = env::var("BRAIN_HOME")
        .unwrap_or_else(|_| format!("{}/repos/docs/brain", env::var("HOME").unwrap_or_default()));
    let rsb_home = env::var("RSB_HOME")
        .unwrap_or_else(|_| detect_rsb_home());

    match topic {
        "feat" => {
            let features_dir = PathBuf::from(&rsb_home).join("docs/tech/features");
            if let Ok(entries) = std::fs::read_dir(&features_dir) {
                for entry in entries.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.starts_with("FEATURES_") && name.ends_with(".md") {
                            let doc_name = name
                                .trim_start_matches("FEATURES_")
                                .trim_end_matches(".md")
                                .to_lowercase();
                            docs.push(doc_name);
                        }
                    }
                }
            }
        }
        "arch" => {
            let arch_dir = PathBuf::from(&brain_home).join("dev/architecture");
            if let Ok(entries) = std::fs::read_dir(&arch_dir) {
                for entry in entries.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        if entry.path().is_dir() {
                            docs.push(name.to_string());
                        } else if name.ends_with(".md") && name != "README.md" {
                            docs.push(name.trim_end_matches(".md").to_string());
                        }
                    }
                }
            }
        }
        "proc" => {
            let proc_dir = PathBuf::from(&brain_home).join("dev/proccess");
            if let Ok(entries) = std::fs::read_dir(&proc_dir) {
                for entry in entries.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        if entry.path().is_dir() {
                            docs.push(name.to_string());
                        } else if name.ends_with(".md") && name != "README.md" {
                            docs.push(name.trim_end_matches(".md").to_string());
                        }
                    }
                }
            }
        }
        "concept" => {
            let concept_dir = PathBuf::from(&brain_home).join("dev/concepts");
            if let Ok(entries) = std::fs::read_dir(&concept_dir) {
                for entry in entries.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        if entry.path().is_dir() {
                            docs.push(name.to_string());
                        } else if name.ends_with(".md") && name != "README.md" {
                            docs.push(name.trim_end_matches(".md").to_string());
                        }
                    }
                }
            }
        }
        _ => {}
    }

    docs.sort();
    docs
}

fn display_doc(path: &PathBuf) {
    match std::fs::read_to_string(path) {
        Ok(content) => {
            // Simple markdown rendering for now
            echo!("\n{}\n", "═".repeat(80));
            for line in content.lines() {
                if line.starts_with("# ") {
                    // Header
                    echo!("\n\x1b[1;36m{}\x1b[0m", line.trim_start_matches("# "));
                } else if line.starts_with("## ") {
                    // Subheader
                    echo!("\n\x1b[1;34m{}\x1b[0m", line.trim_start_matches("## "));
                } else if line.starts_with("### ") {
                    // Sub-subheader
                    echo!("\n\x1b[1;32m{}\x1b[0m", line.trim_start_matches("### "));
                } else if line.starts_with("```") {
                    // Code block marker
                    echo!("\x1b[2m{}\x1b[0m", line);
                } else if line.starts_with("- ") || line.starts_with("* ") {
                    // List item
                    echo!("\x1b[33m•\x1b[0m {}", line.trim_start_matches("- ").trim_start_matches("* "));
                } else {
                    // Normal line
                    echo!("{}", line);
                }
            }
            echo!("\n{}\n", "═".repeat(80));
        }
        Err(e) => {
            stderr!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }
}

fn detect_rsb_home() -> String {
    // Try to detect RSB_HOME from binary location
    if let Ok(exe) = env::current_exe() {
        if let Some(parent) = exe.parent() {
            if let Some(grandparent) = parent.parent() {
                // Assuming binary is in <rsb>/target/release/rsbdoc or <rsb>/bin/
                return grandparent.to_string_lossy().to_string();
            }
        }
    }

    // Fallback to current directory
    env::current_dir()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}
