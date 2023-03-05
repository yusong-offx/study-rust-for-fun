use std::{
    io::{
        self, Read
    },
    collections::{
        HashMap, HashSet, BinaryHeap
    }, 
    cmp::Reverse,
    fmt::Write
};

fn main() {
    let mut buf: String = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut info = buf
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let (problems, length) = (info.next().unwrap(), info.next().unwrap());
    let mut before_work: Vec<usize> = vec![0; problems+1];
    let mut after_work: HashMap<usize, HashSet<usize>> = HashMap::new();
    for _ in 0..length {
        let (first, later): (usize, usize) = (info.next().unwrap(), info.next().unwrap());
        after_work
            .entry(first)
            .and_modify(|x| {x.insert(later);})
            .or_insert(HashSet::from([later]));
        before_work[later] += 1;
    }
    let mut stack: BinaryHeap<Reverse<usize>> = BinaryHeap::new();

    for i in 1..=problems {
        if before_work[i] == 0 {
            stack.push(Reverse(i));
        }
    }

    let mut answer: Vec<usize> = Vec::with_capacity(problems);
    while !stack.is_empty() {
        let Reverse(now) = stack.pop().unwrap();
        answer.push(now);
        for &work in after_work.get(&now).unwrap_or(&HashSet::new()) {
            before_work[work] -= 1;
            if before_work[work] == 0 {
                stack.push(Reverse(work));
            }
        }
    }
    buf.clear();
    for i in answer {
        write!(&mut buf, "{} ", i).unwrap();
    }
    print!("{buf}");
}