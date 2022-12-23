use std::{
    cmp::{max},
    io::{self, Read},
};

struct Info<'a> {
    visited: [bool; 26],
    graph : Vec<&'a [u8]>,
    answer: usize
}

const DELTA : [(isize, isize); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

fn dfs(x: isize, y: isize, cost :usize, info :&mut Box<Info>) {
    if info.answer == 26 {
        return ;
    }
    info.answer = max(info.answer, cost);
    for (nx, ny) in DELTA {
        let dx:isize = x + nx;
        let dy:isize = y + ny;
        if (0 <= dx && dx < info.graph.len() as isize) && (0 <= dy && dy < info.graph[0].len() as isize) {
            let now = info.graph[dx as usize][dy as usize];
            if info.visited[(now - 65) as usize] {
                continue;
            }
            info.visited[(now - 65) as usize] = true;
            dfs(dx, dy, cost+1, info);
            info.visited[(now - 65) as usize] = false;
        }
    }
}

fn main() {
    let mut buf: String = String::new();
    let mut stdin = io::stdin().lock();
    stdin.read_to_string(&mut buf).unwrap();
    let mut input = buf.split_ascii_whitespace();

    input.next();
    input.next();
    let mut graph: Vec<&[u8]> = Vec::new();
    for l in input {
        graph.push(l.as_bytes());
    }

    let mut visited = [false; 26];
    visited[(graph[0][0] - 65) as usize] = true;
    let mut myinfo = Box::new(Info{
        visited : visited, graph : graph, answer : 1
    });
    dfs(0, 0, 1, &mut myinfo);

    print!("{}", myinfo.answer);
}