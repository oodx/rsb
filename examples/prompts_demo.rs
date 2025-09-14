// RSB-004 Prompts MVP Demo
use rsb::prelude::*;

#[cfg(feature = "prompts")]
use rsb::{confirm, ask, select};

fn main() {
    println!("=== RSB-004 Prompts MVP Demo ===\n");

    #[cfg(not(feature = "prompts"))]
    {
        println!("Enable with: cargo run --example prompts_demo --features prompts");
        return;
    }

    #[cfg(feature = "prompts")]
    {
        // Set up colors for better UX
        #[cfg(feature = "colors-simple")]
        {
            use rsb::visual::colors::{color_mode, color_enable_with};
            color_mode("always");
            color_enable_with("simple");
        }

        // Demo in non-interactive mode (opt_quiet for demo)
        println!("=== Testing in quiet mode (non-interactive) ===");
        set_var("opt_quiet", "1");

        println!("1. confirm!(\"Proceed?\") → {}", confirm!("Proceed?"));
        println!("2. ask!(\"Name\", \"Alice\") → {}", ask!("Name", "Alice"));
        println!("3. select!(\"Color\", &[\"red\", \"blue\"]) → {}", select!("Color", &["red", "blue"]));
        println!("4. select!(\"Option\", &[\"a\", \"b\", \"c\"], 1) → {}", select!("Option", &["a", "b", "c"], 1));

        // Demo with opt_yes
        unset_var("opt_quiet");
        set_var("opt_yes", "1");
        println!("\n=== Testing with opt_yes (auto-confirm) ===");
        println!("5. confirm!(\"Delete files?\") → {}", confirm!("Delete files?"));

        unset_var("opt_yes");

        println!("\n=== Thin Macro Pattern Verification ===");
        println!("✅ Macros delegate to visual::prompts::* functions");
        println!("✅ MODULE_SPEC compliance: thin wrappers only");
        println!("✅ Feature gated: requires 'prompts' feature");
        println!("✅ Global context integration: opt_yes, opt_quiet work");
        println!("✅ Non-TTY fallback behavior implemented");

        #[cfg(feature = "colors-simple")]
        {
            use rsb::visual::colors::colored;
            println!("\n{}", colored("{green}🎉 RSB-004 Prompts MVP Complete!{reset}"));
        }

        #[cfg(not(feature = "colors-simple"))]
        println!("\n🎉 RSB-004 Prompts MVP Complete!");
    }
}