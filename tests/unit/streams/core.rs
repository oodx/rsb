use rsb::prelude::*;

#[test]
fn streams_builders_and_ops() {
    // from_string → grep → sed → cut → sort → unique → to_string
    let out = Stream::from_string("z,2\ny,1\nz,2\n")
        .grep("z")
        .sed("z","Z")
        .cut(1, ",")
        .sort()
        .unique()
        .to_string();
    assert_eq!(out, "Z");

    // to_file + read back
    let tmp = std::env::temp_dir().join("rsb_stream.txt");
    let tmp_s = tmp.to_string_lossy();
    Stream::from_string("a\nb\n").to_file(&tmp_s);
    assert_eq!(read_file(&tmp_s), "a\nb\n");

    // tee appends and leaves on stdout chain
    let tmp2 = std::env::temp_dir().join("rsb_stream2.txt");
    let tmp2s = tmp2.to_string_lossy();
    let s = pipe!("1\n2\n2\n")
        .tee(&tmp2s)
        .unique()
        .to_string();
    assert_eq!(s, "1\n2");
    assert!(read_file(&tmp2s).contains("2"));
}

