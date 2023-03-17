// struct Solution{}

// impl Solution {
//     pub fn longest_palindrome(s: String) -> String {
//         let vec_s: Vec<char> = s.chars().collect();
//         let is_palindrome = |left: usize, right:usize| -> bool {
//             let (mut left, mut right) = (left, right);
//             while left < right {
//                 if vec_s[left] != vec_s[right] {
//                     return false;
//                 }
//                 left += 1; right -= 1;
//             }
//             true
//         };
        
//         for length in (0..s.len()).rev() {
//             let mut left = 0;
//             for right in length..s.len() {
//                 if is_palindrome(left, right) {
//                     return  vec_s[left..=right].into_iter().collect();
//                 }
//                 left += 1;
//             }
//         }
//         vec_s[0].to_string()
//     }
// }
struct Solution{}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let mut answer = (0, 0);

        for mid in 0..s.len() {
            let (mut left, mut right) = (mid, mid);

            while right + 1 < s.len() && s[right+1] == s[left] {
                right += 1;
            }

            while left > 0 && right + 1 < s.len() && s[right+1] == s[left-1] {
                left -= 1;
                right += 1;
            }

            if answer.1 - answer.0 < right - left {
                answer = (left, right);
            }
        }

        s[answer.0..=answer.1].iter().collect()
    }
}
fn main() {
    println!("{}", Solution::longest_palindrome(String::from("aacabdkacaa")));
    println!("{}", Solution::longest_palindrome(String::from("aaaa")));
    println!("{}", Solution::longest_palindrome(String::from("cbbd")));
    println!("{}", Solution::longest_palindrome(String::from("babad")));
    println!("{}", Solution::longest_palindrome(String::from("a")));
    println!("{}", Solution::longest_palindrome(String::from("abcba")));
}