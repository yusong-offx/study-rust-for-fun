struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut strs = strs;
        strs.sort();
        let (left, right) = (strs.first().unwrap().as_bytes(), strs.last().unwrap().as_bytes());
        for i in 0..left.len() {
            if left[i] != right[i] {
                return String::from_utf8(left[0..i].to_vec()).unwrap();
            }
        }
        return String::from_utf8(left.to_vec()).unwrap()
    }
}

fn main() {
    assert_eq!(Solution::longest_common_prefix(vec!["flower".to_string(),"flow".to_string(),"flight".to_string()]), "fl".to_string());
    assert_eq!(Solution::longest_common_prefix(vec!["a".to_string(),"a".to_string()]), "a".to_string());

}