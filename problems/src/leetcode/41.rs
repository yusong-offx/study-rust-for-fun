use std::collections::BTreeSet;

struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut num = BTreeSet::new();
        for i in nums {
            if i > 0 {
                num.insert(i);
            }
        }
        let mut checker: Vec<&i32> = Vec::with_capacity(num.len());
        for x in num.iter() {
            checker.push(x);
        }
        let mut answer = 1;
        for i in 0..num.len() {
           if *checker[i] != answer {
                return answer
           }
           answer += 1;
        }
        answer
        // for i in 1..=num.len() {
        //     let i = i as i32;
        //     if !num.contains(&i) {
        //         return i
        //     }
        // }
        // (num.len()+1) as i32
    }
}

fn main() {
    println!("{}", Solution::first_missing_positive(vec![1,2,2,1,3,1,0,4,0]));
    println!("{}", Solution::first_missing_positive(vec![3,4,-1,1]));
}