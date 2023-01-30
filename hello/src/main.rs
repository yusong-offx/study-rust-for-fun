use std::{
    io::{self, Read},
    collections::{BTreeSet},
    fmt::Write,
};

fn main() {
    let mut buf: String = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut info = buf
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    info.next().unwrap();
    let pick: usize = info.next().unwrap();
    let remove_repeat: BTreeSet<usize> = BTreeSet::from_iter(info);
    let mut sol = Solution{
        now: Vec::new(),
        number_box: remove_repeat.into_iter().collect(),
        pick: pick,
    };
    let mut answer: String = String::new();
    sol.repeat_permutations(0, 0, &mut answer);
    print!("{answer}");
    
}


struct Solution {
    now: Vec<usize>,
    number_box: Vec<usize>,
    pick: usize,
}

impl Solution {
    fn repeat_permutations(&mut self, start: usize, depth: usize, answer: &mut String) {
        if depth == self.pick {
            let mut output: String = String::with_capacity(self.pick * 2);
            for c in &self.now {
                output.push_str(&c.to_string());
                output.push(' ');
            }
            output.pop();
            writeln!(answer, "{}", output).unwrap();
            return ;
        }
        for i in start..self.number_box.len() {
            self.now.push(self.number_box[i]);
            self.repeat_permutations(i, depth+1, answer);
            self.now.pop();
        }
    }
}