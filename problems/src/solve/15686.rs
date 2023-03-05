use std:: {
    cmp::{min},
    io::{self, Read},
    collections::{HashSet},
};

struct Info {
    container: HashSet<usize>,
    chichen: Vec<(u8, u8)>,
    human: Vec<(u8, u8)>,
    chichen_lim: u8,
    answer: i32
}

fn dfs(info:&mut Box<Info>, start: usize, depth: u8) {
    if depth == info.chichen_lim {
        let mut now_answer = 0;
        for (x, y) in &info.human {
            let mut shortest = i32::MAX;
            for i in &info.container {
                let (w, v) = info.chichen[*i];
                shortest = min(shortest, 
                    (*x as i32 - w as i32).abs()+ (*y as i32 - v as i32).abs()
                );
                
            }
            now_answer += shortest;
        }
        info.answer = min(info.answer, now_answer);
        return ;
    }
    for i in start..info.chichen.len() {
        info.container.insert(i);
        dfs(info, i+1, depth+1);
        info.container.remove(&i);
    }
}

fn main() {
    let mut buffer: String = String::new();
    let mut stdin = io::stdin().lock();
    stdin.read_to_string(&mut buffer).unwrap();

    let mut input = buffer
        .split_ascii_whitespace()
        .map(|x| x.parse::<u8>().unwrap());
    let graph_size: u8 = input.next().unwrap();
    let chicken_house_lim: u8 = input.next().unwrap();

    let mut chichen_house_loc: Vec<(u8, u8)> = Vec::with_capacity(chicken_house_lim as usize);
    let mut human_house_loc: Vec<(u8, u8)> = Vec::with_capacity(chicken_house_lim as usize);

    let mut graph: Vec<Vec<u8>> = Vec::with_capacity(graph_size as usize);
    for i in 0..graph_size {
        let mut rows: Vec<u8> = Vec::with_capacity(graph_size as usize);
        for j in 0..graph_size {
            let now = input.next().unwrap();
            rows.push(now);
            match now {
                1 => human_house_loc.push((i, j)),
                2 => chichen_house_loc.push((i, j)),
                _ => ()
            }
        }
        graph.push(rows);
    }
    let mut info = Box::new(Info{
        container: HashSet::with_capacity(chicken_house_lim as usize),
        chichen: chichen_house_loc,
        human: human_house_loc,
        chichen_lim: chicken_house_lim,
        answer: i32::MAX
    });
    
    dfs(&mut info, 0, 0);
    print!("{}", info.answer);
}