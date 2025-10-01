use rsb::prelude::*;

#[test]
fn uat_jobs_basic_demo() {
    println!("\n=== UAT: Jobs (sleep, bench, jobs) ===\n");

    // Sleep demo
    println!("sleep_ms(10) ...");
    rsb::jobs::sleep_ms(10);

    // Benchmark demo
    let d = rsb::jobs::bench("uat:sum", || {
        let mut s = 0u64;
        for i in 0..50_000 {
            s += i;
        }
        let _ = s;
    });
    println!("bench: uat:sum -> {:?}", d);

    // Background job demo
    let jid = rsb::jobs::start_background("echo 'uat job'");
    println!("started job: {}", jid);
    let status = rsb::jobs::wait(jid, Some(2)).unwrap_or(-1);
    println!("wait: job {} -> status {}", jid, status);

    // List jobs (may be empty if completed)
    let jobs = rsb::jobs::list_jobs();
    if jobs.is_empty() {
        println!("jobs: <none>");
    }
    for (id, cmd) in jobs {
        println!("jobs: [{}] {}", id, cmd);
    }
}
