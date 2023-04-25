struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![1; n+1];
        for now in 2..=n {
            dp[now] = dp[now-1] + dp[now-2];
        }
        dp[n]
    }
}

fn main() {
    for i in 0..45 {
        print!("{} | ", Solution::climb_stairs(i));
    }
    assert_eq!(Solution::climb_stairs(2), 2);
    assert_eq!(Solution::climb_stairs(3), 3);
    assert_eq!(Solution::climb_stairs(4), 5);
}