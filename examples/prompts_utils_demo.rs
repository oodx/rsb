// RSB Prompts Utils Namespace Demo
// Demonstrates explicit access to low-level timeout functions
use rsb::prelude::*;

#[cfg(feature = "prompts")]
use rsb::visual::utils::*; // Explicit utils namespace

fn main() {
    println!("=== RSB visual::utils Namespace Demo ===\n");

    #[cfg(not(feature = "prompts"))]
    {
        println!("Enable with: cargo run --example prompts_utils_demo --features prompts");
        return;
    }

    #[cfg(feature = "prompts")]
    {
        // Set up demo context
        set_var("opt_quiet", "1"); // Non-interactive for demo
        set_var("opt_prompt_timeout", "3"); // 3 second context timeout

        println!("=== Explicit Utils Access (Advanced Usage) ===");

        // Direct function calls with explicit control
        println!(
            "1. confirm_with_timeout(\"Deploy?\", None, false) → {}",
            confirm_with_timeout("Deploy?", None, false)
        );

        println!(
            "2. confirm_with_timeout(\"Risky op?\", Some(1), true) → {}",
            confirm_with_timeout("Risky op?", Some(1), true)
        );

        println!(
            "3. ask_with_timeout(\"API Key\", Some(\"test\"), Some(2)) → {}",
            ask_with_timeout("API Key", Some("test"), Some(2))
        );

        println!(
            "4. select_with_timeout(\"Env\", &[\"dev\", \"prod\"], Some(1), None) → {}",
            select_with_timeout("Env", &["dev", "prod"], Some(1), None)
        );

        println!("\n=== Comparison: Utils vs Macros ===");

        println!("Utils approach (explicit):");
        let utils_result = confirm_with_timeout("Continue?", Some(5), false);
        println!(
            "  confirm_with_timeout(\"Continue?\", Some(5), false) → {}",
            utils_result
        );

        println!("\nMacros approach (ergonomic):");
        use rsb::confirm_timeout;
        let macro_result = confirm_timeout!("Continue?", 5);
        println!("  confirm_timeout!(\"Continue?\", 5) → {}", macro_result);

        println!(
            "  → Both produce same result: {}",
            utils_result == macro_result
        );

        println!("\n=== When to Use Each Approach ===");
        println!("✅ **Macros** (recommended for most users):");
        println!("   - Ergonomic: confirm_timeout!(\"Ready?\", 10)");
        println!("   - Context-aware: reads global timeout settings");
        println!("   - RSB philosophy: simple, batteries-included");

        println!("\n✅ **Utils** (for advanced/library usage):");
        println!("   - Explicit control over all parameters");
        println!("   - Composable: can build custom wrappers");
        println!("   - Library-friendly: no magic, predictable behavior");

        println!("\n=== MODULE_SPEC Architecture ===");
        println!("📁 src/visual/prompts/");
        println!("   ├── mod.rs         # Orchestrator (no implementation)");
        println!("   ├── interactive.rs # Basic prompt implementations");
        println!("   └── utils.rs       # Timeout-enhanced functions");
        println!("📁 src/visual/");
        println!("   └── utils.rs       # Curated re-exports → visual::utils::*");
        println!("📁 src/macros/");
        println!("   └── visual.rs      # Thin macros → visual::utils functions");

        println!("\n=== Usage Patterns ===");
        println!("// Most applications:");
        println!("use rsb::{{confirm_timeout, ask_timeout}};");
        println!("let result = confirm_timeout!(\"Deploy?\", 30);");
        println!("");
        println!("// Advanced/library usage:");
        println!("use rsb::visual::utils::*;");
        println!("let result = confirm_with_timeout(\"Deploy?\", Some(30), false);");

        #[cfg(feature = "colors-simple")]
        {
            use rsb::colors::colored;
            println!(
                "\n{}",
                colored("{green}🎯 MODULE_SPEC Compliant Architecture Complete!{reset}")
            );
        }

        #[cfg(not(feature = "colors-simple"))]
        println!("\n🎯 MODULE_SPEC Compliant Architecture Complete!");
    }
}
