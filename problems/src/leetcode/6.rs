use std::str::Chars;

struct Solution;

impl Solution {
    fn insert_one_line(rows: &mut Vec<String>, sample: &mut Chars) -> bool {
        for i in 0..rows.len() {
            match sample.next() {
                Some(c) => {
                    rows[i].push(c);
                },
                None => return true,
            }
        }
        false
    }

    fn insert_cross_line(rows: &mut Vec<String>, sample: &mut Chars) -> bool {
        for i in (1..rows.len()-1).rev() {
            match sample.next() {
                Some(c) => {
                    rows[i].push(c);
                },
                None => return true,
            }
        }
        false
    }

    pub fn convert(s: String, num_rows: i32) -> String {
        let mut rows = vec![String::new(); num_rows as usize];
        let mut s = s.chars();
        loop {
            if Self::insert_one_line(&mut rows, &mut s) {
                break;
            }
            if Self::insert_cross_line(&mut rows, &mut s) {
                break;
            }
        }
        rows.join("")
    }
}


fn main() {
    let test = vec![String::from("Hello"), String::from("World")];
    println!("{}", test.join(""));
}
