use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
  pub fn is_valid(s: String) -> bool {
      let conver_to_close = HashMap::from(
        [(')', '('), ('}', '{'), (']', '[')]
      );
      let open_bracket = HashSet::from(['(', '[', '{']);
      let mut stack = Vec::new();
      for bracket in s.chars() {
        if open_bracket.contains(&bracket) {
          stack.push(bracket);
        } else {
          if !(stack.last() == conver_to_close.get(&bracket)) {
            return false
          }
          stack.pop();
        }
      }
      if stack.is_empty() {
        return true
      }
      false
  }
}

fn main() {
  assert!(Solution::is_valid("()".to_string()));
  assert!(Solution::is_valid("()[]{}".to_string()));
  assert!(!Solution::is_valid("(]".to_string()));
}