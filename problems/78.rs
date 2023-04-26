impl Solution {
    fn dfs(ret: &mut Vec<Vec<i32>>, buf: &mut Vec<i32>, nums: &Vec<i32>, n: usize, prev: usize) {
        if buf.len() == n {
            ret.push(buf.clone());
            return ;
        }
        for i in prev..nums.len() {
            buf.push(nums[i]);
            Self::dfs(ret, buf, nums, n, i+1);
            buf.pop();
        }
    }

    fn generator(nums: &Vec<i32>, n: usize) -> Vec<Vec<i32>>{
        let mut ret: Vec<Vec<i32>> = Vec::new();
        Self::dfs(&mut ret, &mut Vec::with_capacity(n), nums, n, 0);
        ret
    }

    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answer = Vec::with_capacity(nums.len());
        for i in 0..=nums.len() {
            answer.extend(Self::generator(&nums, i));
        }
        answer
    }
}