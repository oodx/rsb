// moved from tests/param_test_uat.rs
use rsb::prelude::*;

#[test]
fn uat_param_usage_demo() {
    println!("\n=== UAT: param! usage demo ===\n");
    set_var("FOO", "/home/user/file.txt");
    set_var("EMPTY", "");
    set_var("WORD", "hello");
    set_var("CAPS", "WORLD");
    set_var("NUMS", "0123456789");
    println!("FOO='{}'", param!("FOO"));
    println!("EMPTY='{}'", param!("EMPTY"));
    println!(
        "default: EMPTY -> '{}'",
        param!("EMPTY", default: "fallback")
    );
    println!("default: FOO   -> '{}'", param!("FOO", default: "fallback"));
    println!("alt:     EMPTY -> '{}'", param!("EMPTY", alt: "has_value"));
    println!("alt:     FOO   -> '{}'", param!("FOO", alt: "has_value"));
    println!("len: FOO -> {}", param!("FOO", len));
    println!("sub: NUMS 2,3 -> '{}'", param!("NUMS", sub: 2, 3));
    println!("sub: FOO  0,5 -> '{}'", param!("FOO", sub: 0, 5));
    println!(
        "prefix: FOO '/home' -> '{}'",
        param!("FOO", prefix: "/home")
    );
    println!("suffix: FOO '.txt'  -> '{}'", param!("FOO", suffix: ".txt"));
    println!(
        "replace: FOO '/'=>'_'      -> '{}'",
        param!("FOO", replace: "/" => "_")
    );
    println!(
        "replace: FOO '/'=>'_' all  -> '{}'",
        param!("FOO", replace: "/" => "_", all)
    );
    println!("upper first: WORD -> '{}'", param!("WORD", upper: first));
    println!("upper:        WORD -> '{}'", param!("WORD", upper));
    println!("lower first:  CAPS -> '{}'", param!("CAPS", lower: first));
    println!("lower:        CAPS -> '{}'", param!("CAPS", lower));
    let args = rsb::cli::Args::new(&[
        "bin".into(),
        "cmd".into(),
        "--verbose".into(),
        "--config=demo.conf".into(),
        "-d".into(),
    ]);
    options!(&args);
    println!(
        "opt_verbose='{}' opt_config='{}' opt_d='{}'",
        param!("opt_verbose"),
        param!("opt_config"),
        param!("opt_d")
    );
    println!("\n✓ param! usage demo complete\n");
}
