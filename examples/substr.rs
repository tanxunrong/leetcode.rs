extern crate leetcode;

use leetcode::longest_substr_3::longest_substr;

fn main() {
    let abc = "abcabcbb";
    let size = longest_substr(abc.as_bytes(), 0);
    assert_eq!(size, 3);
}
