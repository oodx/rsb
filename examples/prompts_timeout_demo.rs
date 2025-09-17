// RSB Prompts with Timeout Enhancement Demo
use rsb::prelude::*;

#[cfg(feature = "prompts")]
use rsb::{
    ask, ask_timeout, confirm, confirm_timeout, prompt, prompt_timeout, select, select_timeout,
};

fn main() {
    println!("=== RSB Prompts with Timeout Enhancement Demo ===\n");

    #[cfg(not(feature = "prompts"))]
    {
        println!("Enable with: cargo run --example prompts_timeout_demo --features prompts");
        return;
    }

    #[cfg(feature = "prompts")]
    {
        // Set up colors for better UX
        #[cfg(feature = "colors-simple")]
        {
            use rsb::visual::colors::{color_enable_with, color_mode};
            color_mode("always");
            color_enable_with("simple");
        }

        println!("=== Basic Prompt Macros ===");

        // Demo basic macros in quiet mode
        set_var("opt_quiet", "1");

        println!("1. confirm!(\"Continue?\") → {}", confirm!("Continue?"));
        println!("2. ask!(\"Name\", \"Alice\") → {}", ask!("Name", "Alice"));
        println!(
            "3. select!(\"Color\", &[\"red\", \"blue\"]) → {}",
            select!("Color", &["red", "blue"])
        );
        println!(
            "4. prompt!(\"confirm\", \"Ready?\") → {}",
            prompt!("confirm", "Ready?")
        );
        println!(
            "5. prompt!(\"ask\", \"City\", \"NYC\") → {}",
            prompt!("ask", "City", "NYC")
        );

        unset_var("opt_quiet");

        println!("\n=== Timeout-Enhanced Macros (Context Timeout) ===");

        // Set global timeout context
        set_var("opt_prompt_timeout", "2");
        set_var("opt_quiet", "1"); // Keep quiet for demo

        println!("Context timeout: {} seconds", get_var("opt_prompt_timeout"));
        println!(
            "6. confirm_timeout!(\"Deploy?\") → {}",
            confirm_timeout!("Deploy?")
        );
        println!(
            "7. ask_timeout!(\"API Key\", \"test-key\") → {}",
            ask_timeout!("API Key", "test-key")
        );
        println!(
            "8. select_timeout!(\"Environment\", &[\"dev\", \"prod\"], 0) → {}",
            select_timeout!("Environment", &["dev", "prod"], 0)
        );

        println!("\n=== Timeout-Enhanced Macros (Explicit Timeout) ===");

        // Test explicit timeout overrides
        println!(
            "9. confirm_timeout!(\"Force update?\", 1, true) → {}",
            confirm_timeout!("Force update?", 1, true)
        );
        println!(
            "10. ask_timeout!(\"Username\", \"admin\", 1) → {}",
            ask_timeout!("Username", "admin", 1)
        );
        println!(
            "11. select_timeout!(\"Protocol\", &[\"http\", \"https\"], 1, 1) → {}",
            select_timeout!("Protocol", &["http", "https"], 1, 1)
        );

        println!("\n=== General Timeout Prompt Macro ===");

        println!(
            "12. prompt_timeout!(\"confirm\", \"Backup first?\") → {}",
            prompt_timeout!("confirm", "Backup first?")
        );
        println!(
            "13. prompt_timeout!(\"ask\", \"Email\", \"user@example.com\", 1) → {}",
            prompt_timeout!("ask", "Email", "user@example.com", 1)
        );

        println!("\n=== Environment Variable Fallback ===");

        unset_var("opt_prompt_timeout");
        set_var("PROMPT_TIMEOUT", "3");

        println!("Environment timeout: {} seconds", get_var("PROMPT_TIMEOUT"));
        println!(
            "14. confirm_timeout!(\"Final check?\") → {}",
            confirm_timeout!("Final check?")
        );

        println!("\n=== Priority Demonstration ===");

        // Show priority: explicit > CLI flag > env var > default
        set_var("opt_prompt_timeout", "5"); // CLI flag
        set_var("PROMPT_TIMEOUT", "10"); // Env var

        println!(
            "CLI flag timeout: {}, Env timeout: {}",
            get_var("opt_prompt_timeout"),
            get_var("PROMPT_TIMEOUT")
        );
        println!(
            "15. confirm_timeout!(\"Priority test?\", 1) → {} (explicit 1s wins)",
            confirm_timeout!("Priority test?", 1)
        );

        unset_var("opt_quiet");
        unset_var("opt_prompt_timeout");
        unset_var("PROMPT_TIMEOUT");

        println!("\n=== Feature Summary ===");
        println!("✅ Basic prompt macros: confirm!, ask!, select!, prompt!");
        println!("✅ Timeout enhanced macros: *_timeout! variants");
        println!("✅ Global context integration: opt_prompt_timeout, PROMPT_TIMEOUT");
        println!("✅ Priority system: explicit > CLI flag > env var > 30s default");
        println!("✅ Thread-based timeout with polling");
        println!("✅ Cross-platform TTY detection");
        println!("✅ CI/automation friendly (non-blocking)");

        #[cfg(feature = "colors-simple")]
        {
            use rsb::visual::colors::colored;
            println!(
                "\n{}",
                colored("{green}🎉 RSB Prompts with Timeout Enhancement Complete!{reset}")
            );
        }

        #[cfg(not(feature = "colors-simple"))]
        println!("\n🎉 RSB Prompts with Timeout Enhancement Complete!");
    }
}
