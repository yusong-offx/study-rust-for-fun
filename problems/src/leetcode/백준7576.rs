use std::{
    io::{stdin, Read}
};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut info = buf
        .split_ascii_whitespace()
        .map(|x| x.parse::<i32>().unwrap());
    let (gx, gy) = (info.next().unwrap() as usize, info.next().unwrap() as usize);
    let mut totato_field = vec![vec![0; gx]; gy];
    let mut stack = Vec::new();
    for i in 0..gy {
        for j in 0..gx {
            let status = info.next().unwrap();
            totato_field[i][j] = status;
            if status == 1 {
                stack.push((i, j));
            }
        }
    }
    let delta = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut days = -1;
    while !stack.is_empty() {
        let mut tmp = Vec::new();
        while !stack.is_empty() {
            let (x, y) = stack.pop().unwrap();
            for d in delta {
                let nx = x as i32 + d.0;
                let ny = y as i32 + d.1;
                if 0 <= nx && nx < gy as i32 && 0 <= ny && ny < gx as i32 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if totato_field[nx][ny] == 0 {
                        totato_field[nx][ny] = 1;
                        tmp.push((nx, ny))
                    }
                }
            }
        }

        stack = tmp;
        days += 1;
    }
    for l in &totato_field {
        println!("{:?}", l);
    }
'outer:
    for i in 0..gy {
        for j in 0..gx {
            if totato_field[i][j] == 0 {
                days = -1;
                break 'outer;
            }
        }
    }
    println!("{days}");
}