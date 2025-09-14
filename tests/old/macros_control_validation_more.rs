// control_validation (additional)
use rsb::prelude::*;

#[test]
fn require_and_validate_macros() {
    // require_var passes when set
    set_var("NEEDED", "ok");
    require_var!("NEEDED");

    // require_file/dir with temp
    let tmpd = std::env::temp_dir().join("rsb_cv");
    let _ = std::fs::create_dir_all(&tmpd);
    let fpath = tmpd.join("f.txt");
    std::fs::write(&fpath, "x").unwrap();
    require_dir!(tmpd.to_str().unwrap());
    require_file!(fpath.to_str().unwrap());

    // require_command likely exists for `sh`
    require_command!("sh");

    // validate! should panic in tests when false
    let result = std::panic::catch_unwind(|| {
        validate!(false, "must fail");
    });
    assert!(result.is_err());
}

#[test]
fn export_and_src_macros() {
    // Prepare config
    let tmpd = std::env::temp_dir().join("rsb_cfg");
    let cfg = tmpd.join("app.conf");
    let _ = std::fs::create_dir_all(&tmpd);
    std::fs::write(&cfg, "FOO=BAR\nNAME=\"Alice Smith\"\n").unwrap();

    // load via src!/load_config!
    src!(cfg.to_str().unwrap());
    assert_eq!(get_var("FOO"), "BAR");
    assert_eq!(get_var("NAME"), "Alice Smith");

    // export variables
    let out = tmpd.join("export.sh");
    export!(out.to_str().unwrap());
    let exported = std::fs::read_to_string(&out).unwrap();
    assert!(exported.contains("export FOO='BAR'"));
}

#[test]
fn file_in_variants() {
    let tmpd = std::env::temp_dir().join("rsb_files");
    let _ = std::fs::create_dir_all(&tmpd);
    std::fs::write(tmpd.join("a.txt"), "A").unwrap();
    std::fs::write(tmpd.join("b.txt"), "B").unwrap();

    let mut seen = 0;
    file_in!(file in tmpd.to_str().unwrap() => {
        let _f = get_var("file");
        seen += 1;
    });
    assert!(seen >= 2);

    let mut content_len = 0;
    file_in!(file, content in tmpd.to_str().unwrap() => {
        let c = get_var("content");
        content_len += c.len();
    });
    assert!(content_len >= 2);
}

