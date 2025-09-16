// moved from tests/uat_prompts.rs
use rsb::prelude::*;
use rsb::{confirm, ask, select};

#[test]
fn uat_prompts_demo_and_behaviors() {
    println!("\n=== UAT: Prompts (confirm/ask/select) ===\n");
    #[cfg(feature = "visual")]
    {
        use rsb::visual::colors::{color_enable_with, color_mode, colored};
        color_mode("always");
        color_enable_with("simple");
        println!("{}", colored("{green}Prompts demo active{reset}"));
    }
    set_var("opt_yes", "true");
    let sure = rsb::visual::prompts::confirm("Proceed with operation?");
    println!("auto-yes confirm → {}", if sure { "yes" } else { "no" });
    unset_var("opt_yes");
    set_var("opt_quiet", "true");
    let ok = rsb::visual::prompts::confirm_default("Install now?", false);
    println!("quiet confirm_default(false) → {}", if ok { "yes" } else { "no" });
    let name = rsb::visual::prompts::ask("Enter name", Some("anon"));
    println!("quiet ask(name, default=anon) → {}", name);
    let choice = rsb::visual::prompts::select("Pick one", &["alpha", "beta", "gamma"], Some(1));
    println!("quiet select(default=beta) → {}", choice);

    // Test macro forms
    println!("\n--- Testing Macro Forms ---");
    let macro_confirm = confirm!("Test macro confirm?");
    println!("auto-yes confirm! macro → {}", if macro_confirm { "yes" } else { "no" });

    let macro_ask = ask!("Test macro ask", "default-value");
    println!("quiet ask! macro → {}", macro_ask);

    let macro_select = select!("Test macro select", &["option1", "option2", "option3"], 2);
    println!("quiet select! macro → {}", macro_select);

    set_var("opt_quiet", "");
    #[cfg(feature = "visual")]
    {
        use rsb::visual::colors::colored;
        println!("{}", colored("{yellow}?{reset} This line shows the prompt style"));
        println!("{}", colored("{green}✓{reset} Prompts MVP (RSB-004) implemented with thin macros"));
    }
}
