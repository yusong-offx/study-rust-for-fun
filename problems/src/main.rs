use std::{
    io::{
        self, Read
    },
    collections::{
      HashSet,
    },
};

fn main() {
    let mut buf: String = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut info = buf
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n: usize = info.next().unwrap();
    let mut costs_info: Vec<(usize, usize, usize)> = Vec::with_capacity(n);
    for i in 0..n {
        for j in 0..n {
            let now_cost = info.next().unwrap();
            if i < j { costs_info.push((i, j, now_cost)) }
        }
    }
    costs_info.sort_by_key(|x| x.2);
    let mut is_connected: HashSet<usize> = HashSet::from([0]);
    let mut answer: usize = 0;
    while is_connected.len() < n {
        for (x, y, c) in &costs_info {
            if (!is_connected.contains(x) && is_connected.contains(y)) ||
                (is_connected.contains(x) && !is_connected.contains(y)) {
                is_connected.insert(*x);
                is_connected.insert(*y);
                answer += *c;
                break
            }
        }
    }
    print!("{answer}");
}