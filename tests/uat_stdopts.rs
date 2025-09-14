// UAT Stdopts Demo: Shows short -> descriptive expansion visibly

use rsb::prelude::*;

#[test]
fn uat_stdopts_demo() {
    // Simulate CLI args with short flags
    let args = vec![
        "rsb-uat".to_string(),
        "-d".to_string(), "-q".to_string(), "-t".to_string(),
        "-D".to_string(), "-y".to_string(), "-s".to_string(),
    ];
    let args = rsb::cli::Args::new(&args);
    options!(&args);

    println!("\n=== UAT: Stdopts Demo ===\n");
    println!("Input flags: -d -q -t -D -y -s\n");

    // Always present short flags
    for (k, v) in [
        ("opt_d", get_var("opt_d")),
        ("opt_q", get_var("opt_q")),
        ("opt_t", get_var("opt_t")),
        ("opt_D", get_var("opt_D")),
        ("opt_y", get_var("opt_y")),
        ("opt_s", get_var("opt_s")),
    ] {
        println!("{:>12} = {}", k, if v.is_empty() { "(empty)" } else { &v });
    }

    // Feature-dependent expansions
    #[cfg(feature = "stdopts")]
    {
        println!("\nDescriptive expansions (stdopts=ON):");
        for (k, v) in [
            ("opt_debug", get_var("opt_debug")),
            ("opt_quiet", get_var("opt_quiet")),
            ("opt_trace", get_var("opt_trace")),
            ("opt_dev_mode", get_var("opt_dev_mode")),
            ("opt_yes", get_var("opt_yes")),
            ("opt_safe", get_var("opt_safe")),
        ] {
            println!("{:>12} = {}", k, if v.is_empty() { "(empty)" } else { &v });
        }
    }

    #[cfg(not(feature = "stdopts"))]
    {
        println!("\nDescriptive expansions (stdopts=OFF): skipped");
    }
}
