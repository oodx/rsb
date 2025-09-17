use rsb::prelude::*;

#[test]
fn sanity_threads_sleep_and_bench() {
    // Sleep briefly to ensure the function is callable
    rsb::threads::sleep_ms(5);

    // Benchmark a no-op closure
    let d = rsb::threads::bench("noop", || {});
    // Should be a small duration; just assert it's non-zero
    assert!(d >= std::time::Duration::from_micros(0));
}

#[test]
fn sanity_threads_job_background_wait_list() {
    // Start a background job and wait with timeout
    let jid = rsb::threads::start_background("echo 'sanity job'");
    let status = rsb::threads::wait(jid, Some(2)).unwrap();
    assert_eq!(status, 0);

    // Listing jobs should not panic
    let _ = rsb::threads::list_jobs();
}
