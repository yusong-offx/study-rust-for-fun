struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut graph = vec![vec![1; n as usize]; m as usize];
        for i in 1..m as usize {
            for j in 1..n as usize {
                graph[i][j] = graph[i-1][j] + graph[i][j-1];
            }
        }
        graph[(m-1) as usize][(n-1) as usize]
    }
}

fn main() {
    assert_eq!(Solution::unique_paths(3, 7), 28)
}