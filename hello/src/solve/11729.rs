use std:: {
    io::{self, Read},
    fmt::Write,
};

fn main() {
    let mut buf: String = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut info = buf
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n: usize = info.next().unwrap();
    let mut answer: String = String::new();
    hanoi(n, 1, 3, 2,&mut answer);
    print!("{}\n{}", answer.len() / 4, answer);
}

fn hanoi(depth: usize, from: usize, to: usize, sub: usize, answer: &mut String) {
    if depth == 1 {
        writeln!(answer, "{} {}", from, to).unwrap();
        return ;
    }
    hanoi(depth-1, from, sub, to, answer);
    writeln!(answer, "{} {}", from, to).unwrap();
    hanoi(depth-1, sub, to, from, answer);
}