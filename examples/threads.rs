// Example: threads & jobs utilities
use rsb::prelude::*;

fn main() {
    // Sleep
    rsb::threads::sleep_ms(50);

    // Benchmark a simple operation
    let _ = rsb::threads::bench("add", || {
        let mut acc = 0;
        for i in 0..10_000 {
            acc += i;
        }
        let _ = acc;
    });

    // Start a background job and wait
    let jid = rsb::threads::start_background("echo 'hello from job'");
    let status = rsb::threads::wait(jid, Some(3)).unwrap_or(-1);
    echo!("job {} -> status {}", jid, status);

    // List jobs (likely empty after wait)
    for (id, cmd) in rsb::threads::list_jobs() {
        echo!("running: [{}] {}", id, cmd);
    }
}
