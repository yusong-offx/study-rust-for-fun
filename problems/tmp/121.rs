impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut answer = 0;
        let mut base = prices[0];
        for i in 1..prices.len() {
            if base < prices[i] {
                answer = answer.max(prices[i]-base);
            } else {
                base = prices[i];
            }
        }
        answer
    }
}