use std::{
    io::{self},
    io::Read,
    collections::{HashSet, HashMap, BinaryHeap, VecDeque},
    cmp::Reverse
};


fn main(){
    let mut buffer = String::new();
    let mut stdin = io::stdin().lock();
    stdin.read_to_string(&mut buffer).unwrap();
    let mut lst = buffer
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap());
    let node_cnt = lst.next().unwrap().try_into().unwrap();
    let line_cnt: usize = lst.next().unwrap().try_into().unwrap();
    let lst: Vec<i64> = lst.collect();
    let mut l: HashMap<i64, Vec<[i64; 2]>> = HashMap::with_capacity(line_cnt);
    let mut visited: Vec<bool> = vec![false; node_cnt];
    for i in (0..lst.len()).step_by(3) {
        l.entry(lst[i]-1)
            .or_insert_with(Vec::new)
            .push([lst[i+2], lst[i+1]-1]);
        l.entry(lst[i+1]-1)
            .or_insert_with(Vec::new)
            .push([lst[i+2], lst[i]-1]);
    }
    let mut answer: i64 = 0;
    let mut heap: BinaryHeap<Reverse<[i64; 2]>> = BinaryHeap::new();
    heap.push(Reverse([0, 0]));
    while !heap.is_empty() {
        match heap.pop() {
            Some(Reverse(x)) => {
                let next_node: usize = x[1].try_into().unwrap();
                if !visited[next_node] {
                    visited[next_node] = true;
                    answer += x[0];
                    match l.get(&x[1]) {
                        Some(k) => {
                            for n in k {
                                heap.push(Reverse(*n));
                            }
                        },
                        None => ()
                    }

                }
            },
            None => ()
        }
    }
    print!("{}", answer);
}

