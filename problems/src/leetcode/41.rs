use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut num = BinaryHeap::new();
        for i in nums {
            if i > 0 &&  {
                num.push(-i);
            }
        }
        let mut now = 1;
        while !num.is_empty() {
            if num.pop().unwrap() != now {
                return now
            }
            now += 1;
        }
        now
    }
}

fn main() {

}