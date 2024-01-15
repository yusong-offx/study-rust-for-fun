struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut left_max, mut right_max) = (i32::MIN, i32::MIN);
        let (mut left, mut right) = (0, height.len() - 1);
        let mut acc = 0;
        while left <= right {
            if height[left] <= height[right] {
                left_max = std::cmp::max(left_max, height[left]);
                acc += left_max - height[left];
                left += 1;
            } else {
                right_max = std::cmp::max(right_max, height[right]);
                acc += right_max - height[right];
                right -= 1;
            }
        }
        acc
    }
}

fn main() {
    let answer = Solution::trap(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
    println!("The answer is: {}", answer);
}
