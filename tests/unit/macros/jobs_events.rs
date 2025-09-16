// jobs_events
use rsb::prelude::*;

#[test]
fn event_and_trap_macros() {
    use std::sync::{Arc, Mutex};
    let hit = Arc::new(Mutex::new(0));
    let hit2 = hit.clone();
    trap!(move |_data: &EventData| {
        let mut n = hit2.lock().unwrap();
        *n += 1;
    }, on: "COMMAND_ERROR");

    // Emit command error via run! failing silently in test mode
    let _ = run!("sh -c 'exit 3'");
    assert_eq!(*hit.lock().unwrap(), 1);
}

#[test]
fn lock_macros_basic() {
    let lock_path = std::env::temp_dir().join("rsb_lock_test.lock");
    let s = lock_path.to_string_lossy();
    lock!(s.as_ref());
    unlock!(s.as_ref());
}

