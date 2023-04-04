struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort();

        let mut answer = Vec::from([intervals[0].clone()]);
        for now in intervals.into_iter().skip(1) {
            let prev = answer.last_mut().unwrap();
            if prev[1] >= now[0] {
                *prev = vec![prev[0], now[1].max(prev[1])];
            } else {
                answer.push(now);
            }
        }
        answer
    }
}

fn main() {
    assert_eq!(Solution::merge(vec![vec![1,3],vec![2,6],vec![8,10],vec![15,18]]), vec![vec![1,6],vec![8,10],vec![15,18]]);
    // assert_eq!(Solution::merge(vec![vec![1,4],vec![4,5]]), vec![vec![1,5]])
    assert_eq!(Solution::merge(vec![vec![1,4],vec![2,3]]), vec![vec![1,4]])
}