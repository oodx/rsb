use rsb::prelude::*;

#[test]
fn sanity_bash_network_and_archives_do_not_panic() {
    // Network: likely to fail if curl not installed; should not panic
    let _ = rsb::bash::http_get("https://example.com");
    let _ = rsb::bash::http_get_with_options("https://example.com", "-I");
    let _ = rsb::bash::http_post("https://example.com", "a=1");

    // Archives: run with non-existent files; should not panic
    let res = rsb::bash::list_tar("/no/such/file.tar");
    assert!(res.status != 0);
    let res2 = rsb::bash::list_zip("/no/such/file.zip");
    assert!(res2.status != 0);
}

