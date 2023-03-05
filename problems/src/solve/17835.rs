use std::{
    cmp::Reverse,
    io::{stdin, Read},
    collections::{HashMap, BinaryHeap, HashSet}
};

#[derive(Debug)]
struct INFO {
    cities: isize,
    interview_location: HashSet<isize>,
    m: HashMap<isize, Vec<(isize, isize)>>,
    answer_location: isize,
    answer_cost: isize,
}

fn main() {
    let mut buffer: String = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut info = buffer
        .split_ascii_whitespace()
        .map(|x| x.parse::<isize>().unwrap());
    let cities = info.next().unwrap();
    let lines = info.next().unwrap();
    info.next();

    let mut m: HashMap<isize, Vec<(isize, isize)>> = HashMap::with_capacity(lines as usize);
    for _ in 0..lines {
        let a = info.next().unwrap();
        let b = info.next().unwrap();
        let c = info.next().unwrap();
        if !m.contains_key(&b) {
            m.insert(b, Vec::new());
        }
        m.get_mut(&b).unwrap().push((c, a));
    }
    let interview_locations: HashSet<isize> = HashSet::from_iter(info);
    let mut var: Box<INFO> = Box::new(INFO{
        cities: cities,
        interview_location: interview_locations,
        m: m,
        answer_location: 0,
        answer_cost: 0,
    });
    solution(&mut var);
    print!("{}\n{}", var.answer_location, var.answer_cost);
}

fn solution(info :&mut INFO) {
    let cost = dijkstra(info);
    for i in 1..=info.cities {
        if info.interview_location.contains(&i) {
            continue
        }
        if cost[i as usize] > info.answer_cost {
            info.answer_cost = cost[i as usize];
            info.answer_location = i;
        }
    }
}

fn dijkstra(info :&INFO) -> Vec<isize> {
    let mut heap: BinaryHeap<Reverse<(isize, isize)>> = BinaryHeap::new();
    let mut visited: Vec<bool> = vec![false; info.cities as usize + 1];
    let mut cost: Vec<isize> = vec![std::isize::MAX; info.cities as usize + 1];
    for i in info.interview_location.iter() {
        heap.push(Reverse((0, *i)));
    }
    while !heap.is_empty() {
        let Reverse((now_cost, now_location)) = heap.pop().unwrap();
        if cost[now_location as usize] < now_cost {
            continue;
        }
        visited[now_location as usize] = true;
        cost[now_location as usize] = now_cost;
        if !info.m.contains_key(&now_location) {
            continue;
        }
        for (next_cost, next_location) in info.m.get(&now_location).unwrap() {
            let new_cost = *next_cost + now_cost;
            if new_cost < cost[*next_location as usize] && !visited[*next_location as usize] {
                cost[*next_location as usize] = new_cost;
                heap.push(Reverse((new_cost, *next_location)));
            }
        }
    }
    cost
}