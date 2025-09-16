use rsb::prelude::*;

#[test]
fn random_macros_and_dicts() {
    // Random string lengths
    assert_eq!(rand_alnum!(5).len(), 5);
    assert_eq!(rand_alpha!(6).len(), 6);
    assert_eq!(rand_hex!(7).len(), 7);
    assert_eq!(rand_string!(8).len(), 8);
    assert_eq!(rand_uuid!().len(), 36);

    // rand_range inclusive
    let v = rand_range!(1, 3);
    assert!(v >= 1 && v <= 3);

    // gen_dict and rand_dict
    gen_dict!(alnum, 3, into: "WORDS");
    assert_eq!(array_length("WORDS"), 3);
    let w = rand_dict!("WORDS");
    assert!(!w.is_empty());
}
// json_dict_random
