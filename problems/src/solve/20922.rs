use std::{
    io::{
        self, Read
    }, 
    collections::HashMap,
    cmp::max,
};

fn main() {
    // 파싱
    let mut buf: String = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut info = buf
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().expect("not numberic"));

    // 정보 분해
    let (_, limit): (usize, usize) = (info.next().unwrap(), info.next().unwrap());
    let elements: Vec<usize> = info.collect();

    // 원소를 순회하며 지정한 길이내의 중복되는 원소의 갯수를 기록
    // 가장 길었을때를 정답으로 기록
    let mut counter: HashMap<usize, usize> = HashMap::new();
    let mut answer: usize = 0;
    let mut left: usize = 0;
    for (right, now) in elements.iter().enumerate() {
        // 중복된 원소 갯수 세기
        counter.entry(*now)
            .and_modify(|v| *v +=1)
            .or_insert(1);
        // 제한된 중복횟수를 넘어가면 가장 먼저 나왔던 중복된 원소를 제거
        while *counter.get(now).unwrap() > limit {
            let repeat = elements[left];
            *counter.get_mut(&repeat).unwrap() -= 1;
            left += 1;
        }
        // 정답 기록
        answer = max(answer, right - left + 1);
    }
    println!("{answer}");
}