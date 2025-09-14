use rsb::prelude::*;

#[test]
fn sed_and_path_macros() {
    let src = "line1\nSTART\nkeep\nEND\nline9";
    let around = sed_around!(src, "START", 1);
    assert!(around.contains("line1"));

    let lines = sed_lines!(src, 2, 4);
    assert_eq!(lines, "START\nkeep\nEND");

    let replaced = sed_template!("X", "keep", src);
    assert!(replaced.contains("X"));

    // path helpers
    let path = "/tmp/test/file.txt";
    let canon = path_canon!("/tmp");
    assert!(!canon.is_empty());
    path_split!(path, into: "P");
    assert_eq!(get_var("P_file_name"), "file.txt");
}

#[test]
fn meta_keys_macro() {
    let tmpd = std::env::temp_dir();
    let file = tmpd.join("rsb_meta_test_file.txt");
    std::fs::write(&file, "# author : Testy\n# version: 9.9.9\nBody\n").unwrap();
    meta_keys!(file.to_str().unwrap(), into: "META");
    assert_eq!(get_var("META_author"), "Testy");
    assert_eq!(get_var("META_version"), "9.9.9");
}
// fs_data
