use std:: {
    io::{self, Read},
    collections::{BTreeMap, HashSet},
    cmp::max,
};

fn main() {
    let mut buf: String = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let info: Vec<&str>= buf
        .split_ascii_whitespace()
        .collect();
    let n: usize = info[0].parse().unwrap();
    let mut graph: Vec<Vec<&str>> = Vec::with_capacity(n);
    for start in (1..info.len()).step_by(n) {
        graph.push(info[start..start+n].to_vec());
    }
    let mut up_site: BTreeMap<isize, Vec<(usize, usize, isize)>>= BTreeMap::new();
    // let mut down_site: HashMap<isize, Vec<(usize, usize, isize)>>= HashMap::with_capacity(n);
    for i in 0..graph.len() {
        for j in 0..graph[0].len() {
            if graph[i][j] == "1" {
                let w: isize = i as isize;
                let v: isize = j as isize;
                up_site.entry(w+v).or_insert(Vec::new()).push((i, j, w-v));
            }
        }
    }
    // let tmp: Vec<Vec<(usize, usize, isize)>> = up_site.values().cloned().collect();
    let mut one = Vec::new();
    let mut two = Vec::new();
    for i in up_site.keys() {
        if i.abs() % 2 == 0{
            one.push(up_site.get(i).unwrap().clone());
        } else {
            two.push(up_site.get(i).unwrap().clone());
        }
    }
    let solution = Bishop {
        up_site: one,
    };
    let mut answer: usize = 0;
    solution.solve(0, &mut HashSet::new(), &mut answer, &mut Vec::new());


    let solution2 = Bishop {
        up_site: two,
    };
    let mut answer2: usize = 0;
    solution2.solve(0, &mut HashSet::new(), &mut answer2, &mut Vec::new());

    print!("{}", answer + answer2);

}

struct Bishop {
    up_site: Vec<Vec<(usize, usize, isize)>>,
}

impl Bishop {
    fn solve(&self, depth: usize, grab: &mut HashSet<isize>, answer: &mut usize, trace: &mut Vec<(usize, usize)>) {
        if depth == self.up_site.len() {
            *answer = max(*answer, grab.len());
            return ;
        }
        for (x, y, loc) in &self.up_site[depth] {
            if grab.contains(&loc) {
                continue ;
            }
            grab.insert(*loc);
            trace.push((*x, *y));
            self.solve(depth+1, grab, answer, trace);
            trace.pop();
            grab.remove(loc);
        }
        self.solve(depth+1, grab, answer, trace);
    }
}