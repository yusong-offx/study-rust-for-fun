impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
          let mut answer = Vec::with_capacity(5);
          answer.push(vec![1]);
          for i in 1..num_rows as usize {
              let mut row = vec![1; i+1];
              for j in 1..i {
                  row[j] = answer[i-1][j] + answer[i-1][j-1];
              }
              answer.push(row);
          }
        answer
    }
  }