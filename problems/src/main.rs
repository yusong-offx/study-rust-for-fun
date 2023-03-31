use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        false
    }
}

fn main() {
    // assert!(Solution::is_match(String::from("aa"), String::from("a")));
    // assert!(Solution::is_match(String::from("ab"), String::from(".*c")) == false);
    assert!(!Solution::is_match(String::from("aa"), String::from("a")));
    assert!(Solution::is_match(String::from("aaa"), String::from("*")));
    assert!(!Solution::is_match(String::from("cb"), String::from("?a")));
}