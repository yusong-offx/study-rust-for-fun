
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack.len() < needle.len() {
            return -1
        }
        let needle: Vec<char> = needle.chars().collect();
        let haystack: Vec<char> = haystack.chars().collect();
        let mut answer = 0;
        for i in 0..(haystack.len()-needle.len()+1) {
            if haystack[i] == needle[0] {
                let mut j = i;
                for c in needle.iter() {
                    if haystack[j] != *c {
                        break
                    }
                    j += 1;
                };
                if j - i == needle.len() {
                    return i as i32
                }
            }
        }
        -1
    }
}