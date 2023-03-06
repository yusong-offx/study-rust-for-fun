use std::{
    io::{
        self, Read
    },
    collections::{
      BinaryHeap, HashMap
    },
    usize::MAX, 
    cmp::Reverse, 
    fmt::Write
};

fn main() {
    let mut buf: String = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut info = buf
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let (locs, lines): (usize, usize) = (info.next().unwrap(), info.next().unwrap());
    let mut graph: HashMap<usize, Vec<(usize, usize)>> = HashMap::with_capacity(locs+1);
    for _ in 0..lines {
        let (a, b, c): (usize, usize, usize) = 
            (info.next().unwrap(), info.next().unwrap(), info.next().unwrap());
        graph
            .entry(a)
            .and_modify(|v| v.push((b, c)))
            .or_insert(vec![(b, c)]);
        graph
            .entry(b)
            .and_modify(|v| v.push((a, c)))
            .or_insert(vec![(a, c)]);
    }

    let dijkstra = |start: usize| -> Vec<usize>{
        let mut costs: Vec<usize> = vec![MAX; locs+1];
        let mut heap: BinaryHeap<(Reverse<usize>, usize, usize)> = BinaryHeap::from([(Reverse(0), start, 0)]);
        costs[start] = 0;

        let mut ret: Vec<usize> = vec![0; locs+1];
        let mut first_arrive: bool = true;

        while !heap.is_empty() {
            let (Reverse(now_cost), now_loc, first): (Reverse<usize>, usize, usize) = heap.pop().unwrap();
            let mut first: usize = first;
            if costs[now_loc] < now_cost {
                continue;
            }
            for &(next_loc, next_cost) in graph.get(&now_loc).unwrap_or(&Vec::new()) {
                let new_cost: usize = next_cost + now_cost;
                if new_cost < costs[next_loc] {
                    costs[next_loc] = new_cost;
                    if first_arrive { first = next_loc }
                    ret[next_loc] = first;
                    heap.push((Reverse(new_cost), next_loc, first));
                }
            }
            if first_arrive { first_arrive = !first_arrive }
        }

        ret
    };

    buf.clear();
    for s in 1..=locs {
        let l = &dijkstra(s)[1..];
        for &el in l {
            if el == 0 {
                write!(buf, "- ").unwrap();
            } else {
                write!(buf, "{el} ").unwrap();
            }
        }
        write!(buf, "\n").unwrap();
    }
    print!("{buf}");
}