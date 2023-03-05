use std:: {
    io::{
        self, Read
    },
    collections::{
        HashSet,
    },
};

fn main() {
    let mut buffer: String = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut info = buffer
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let mut get_next: Box<dyn FnMut() -> usize> = Box::new(|| {info.next().unwrap()});

    // 기본정보
    let (locs, lines, power_plants) = (get_next(), get_next(), get_next());
    
    let mut is_connected: HashSet<usize> = HashSet::new();
    let mut costs: Vec<(usize, usize, usize)> = Vec::with_capacity(lines);

    // 발전소는 연결된 장소로 취급
    for _ in 0..power_plants {
        is_connected.insert(get_next());
    }
    
    // 연결 정보를 비용에 따라 정렬
    for _ in 0..lines {
        costs.push((get_next(), get_next(), get_next()));
    }
    costs.sort_by(|a, b| a.2.cmp(&b.2));

    // 비용이 작은 것부터 연결
    let mut answer: usize = 0;

    while is_connected.len() < locs {
        for i in 0..costs.len() {
            let (from, to, cost) = costs[i];
            if (is_connected.contains(&from) && !is_connected.contains(&to))
                || (!is_connected.contains(&from) && is_connected.contains(&to)){
                    is_connected.insert(from);
                    is_connected.insert(to);
                    answer += cost;
                    break
            }
        }
    }
    println!("{answer}");
}