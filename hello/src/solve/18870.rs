use std::{
    io,
    io::Read,
    fmt::Write,
    collections::{BTreeSet, HashMap}
};

fn main() {
    let mut buf: String = String::new();
    let mut stdin = io::stdin().lock();
    stdin.read_to_string(&mut buf).unwrap();

    let mut lst = buf
        .split_ascii_whitespace()
        .map(|x| x.parse::<i32>().unwrap());
    lst.next().unwrap();
    
   let hash: BTreeSet<i32> = lst.clone().collect();
   let mut answer: HashMap<i32, usize> = HashMap::new();
    for (a, b) in hash.into_iter().enumerate() {
        answer.insert(b, a);
    }

    let mut output: String = String::new();
    for i in lst {
        write!(output, "{} ", answer.get(&i).unwrap()).unwrap();
    }
    println!("{output}");
}

