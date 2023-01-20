use std:: {
    io::{self, Read},
};

fn main() {
    let mut buffer: String = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut info = buffer.split_ascii_whitespace();
    info.next();
    let mut answer = 0;
    for s in info {
        let mut stack: Vec<char> = Vec::with_capacity(s.len());
        for c in s.chars() {
            if !stack.is_empty() && *stack.last().unwrap() == c {
                stack.pop();
            } else {
                stack.push(c);
            }
        }
        if stack.is_empty() {
            answer += 1;
        }
    }
    print!("{answer}");
}