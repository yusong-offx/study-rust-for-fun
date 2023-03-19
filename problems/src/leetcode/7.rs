struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut left = 0;
        if x < 0 {
            left = 1;
        }
        let mut x = x.to_string().as_bytes().to_owned();
        let mut right = x.len()-1;
        while left < right {
            x.swap(left, right);
            left += 1; right -= 1;
        }
        std::str::from_utf8(&x).unwrap().parse().unwrap_or(0)
    }
}

fn main() {
    assert_eq!(Solution::reverse(1534236469), 0);
}