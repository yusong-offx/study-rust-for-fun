struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        match x < 0 {
            true => false,
            false => {
                let atoi: Vec<char> = x.to_string().chars().collect();
                let (mut left, mut right) = (0, atoi.len() - 1);
                while left < right {
                    if atoi[left] != atoi[right] {
                        return false;
                    }
                    left += 1;
                    right -= 1;
                }
                true
            }
        }
    }
}

fn main() {
    assert_eq!(Solution::is_palindrome(-121), false);
}
