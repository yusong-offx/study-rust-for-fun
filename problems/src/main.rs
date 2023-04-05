struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut carry = true;
        for i in (0..digits.len()).rev() {
            if carry {
                carry = false;
                digits[i] += 1;
                if digits[i] == 10 {
                    digits[i] = 0;
                    carry = true;
                }
            } else {
                break
            }
        }
        if carry {
            digits.insert(0, 1);
        }
        digits
    }
}

fn main() {
    println!("{:?}", Solution::plus_one(vec![9,9,9,9]));
}