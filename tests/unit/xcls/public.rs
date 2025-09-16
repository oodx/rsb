use rsb::xcls::*;

#[test]
fn xgrep_and_xfilter_smoke() {
    let input = "a1\na2\nb1";
    let res = xgrep(input).filter_lines(|l| l.starts_with('a')).to_string();
    assert_eq!(res, "a1\na2");

    let res2 = xfilter(input)
        .filter_transform(|l| l.contains('1'), |l| l.to_uppercase())
        .to_string();
    assert_eq!(res2, "A1\nB1");
}

#[test]
fn xsed_transform_chain() {
    let out = xsed("k=\"a\"; v=\"b\"")
        .transform_values(|v| v.to_uppercase())
        .to_string();
    assert!(out.contains("A") && out.contains("B"));
}
