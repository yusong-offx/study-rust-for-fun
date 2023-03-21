use std::collections::HashMap;

struct Solution;

impl Solution {
    fn recur(dep:usize, v: &Vec<&Vec<char>>, buf:&mut Vec<char>, answer: &mut Vec<String>) {
        if dep == v.len() {
            let mut s = String::with_capacity(buf.len());
            for c in buf {
                s.push(*c);
            }
            answer.push(s);
            return
        }
        for c in v[dep] {
            buf.push(*c);
            Self::recur(dep+1, v, buf, answer);
            buf.pop();
        }
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut answer = Vec::new();
        if digits == String::from("") {
            return answer
        }
        let number_to_letters = HashMap::from([
            ('2', vec!['a', 'b', 'c']), ('3', vec!['d', 'e', 'f']), ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']), ('6', vec!['m', 'n', 'o']), ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']), ('9', vec!['w', 'x', 'y', 'z'])
        ]);

        let mut material = Vec::new();
        for c in digits.chars() {
            material.push(&number_to_letters[&c]);
        }

        Self::recur(0, &material, &mut Vec::new(), &mut answer);
        answer
    }
}

fn main() {
    assert_eq!(Solution::letter_combinations("23".to_string()), ["ad","ae","af","bd","be","bf","cd","ce","cf"]);
    assert_eq!(Solution::letter_combinations("".to_string()), Vec::<String>::new());
}