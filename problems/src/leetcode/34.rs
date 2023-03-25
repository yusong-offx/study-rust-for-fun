struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let find = match nums.binary_search(&target) {
            Ok(loc) => loc as i32,
            _ => return vec![-1, -1]
        };
        let (mut left, mut right) = (find, find);
        while left - 1 >= 0 && nums[(left - 1) as usize] == target {
            left -= 1;
        }
        while right + 1 < nums.len() as i32 && nums[(right + 1) as usize] == target {
            right += 1;
        }
        vec![left, right]
    }
}

fn main() {
    Solution::search_range(vec![5,7,7,8,8,10], 8);
}