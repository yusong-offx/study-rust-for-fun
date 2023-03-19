use std::collections::HashMap;

struct Solution;

impl Solution {
    fn dfs(i:usize, j: usize, s:&[u8], p:&[u8], cache:&mut HashMap<(usize, usize), bool>) -> bool {
        if i >= s.len() && j >= p.len() {
            return true
        }
        if j >= p.len() {
            return false
        }

        let m = i < s.len() && (s[i] == p[j] || p[j] as char == '.');
        if (j + 1) < p.len() && p[j+1] as char == '*' {
            let now = Self::dfs(i, j+2, s, p, cache) || (m && Self::dfs(i+1, j, s, p, cache));
            cache
                .entry((i, j))
                .and_modify(|x| *x=now)
                .or_insert(now);
            return *cache.get(&(i, j)).unwrap()
        }
        if m {
            let now = Self::dfs(i+1, j+1, s, p, cache);
            cache
                .entry((i, j))
                .and_modify(|x| *x=now)
                .or_insert(now);
            return cache[&(i, j)]
        }
        false
    }

    pub fn is_match(s: String, p: String) -> bool {
        let (s, p) = (s.as_bytes(), p.as_bytes());
        Self::dfs(0, 0, s, p, &mut HashMap::new())
    }
}
fn main() {
    // assert!(Solution::is_match(String::from("aa"), String::from("a")));
    assert!(Solution::is_match(String::from("aa"), String::from(".*")));
    assert!(Solution::is_match(String::from("aaa"), String::from("b")) == false);
    assert!(Solution::is_match(String::from("aab"), String::from("c*a*b")));
    assert!(Solution::is_match(String::from("ab"), String::from(".*c")) == false);
}