use rsb::prelude::*;

fn handle_init(_args: rsb::cli::Args) -> i32 {
    println!("pre:init");
    println!("opt_verbose={}", rsb::global::get_var("opt_verbose"));
    0
}

fn handle_run(_args: rsb::cli::Args) -> i32 {
    println!("dispatch:run");
    println!("opt_config={}", rsb::global::get_var("opt_config"));
    0
}

fn main() {
    let args = bootstrap!();
    options!(&args);

    if pre_dispatch!(&args, { "init" => handle_init }) {
        return;
    }

    dispatch!(&args, { "run" => handle_run });
}
