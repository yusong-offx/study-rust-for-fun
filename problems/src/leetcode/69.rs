struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {return 0}
        let (mut start, mut end) = (1, x);
        let mut mid= 1;
        while start <= end {
            mid = start + (end - start) / 2;
            if mid == x / mid{
                return mid
            } else if mid < x / mid{
                start = mid + 1;
            } else {
                end = mid - 1;
            }
        }
        end
    }
}

fn main() {
    assert_eq!(Solution::my_sqrt(4), 2);
    assert_eq!(Solution::my_sqrt(8), 2);
    assert_eq!(Solution::my_sqrt(1), 1);
    assert_eq!(Solution::my_sqrt(3), 1);
}