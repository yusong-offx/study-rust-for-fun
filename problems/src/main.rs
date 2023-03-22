struct Solution;

impl Solution {
    fn recur(open_bracket: i32, close_bracket: i32, buf: &mut Vec<char>, answer: &mut Vec<String>) {
        if open_bracket == 0 && close_bracket == 0 {
            answer.push(buf.iter().collect::<String>());
            return
        }
        if 0 < open_bracket {
            buf.push('(');
            Self::recur(open_bracket-1, close_bracket, buf, answer);
            buf.pop();
        }
        if open_bracket < close_bracket {
            buf.push(')');
            Self::recur(open_bracket, close_bracket-1, buf, answer);
            buf.pop();
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut answer = Vec::new();
        Self::recur(n, n, &mut Vec::new() , &mut answer);
        answer
    }
}