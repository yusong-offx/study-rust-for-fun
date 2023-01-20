use std::{
    io::{self, Read},
    collections::{VecDeque}
};
fn main() {
    let mut buffer: String = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut info_iter = buffer
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let origin_count = info_iter.next().unwrap();
    let mut origin_list: VecDeque<usize> = (1..=origin_count).collect();
    info_iter.next().unwrap();
    let extract_list: Vec<usize> = info_iter.collect();
    let mut work_count: usize = 0;
    for now in extract_list {
        if *origin_list.front().unwrap() != now {
            let mid: usize = (origin_list.len() / 2) + 1;
            let mut index_of: usize = 0;
            for i in 0..origin_list.len() {
                if now == origin_list[i] {
                    index_of = i;
                    break;
                }
            }
            if index_of < mid {
                work_count += index_of;
                origin_list.rotate_left(index_of);
            } else {
                let tmp = origin_list.len() - index_of;
                work_count += tmp;
                origin_list.rotate_right(tmp);
            }
        }
        origin_list.pop_front();
    }
    print!("{work_count}");
}