use std::{
    io::{self, Read},
};

fn main() {
    let mut buffer: String = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut info = buffer
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let test_cases: usize = info.next().unwrap();
    for _ in 0..test_cases {
        let (x, y, z) = (info.next().unwrap(), info.next().unwrap(), info.next().unwrap());
        let mut sol: Solution = Solution { graph: vec![vec![0; y]; x], answer: 0 };
        for _ in 0..z {
            let (a, b) = (info.next().unwrap(), info.next().unwrap());
            sol.graph[a][b] = 1;
        }
        sol.solve();
        // for l in sol.graph {
        //     println!("{:?}", l);
        // }
        println!("{}", sol.answer);
    }
}
struct Solution {
    graph: Vec<Vec<i32>>,
    answer: usize
}

impl Solution {
    fn solve(&mut self) {
        for i in 0..self.graph.len() {
            for j in 0..self.graph[0].len() {
                if self.graph[i][j] == 1 {
                    self.dfs(i, j);
                    self.answer += 1;
                }
            }
        }
    }

    fn dfs(&mut self, x: usize, y: usize) {
        const DELTA: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut stack: Vec<(usize, usize)> = Vec::with_capacity(4);
        stack.push((x, y));
        while !stack.is_empty() {
            let (a, b) = stack.pop().unwrap();
            for (k, w) in DELTA {
                let na = a as i32 + k;
                let nb = b as i32 + w;
                if 0 <= na && na < self.graph.len() as i32 && 0 <= nb && nb < self.graph[0].len() as i32 && self.graph[na as usize][nb as usize] == 1 {
                    self.graph[na as usize][nb as usize] = 0;
                    stack.push((na as usize, nb as usize));
                }
            }
        }
    }
}
