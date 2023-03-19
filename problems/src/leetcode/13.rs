use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let roman_int = HashMap::from([('M', 1000), ('D', 500), ('C', 100), ('L', 50), ('X', 10), ('V', 5), ('I', 1)]);
        let mut prev = 0;
        let mut answer = 0;
        for now in s.chars().rev() {
            let now_val = *roman_int.get(&now).unwrap();
            if prev <= now_val {
                answer += now_val
            } else {
                answer -= now_val
            }
            prev = now_val;
        }
        answer
    }
}

fn main() {
    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
}