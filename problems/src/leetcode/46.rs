struct Solution;

impl Solution {
    fn recur(els: &Vec<i32>, depth: usize, visited: &mut Vec<bool>, buf: &mut Vec<i32>, answer: &mut Vec<Vec<i32>>) {
        if depth == els.len() {
            answer.push(buf.clone());
            return
        }
        for i in 0..els.len() {
            if !visited[i] {
                visited[i] = true;
                buf.push(els[i]);
                Self::recur(els, depth+1, visited, buf, answer);
                visited[i] = false;
                buf.pop();
            }
        }
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answer = Vec::with_capacity((0..nums.len()).product());
        Self::recur(&nums, 0, &mut vec![false; nums.len()],&mut Vec::with_capacity(nums.len()), &mut answer);
        answer
    }
}

fn main() {
    println!("{:?}", Solution::permute(vec![1, 2, 3]));
}