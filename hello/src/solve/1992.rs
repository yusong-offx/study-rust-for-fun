use std::{
    io::{self, Read},
};

fn main() {
    let mut buf: String = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut info = buf
        .split_ascii_whitespace()
        .into_iter();
    let n: usize = info.next().unwrap()
        .parse().unwrap();
    let graph: Vec<Vec<char>> = info.map(|x| x.chars().collect()).collect();
    
    print!("{}", compress(n, graph));
}

fn compress(n: usize, graph: Vec<Vec<char>>) -> String{
    let mut answer: String = String::new();
    if !format_checker(&graph) {
        // Divide 4 parts
        answer.push('(');
        let m = n / 2;
        let up = &graph[0..m];
        let down = &graph[m..graph.len()];

        let mut up_left: Vec<Vec<char>> = Vec::with_capacity(m);
        let mut up_right: Vec<Vec<char>> = Vec::with_capacity(m);
        for l in up {
            up_left.push(l[0..m].to_vec());
            up_right.push(l[m..n].to_vec());
        }

        let mut down_left: Vec<Vec<char>> = Vec::with_capacity(m);
        let mut down_right: Vec<Vec<char>> = Vec::with_capacity(m);
        for l in down {
            down_left.push(l[0..m].to_vec());
            down_right.push(l[m..n].to_vec());
        }
        
        answer.push_str(&compress(m, up_left));
        answer.push_str(&compress(m, up_right));
        answer.push_str(&compress(m, down_left));
        answer.push_str(&compress(m, down_right));
        answer.push(')');
    } else {
        // First char
        answer.push(graph[0][0]);
    }
    answer
}

// Check all same char
fn format_checker(graph: &Vec<Vec<char>>) -> bool {
    let mut one: bool = false;
    let mut zero: bool = false;
    for l in graph {
        for c in l {
            match c {
                '1' => one = true,
                '0' => zero = true,
                _ => (),
            };
            if one && zero {
                return false;
            }
        }
    }
    true
}

// 범위로 넘기면 메모리 아낄수 있음.