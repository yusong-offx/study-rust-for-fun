struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut answer = 0;
        let mut stack: Vec<(usize, i32)> = Vec::new();
        for (i, h) in heights.iter().enumerate() {
            let mut start = i;
            while !stack.is_empty() && stack[stack.len()-1].1 > *h {
                let (idx, height) =  stack.pop().unwrap();
                answer = i32::max(answer, (i-idx) as i32 * height);
                start = idx;
            }
            stack.push((start, *h));
        }
        for (i, h) in stack {
            answer = i32::max(answer, h * (heights.len()-i) as i32);
        }
        answer
    }
}

fn main() {
    // assert_eq!(Solution::largest_rectangle_area(vec![2,1,5,6,2,3]), 0);
    assert_eq!(Solution::largest_rectangle_area(vec![0, 0, 0]), 0);
}