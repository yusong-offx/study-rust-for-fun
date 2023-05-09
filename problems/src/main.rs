struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map = HashMap::new();
        for c in s.chars() {
            map.entry(c)
                .and_modify(|v| *v = true)
                .or_default();
        }
        for (i, c) in s.chars().enumerate() {
            if !*map.get(&c).unwrap() {
                return i as i32
            }
        }
        -1
    }
}

fn main() {
    assert_eq!(Solution::first_uniq_char(String::from("leetcode")), 0);
    assert_eq!(Solution::first_uniq_char(String::from("loveleetcode")), 2);
}