struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left = vec![0; height.len()];
        let mut right = vec![0; height.len()];
        let mut now = 0;
        for i in 0..height.len() {
            now = std::cmp::max(height[i], now);
            left[i] = now;
        }
        now = 0;
        for i in (0..height.len()).rev() {
            now = std::cmp::max(height[i], now);
            right[i] = now;
        }

        let mut answer = 0;
        for i in 0..height.len() {
            let m = std::cmp::min(left[i], right[i]) - height[i];
            if m > 0 {
                answer += m;
            }
        }
        answer    
    }
}

fn main() {
    assert_eq!(Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
    assert_eq!(Solution::trap(vec![4,2,0,3,2,5]), 9);
}