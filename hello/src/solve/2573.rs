use std:: {
    io::{self, Read},
};

fn main() {
    let mut buffer: String = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut info = buffer
        .split_ascii_whitespace()
        .map(|x| x.parse::<i32>().unwrap());
    let rows = info.next().unwrap() as usize;
    let cols = info.next().unwrap() as usize;
    let mut graph: Box<Vec<Vec<i32>>> = Box::new(vec![Vec::with_capacity(cols); rows]);
    for i in 0..rows {
        for _ in 0..cols {
            graph[i].push(info.next().unwrap());
        }
    }
    println!("{:?}", maltdown(&mut graph));
}

fn maltdown(graph :&mut Box<Vec<Vec<i32>>>) -> i32 {
    let delta: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut ret = 0;
    let mut stack: Vec<(usize, usize, i32)> = Vec::with_capacity(graph.len() * graph[0].len() / 2);
    loop {
        ret += 1;
        for i in 0..graph.len() {
            for j in 0..graph[0].len() {
                if graph[i][j] > 0 {
                    let mut acc = 0;
                    for k in 0..4 {
                        let nx = i as i32 + delta[k].0;
                        let ny = j as i32 + delta[k].1;
                        if 0 <= nx && nx < graph.len() as i32 && 0 <= ny && ny < graph[0].len() as i32 && graph[nx as usize][ny as usize] == 0{
                            acc += 1;
                        }
                    }
                    stack.push((i, j, acc));
                }
            }
        }
        while !stack.is_empty() {
            let now = stack.pop().unwrap();
            graph[now.0][now.1] -= now.2;
            if graph[now.0][now.1] < 0 {
                graph[now.0][now.1] = 0;
            }
        }
        let mut flag = true;
        'outer:for i in 0..graph.len() {
            for j in 0..graph[0].len() {
                if graph[i][j] > 0 {
                    flag = false;
                    break 'outer
                }
            }
        }
        if flag {
            return 0;
        }
        // println!("{ret}");
        // for i in 0..graph.len() {
        //     println!("{:?}", graph[i]);
        // }
        if count_island(graph) { break }
    }
    ret
}

fn count_island(graph :&mut Box<Vec<Vec<i32>>>) -> bool {
    let mut visited: Box<Vec<Vec<bool>>> = Box::new(vec![vec![false; graph[0].len()]; graph.len()]);
    let mut count = 0;
    for i in 0..graph.len() {
        for j in 0..graph[0].len() {
            if graph[i][j] > 0 && !visited[i][j]{
                count += 1;
                if count >= 2 {
                    return true
                }
                bfs(i, j, graph, &mut visited);
                // println!("visited");
                // for i in 0..graph.len() {
                //     println!("{:?}", visited[i]);
                // }
            }
        }
    }
    false
}

fn bfs(x: usize, y: usize, graph :&mut Box<Vec<Vec<i32>>>, visited: &mut Box<Vec<Vec<bool>>>) {
    let delta: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut stack: Vec<(usize, usize)> = Vec::with_capacity(4);
    stack.push((x, y));
    visited[x][y] = true;
    while !stack.is_empty() {
        let now = stack.pop().unwrap();
        for k in 0..4 {
            let nx = now.0 as i32 + delta[k].0;
            let ny = now.1 as i32 + delta[k].1;
            if 0 <= nx && nx < graph.len() as i32 && 0 <= ny && ny < graph[0].len() as i32 && !visited[nx as usize][ny as usize] && graph[nx as usize][ny as usize] > 0{
                stack.push((nx as usize, ny as usize));
                visited[nx as usize][ny as usize] = true;
            }
        }
    }
}