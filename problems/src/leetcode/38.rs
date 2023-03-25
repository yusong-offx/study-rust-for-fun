struct Solution;

impl Solution {
    fn recur(num:Vec<char>, depth: i32) -> String {
        if depth == 1 {
            let mut answer = String::with_capacity(num.len());
            for c in num {
                answer.push(c)
            }
            return answer
        }
        let (mut prev, mut cnt) = (num[0], 0);
        let mut next_num = Vec::with_capacity(num.len());
        for c in num.iter() {
            if prev == *c {
                cnt += 1;
            } else {
                next_num.extend(cnt.to_string().chars());
                next_num.push(prev);
                prev = *c; cnt = 1;
            }
        }
        next_num.extend(cnt.to_string().chars());
        next_num.push(prev);
        Self::recur(next_num, depth - 1)
    }

    pub fn count_and_say(n: i32) -> String {
        Self::recur(vec!['1'], n)
    }
}

fn main() {
    let a = Solution::count_and_say(4);
    println!("{a}");
}