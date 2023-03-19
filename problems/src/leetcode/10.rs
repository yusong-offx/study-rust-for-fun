
use std::cmp::{min, max};
struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut answer = 0;
        let (mut l, mut r) = (0, height.len() - 1);
        while l < r {
            let vol = (r - l) as i32 * min(height[l], height[r]);
            answer = max(vol, answer);
            if height[l] <= height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        answer
    }
}

fn main() {
    assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
    assert_eq!(Solution::max_area(vec![1,1]), 1);
    assert_eq!(Solution::max_area(vec![1,2,4,3]), 4);
}