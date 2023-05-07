struct Solution;

impl Solution {
    fn recur(s: &String, right: usize, buf: &mut Vec<usize>) -> usize {
        if right >= s.len() {
            println!("{:?}", buf);
            return 1
        }
        let mut cnt = 0;
        for i in (right)..s.len() {
            let frag = s[right..=i].parse::<usize>().unwrap();
            if 1 <= frag && frag <= 27 {
                buf.push(frag);
                cnt += Self::recur(s, i+1, buf);
                buf.pop();
            } else {
                break
            }
        }
        cnt
    }

    pub fn num_decodings(s: String) -> i32 {
        Self::recur(&s, 0, &mut Vec::new()) as i32
    }
}

fn main() {
    // assert_eq!(Solution::num_decodings(String::from("12")), 2);
    // assert_eq!(Solution::num_decodings(String::from("06")), 0);
    // assert_eq!(Solution::num_decodings(String::from("226")), 3);
    assert_eq!(Solution::num_decodings(String::from("27")), 2);
    // let a = String::from("123");
    // let b = String::from("124");
    // let c = &a[1..=1];
    // println!("{:?}", c);
}