impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut answer = Vec::with_capacity(n as usize);
        for i in 1..=n {
            let remainder = (i % 3, i % 5);
            if remainder.1 == 0 && remainder.0 == 0 {
                answer.push(String::from("FizzBuzz"));
            } else if remainder.0 == 0 {
                answer.push(String::from("Fizz"));
            } else if remainder.1 == 0 {
                answer.push(String::from("Buzz"));
            } else {
                answer.push(i.to_string());
            }
        }
        answer
    }
}