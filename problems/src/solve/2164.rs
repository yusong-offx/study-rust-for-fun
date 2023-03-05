use std::{
    io::{self, Read},
    collections::VecDeque,
};
 
 fn main() {
    let mut buffer: String = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut tmp_iter = buffer
        .split_ascii_whitespace()
        .map(|x| x.parse::<isize>().unwrap());
    let n: isize = tmp_iter.next().unwrap();
    let mut dq: VecDeque<isize> = (1..=n).collect();
    while dq.len() > 1 {
        dq.pop_front();
        dq.rotate_left(1);
    }
    println!("{:?}", dq[0]);
}
