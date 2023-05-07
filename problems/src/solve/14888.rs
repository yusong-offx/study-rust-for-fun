use std::io::{
    Read, stdin
};


fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut info = buf
        .split_ascii_whitespace()
        .map(|x| x.parse::<i32>().unwrap());

    let length = info.next().unwrap() as usize;

    let mut nums = Vec::with_capacity(length);
    for _ in 0..length {
        nums.push(info.next().unwrap());
    }

    // 0+; 1-; 2*; 3/;
    let mut ops = Vec::with_capacity(length-1);
    for i in 0..4 {
        for _ in 0..info.next().unwrap() {
            ops.push(i);
        }
    }
    let answer = permutations(&nums, &ops);
    print!("{}\n{}", answer[0], answer[1]);
}

fn recur(
    nums: &Vec<i32>,
    ops: &Vec<i32>,
    visited: &mut Vec<bool>,
    buf: &mut Vec<i32>,
    answer: &mut [i32; 2]
) {
    if buf.len() == ops.len() {
        let mut v = nums[0];
        for i in 1..nums.len() {
            match buf[i-1] {
                0 => v += nums[i],
                1 => v -= nums[i],
                2 => v *= nums[i],
                3 => v /= nums[i],
                _ => unreachable!(),
            }
        }
        answer[0] = i32::max(answer[0], v);
        answer[1] = i32::min(answer[1], v);
        return
    }
    for i in 0..ops.len() {
        if visited[i] {
            continue
        }
        visited[i] = true;
        buf.push(ops[i]);

        recur(nums, ops, visited, buf, answer);

        visited[i] = false;
        buf.pop();
    }
}

fn permutations(nums: &Vec<i32>, ops: &Vec<i32>) -> [i32; 2] {
    let mut answer = [i32::MIN, i32::MAX];
    let mut visited = vec![false; ops.len()];
    recur(nums, ops, &mut visited,&mut Vec::with_capacity(ops.len()), &mut answer);
    answer
}