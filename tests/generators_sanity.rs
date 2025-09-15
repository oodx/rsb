use rsb::prelude::*;

#[test]
fn test_rand_string_lengths() {
    assert_eq!(rand_alnum!(10).len(), 10);
    assert_eq!(rand_alpha!(8).len(), 8);
    assert_eq!(rand_hex!(6).len(), 6);
    assert_eq!(rand_string!(5).len(), 5);
}

#[test]
fn test_rand_uuid_format() {
    let u = rand_uuid!();
    assert_eq!(u.len(), 36);
    assert!(u.chars().filter(|&c| c == '-').count() == 4);
}

#[test]
fn test_rand_range_inclusive() {
    for _ in 0..50 {
        let v = rand_range!(1, 3);
        assert!(v >= 1 && v <= 3);
    }
    assert_eq!(rand_range!(5, 5), 5);
}

#[test]
fn test_gen_and_rand_dict_macros() {
    gen_dict!(alnum, 4, into: "WORDS");
    let words = rsb::utils::get_array("WORDS");
    assert_eq!(words.len(), 4);
    for w in words.iter() { assert!((4..=8).contains(&w.len())); }

    let pick = rand_dict!("WORDS");
    assert!(!pick.is_empty());
}

#[test]
fn test_dict_loading_and_rand_from_file() {
    let colors = dict!("src/gx/data/dict/colors.txt");
    assert!(colors.contains(&"red".to_string()));
    assert!(colors.contains(&"blue".to_string()));

    // gx adapter pick
    let pick = rsb::gx::rand_from_dict_file("src/gx/data/dict/fruits.txt");
    assert!(pick.is_some());
}

