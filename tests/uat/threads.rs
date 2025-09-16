use rsb::prelude::*;

#[test]
fn uat_threads_basic_demo() {
    println!("\n=== UAT: Threads (sleep, bench, jobs) ===\n");

    // Sleep demo
    println!("sleep_ms(10) ...");
    rsb::threads::sleep_ms(10);

    // Benchmark demo
    let d = rsb::threads::bench("uat:sum", || {
        let mut s = 0u64; for i in 0..50_000 { s += i; }
        let _ = s;
    });
    println!("bench: uat:sum -> {:?}", d);

    // Background job demo
    let jid = rsb::threads::start_background("echo 'uat job'");
    println!("started job: {}", jid);
    let status = rsb::threads::wait(jid, Some(2)).unwrap_or(-1);
    println!("wait: job {} -> status {}", jid, status);

    // List jobs (may be empty if completed)
    let jobs = rsb::threads::list_jobs();
    if jobs.is_empty() { println!("jobs: <none>"); }
    for (id, cmd) in jobs { println!("jobs: [{}] {}", id, cmd); }
}

