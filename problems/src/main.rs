use std::io::Read;

fn spread(graph: &mut Vec<Vec<i8>>, virus: &Vec<(usize, usize)>) -> i32 {
    let delta: Vec<(i8, i8)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut queue = virus.clone();
    while !queue.is_empty() {
        let (x, y) = queue.pop().unwrap();
        for (dx, dy) in delta.iter() {
            let nx = x as i8 + dx;
            let ny = y as i8 + dy;
            if nx < 0 || nx >= graph.len() as i8 || ny < 0 || ny >= graph[0].len() as i8 {
                continue;
            }
            if graph[nx as usize][ny as usize] == 0 {
                graph[nx as usize][ny as usize] = 2;
                queue.push((nx as usize, ny as usize));
            }
        }
    }
    let mut cnt = 0;
    for i in 0..graph.len() {
        for j in 0..graph[0].len() {
            if graph[i][j] == 0 {
                cnt += 1;
            }
        }
    }
    cnt
}

fn solution(graph: Vec<Vec<i8>>) -> i32 {
    let mut graph = graph;

    let mut virus = Vec::new();
    let mut can_change = Vec::new();
    for i in 0..graph.len() {
        for j in 0..graph[0].len() {
            if graph[i][j] == 0 {
                can_change.push((i, j));
            } else if graph[i][j] == 2 {
                virus.push((i, j));
            }
        }
    }

    let mut pick_change = Vec::new();
    for i in 0..can_change.len() {
        for j in i+1..can_change.len() {
            for k in j+1..can_change.len() {
                pick_change.push((i, j, k));
            }
        }
    }

    let mut answer = 0;
    for (i, j, k) in pick_change {
        graph[can_change[i].0][can_change[i].1] = 1;
        graph[can_change[j].0][can_change[j].1] = 1;
        graph[can_change[k].0][can_change[k].1] = 1;
        let temp = spread(&mut graph.clone(), &virus);
        if temp > answer {
            answer = temp;
        }
        graph[can_change[i].0][can_change[i].1] = 0;
        graph[can_change[j].0][can_change[j].1] = 0;
        graph[can_change[k].0][can_change[k].1] = 0;
    }
    answer
}

fn main() {
    // Parse
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let mut info = buffer
        .split_ascii_whitespace()
        .map(|x| x.parse::<i8>().unwrap());
    let (n, m) = (info.next().unwrap(), info.next().unwrap());
    let mut lines = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let mut line = Vec::with_capacity(m as usize);
        for _ in 0..m {
            line.push(info.next().unwrap());
        }
        lines.push(line);
    }

    let answer = solution(lines);
    println!("{}", answer);
}


