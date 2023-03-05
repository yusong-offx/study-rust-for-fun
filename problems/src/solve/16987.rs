use std::{
    io::{self, Read},
    collections::{HashSet}, cmp::max,
};
 
 fn main() {
    let mut buffer: String = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let info: Vec<isize> = buffer
        .split_ascii_whitespace()
        .map(|x| x.parse::<isize>().unwrap())
        .collect();
    let length: usize = info[0] as usize;
    let mut eggs: Vec<[isize; 2]> = Vec::with_capacity(length);
    for n in (1..info.len()).step_by(2) {
        eggs.push([info[n], info[n+1]]);
    }
    let mut answer: usize = 0;
    egg_fight(0, &mut eggs, &mut HashSet::new(), &mut answer);
    print!("{answer}");
 }

 fn egg_fight(now: usize, eggs: &mut Vec<[isize; 2]>, crash_eggs: &mut HashSet<usize>, answer: &mut usize) {
    // Only now in range
    if now < eggs.len() {
        // Now egg broken or not
        if eggs[now][0] <= 0 {
            egg_fight(now+1, eggs, crash_eggs, answer);
        } else {
            let mut crash: HashSet<usize> = HashSet::with_capacity(2);
    
            for opp_egg in 0..eggs.len() {
                // Pass same egg
                if now == opp_egg || crash_eggs.contains(&opp_egg) { continue }
                eggs[opp_egg][0] -= eggs[now][1];
                eggs[now][0] -= eggs[opp_egg][1];
                if eggs[opp_egg][0] <= 0 { crash.insert(opp_egg); }
                if eggs[now][0] <= 0 { crash.insert(now); }
                crash_eggs.extend(crash.iter());
                egg_fight(now+1, eggs, crash_eggs, answer);
                for v in crash.drain() { crash_eggs.remove(&v); }
                eggs[opp_egg][0] += eggs[now][1];
                eggs[now][0] += eggs[opp_egg][1];
            }
        }
    }

    // Check max crash eggs
    *answer = max(crash_eggs.len(), *answer);
 }

