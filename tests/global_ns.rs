use rsb::prelude::*;

#[test]
fn global_ns_dunder_and_colon() {
    // Dunder set/get
    rsb::global::ns_set("app", "key", "val");
    assert_eq!(rsb::global::ns_get("app", "key"), "val");

    // Colon set/get
    rsb::global::ns_set_cc("svc", "url", "https://");
    assert_eq!(rsb::global::ns_get("svc", "url"), "https://");

    // Get all merges styles
    let items = rsb::global::ns_get_all("app");
    assert!(items.iter().any(|(k, v)| k == "key" && v == "val"));

    // Overlay to plain and back
    let n = rsb::global::ns_overlay_to_plain("app");
    assert!(n >= 1);
    assert_eq!(rsb::global::get_var("key"), "val");

    let m = rsb::global::ns_overlay_plain_to_ns("app2", &["key"], None);
    assert_eq!(m, 1);
    assert_eq!(rsb::global::ns_get("app2", "key"), "val");
}

