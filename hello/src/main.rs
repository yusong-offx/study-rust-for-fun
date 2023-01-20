use std:: {
    io::{self, Read},
    collections::{VecDeque}
};

fn main() {
    let mut buffer: String = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let trim_str:&[_] = &['[', ']'];
    let mut info_iter = buffer.split_ascii_whitespace();
    let x: usize = info_iter.next().unwrap().parse().unwrap();
    'outer:for _ in 0..x {
        let cmd = info_iter.next().unwrap().chars();
        info_iter.next();
        let mut head: bool = true;
        let mut container: VecDeque<_> = info_iter.next().unwrap()
            .trim_matches(trim_str)
            .split(',')
            .collect();
        if container[0] == "" {
            container.pop_front();
        }
        for c in cmd {
            if c == 'R' {
                head = !head;
            } else {
                if container.is_empty() {
                    println!("error");
                    continue 'outer; 
                }
                match head {
                    true => container.pop_front().unwrap(),
                    false => container.pop_back().unwrap(),
                };
            }
        }
        if !head {
            container.make_contiguous().reverse();
        }
        let answer: Vec<_> = container.into();
        println!("[{}]", answer.join(","));
    }

}