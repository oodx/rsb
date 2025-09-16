use rsb::prelude::*;

#[test]
fn time_math_macros() {
    // date variants should produce non-empty strings
    assert!(!date!().is_empty());
    assert!(!date!(iso).is_empty());
    assert!(!date!(epoch).is_empty());
    assert!(!date!(human).is_empty());
    assert!(!date!("%Y").is_empty());

    // math evaluates and updates context
    set_var("A", "10");
    math!("A += 5");
    assert_eq!(get_var("A"), "15");

    // sleep macro should not panic
    sleep!(ms: 5);

    // benchmark returns a duration
    let d = benchmark!({
        sleep!(ms: 1);
    });
    assert!(d.as_nanos() >= 0);
}
// time_math
