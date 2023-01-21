use std:: {
    io::{self, Read},
};

fn main() {
    let mut buffer: String = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut info: String= String::from(buffer.trim());
    info = format!("({info})");
    info = info.replace("()", "2");
    info = info.replace("[]", "3");
    let mut stack: Vec<String> = Vec::new();
    let mut flag: bool = true;
    'outer:for c in info.chars() {
        let mut acc = 0;
        if c == ')' {
            while !stack.is_empty() {
                let last_of = stack.pop().unwrap();
                if last_of == "(" {
                    stack.push((acc*2).to_string());
                    break;
                }
                match last_of.parse::<i32>() {
                    Ok(x) => acc += x,
                    Err(_) => {
                        flag = false;
                        break 'outer;
                    },
                }
            }
        } else if c == ']' {
            while !stack.is_empty() {
                let last_of = stack.pop().unwrap();
                if last_of == "[" {
                    stack.push((acc*3).to_string());
                    break;
                }
                match last_of.parse::<i32>() {
                    Ok(x) => acc += x,
                    Err(_) => {
                        flag = false;
                        break 'outer;
                    },
                }
            }
        } else {
            stack.push(String::from(c));
        }
    }
    let mut answer = 0;
    if flag {
        for i in stack {
            match i.parse::<i32>() {
                Ok(x) => answer += x,
                Err(_) => {
                    print!("0");
                    flag = false;
                    break;
                },
            }
        }
        if flag {
            print!("{}",answer/2);
        }
    } else {
        print!("0");
    }
}