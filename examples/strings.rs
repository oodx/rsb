// Working example showcasing string helpers
use rsb::prelude::*;

fn main() {
    echo!("sub: {}", str_sub("abcdef", 2, Some(3))); // cde
    echo!("prefix: {}", str_prefix("a/b/c.txt", "*/", false)); // b/c.txt
    echo!("suffix: {}", str_suffix("file.tar.gz", "*.gz", false)); // file.tar
    echo!("replace once: {}", str_replace("a/b/c", "/", "_", false)); // a_b/c
    echo!("upper first: {}", str_upper("hello", false)); // Hello
}
