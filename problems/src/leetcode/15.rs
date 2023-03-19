

struct Solution;

// use std::collections::HashSet;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answer: Vec<Vec<i32>> = Vec::new();
        let mut nums = nums;
        nums.sort_unstable();

        for first in 0..nums.len()-2 {
            if first > 0 && nums[first] == nums[first-1] {
                continue;
            }

            let (mut second, mut third) = (first+1, nums.len()-1);
            while second < third {
                let sum = nums[first] + nums[second] + nums[third];
                if 0 == sum {
                    answer.push(vec![nums[first], nums[second], nums[third]]);
                }

                if sum < 0 {
                    let second_value = nums[second];
                    while {
                        second += 1;
                        second < third && nums[second] == second_value
                    } {}
                } else {
                    let third_value = nums[third];
                    while {
                        third -= 1;
                        third > second && nums[third] == third_value
                    } {}
                }
            }
        }

        answer
    }
}

fn main() {
    assert_eq!(Solution::three_sum(vec![-1,0,1,2,-1,-4]), vec![vec![-1,-1,2], vec![-1,0,1]]);
    assert_eq!(Solution::three_sum(vec![-2,0,1,1,2]), vec![vec![-2,0,2], vec![-2,1,1]]);
    assert_eq!(Solution::three_sum(vec![-1,0,1,2,-1,-4,-2,-3,3,0,4]), vec![[-4,0,4],[-4,1,3],[-3,-1,4],[-3,0,3],[-3,1,2],[-2,-1,3],[-2,0,2],[-1,-1,2],[-1,0,1]]);
    assert_eq!(Solution::three_sum(vec![0,0,0]), vec![vec![0, 0, 0]]);

}