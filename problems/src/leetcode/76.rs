struct Solution;

use std::collections::{HashMap};

impl Solution {
    fn all_zero(s: &HashMap<char, i32>) -> bool {
        for (_, i) in s {
            if *i > 0 {
                return false
            }
        }
        true
    }

    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() { return String::new() }
        let mut counter = HashMap::new();
        for c in t.chars() {
            counter.entry(c)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
        let mut left = 0;
        let mut answer = (0, s.len() + 1);
        let s: Vec<char> = s.chars().collect();
        let mut start = 0;
        for i in 0..s.len() {
            if counter.contains_key(&s[i]) {
                start = i;
                left = i;
                break;
            }
        }
        for right in start..s.len() {
            if counter.contains_key(&s[right]) {
                *counter.get_mut(&s[right]).unwrap() -= 1;
            }


            while Self::all_zero(&counter) {
                if right - left < answer.1 - answer.0 {
                    answer = (left, right);
                }
                *counter.get_mut(&s[left]).unwrap() += 1;
                for i in left+1..s.len() {
                    if counter.contains_key(&s[i]) {
                        left = i;
                        break;
                    }
                }
            }

        }
        if answer.1 > s.len() {
            return String::new()
        }
        s[answer.0..=answer.1].to_owned().iter().collect()
    }
}

fn main() {
    assert_eq!(
        Solution::min_window(
            String::from("ADOBECODEBANC"), 
            String::from("ABC")
        ), 
        String::from("BANC")
    );
}