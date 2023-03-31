use std::collections::HashMap;

struct Solution;

impl Solution {
    fn recur(s: &Vec<char>, p: &Vec<char>, i: usize, j: usize, cache: &mut HashMap<(usize, usize), bool>) -> bool {
        // println!("{i} | {j}");
        if let Some(&c) = cache.get(&(i, j)) {
            println!("{c}");
            return c
        }
        if i >= s.len() && j >= p.len() {
            return true
        }
        if j >= p.len() {
            return false
        }

        let now_match = i < s.len() && (s[i] == p[j] || p[j] == '.');
        if j + 1 < p.len() && p[j+1] == '*' {
            let c = Self::recur(s, p, i, j+2, cache) || (now_match && Self::recur(s, p, i+1, j, cache));
            cache.insert((i, j), c);
            return c
        }
        if now_match {
            let c = Self::recur(s, p, i+1, j+1, cache);
            cache.insert((i, j), c);
            return c
        }
        cache.insert((i, j), false);
        false
    }

    pub fn is_match(s: String, p: String) -> bool {
        Self::recur(&s.chars().collect(), &p.chars().collect(), 0, 0, &mut Box::new(HashMap::new()))
    }
}

fn main() {
    // assert!(Solution::is_match(String::from("aa"), String::from("a")));
    assert!(Solution::is_match(String::from("aa"), String::from(".*")));
    assert!(Solution::is_match(String::from("aaa"), String::from("b")) == false);
    assert!(Solution::is_match(String::from("aab"), String::from("c*a*b")));
    assert!(Solution::is_match(String::from("ab"), String::from(".*c")) == false);
}