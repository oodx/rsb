use rsb::prelude::*;

#[test]
fn args_processing_stub() {
    let raw = vec!["cmd".into(), "--flag".into(), "pos".into()];
    let args = Args::new(&raw);
    assert_eq!(args.get(1), "--flag");
}
