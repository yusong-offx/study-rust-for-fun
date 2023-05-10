impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut b: Vec<char> = format!("{:032b}", x).chars().collect();
        let l = b.len();
        for i in 0..l/2 {
            let tmp = b[l-1-i];
            b[l-1-i] = b[i];
            b[i] = tmp;
        }
        let mut new_b = String::with_capacity(l);
        for c in b {
            new_b.push(c);
        }
        u32::from_str_radix(&new_b, 2).unwrap() 
    }
}

// fn main() {
//     let a: u32 = 24;
//     let b = format!("{:032b}", a);
//     println!("{b}");
// }