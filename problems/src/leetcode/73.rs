struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (mut col, mut row) = (HashSet::new(), HashSet::new());
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 0 {
                    col.insert(i);
                    row.insert(j);
                }
            }
        }
        for i in col {
            for j in 0..matrix[0].len() {
                matrix[i][j] = 0;
            }
        }
        for i in row {
            for j in 0..matrix.len() {
                matrix[j][i] = 0;
            }
        }
    }
}