use std::{
    io::{self, Read},
    collections::HashMap, cmp::max,
};

fn main() {
    let mut buf: String = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut info = buf
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let length: usize = info.next().unwrap();
    let limit_of_repeat: usize = info.next().unwrap();
    let mut counter: HashMap<usize, usize> = HashMap::new();
    let mut start: usize = 0;
    let mut answer: usize = 0;
    let info: Vec<usize> = info.collect();
    for (end, now) in info.iter().enumerate() {
        let now_count = counter.entry(*now).or_insert(0);
        *now_count += 1;
        if *now_count <= limit_of_repeat {
            answer = max(answer, end - start + 1);
        } else {
            while *counter.get(&now).unwrap() > limit_of_repeat {
                *counter.get_mut(&info[start]).unwrap() -= 1;
                start += 1;
            }
        }

        if length - start <= answer {
            break ;
        }
    }
s
    println!("{answer}");
}