use std::io;
use std::fmt::Write;

fn main() {
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let number: i32 = buffer.trim().parse::<i32>().unwrap();
    let mut answer = String::new();
    for i in 1..=number {
        writeln!(answer, "{}", i).unwrap();
    }
    print!("{}", answer)
}
