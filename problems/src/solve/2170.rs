use std::{
    io::{
        self, Read
    },
    cmp::{
        max, min
    }
};

fn main() {
    let mut buf: String = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let mut info = buf
        .split_ascii_whitespace()
        .map(|x| x.parse::<isize>().unwrap());

    let n: usize = info.next().unwrap() as usize;
    let mut lines: Vec<(isize, isize)> = Vec::with_capacity(n);
    for _ in 0..n {
        lines.push((info.next().unwrap(), info.next().unwrap()));
    }

    let mut combine_lines: Vec<(isize, isize)> = Vec::new();
    while !lines.is_empty() {
        let now: (isize, isize) = lines.pop().unwrap();
        if 'find: {
            for i in 0..combine_lines.len() {
                let (start, end): (isize, isize) = combine_lines[i];
                if start <= now.0 && now.0 <= end && 
                    start <= now.1 && now.1 <= end {
                    break 'find false
                } else if start <= now.0 && now.0 <= end ||
                    start <= now.1 && now.1 <= end {
                    let r: (isize, isize) = combine_lines.remove(i);
                    let new_line: (isize, isize) = (min(r.0, now.0), max(r.1, now.1));
                    lines.push(new_line);
                    break 'find false
                }
            }
            break 'find true
        } {
            combine_lines.push(now);
        }
    }

    let mut answer: isize = 0;
    for (start, end) in combine_lines {
        answer += end - start;
    }
    print!("{answer}");
}