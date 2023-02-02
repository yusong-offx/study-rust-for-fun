use std::{
    io::{self, Read},
    cmp::max,
    collections::VecDeque,
};

fn main() {
    let mut buf: String = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut info = buf
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let list_length = info.next().unwrap();
    let can_remove_count = info.next().unwrap();
    let mut use_remove_count = can_remove_count;
    let list: Vec<usize> = info.collect();
    let mut tmp_list: VecDeque<usize> = VecDeque::new();
    let mut now: usize = 0;
    let mut answer: usize = 0;

    while now < list_length {
        if is_even(list[now]) {
                tmp_list.push_back(list[now]);
                answer = max(answer, tmp_list.len() - (can_remove_count - use_remove_count));
                now += 1;
        } else {
            if use_remove_count > 0 {
                tmp_list.push_back(list[now]);
                use_remove_count -= 1;
                now += 1;
            } else {
                while !tmp_list.is_empty() {
                    if !is_even(tmp_list.pop_front().unwrap()) {
                        use_remove_count += 1;
                        break ;
                    }
                }
            }
        }
    }
    print!("{answer}");
}


fn is_even(n: usize) -> bool {
    match n % 2{
        0 => true,
        _ => false,
    }
}