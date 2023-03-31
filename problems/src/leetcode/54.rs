
struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut up, mut down, mut left, mut right) = (0, matrix.len()-1, 0, matrix[0].len()-1);
        let mut answer = Vec::with_capacity(matrix.len() * matrix[0].len());
        let mut direction= [0, 1, 2, 3].iter().cycle();
        while up <= down && left <= right {
            match *(direction.next().unwrap()) {
                0 => {
                    for i in left..=right {
                        answer.push(matrix[up][i]);
                    }
                    if up == matrix.len() {
                        break
                    }
                    up += 1;
                }
                1 => {
                    for i in up..=down {
                        answer.push(matrix[i][right]);
                    }
                    if right == 0 {
                        break
                    }
                    right -= 1;
                }
                2 => {
                    for i in (left..=right).rev() {
                        answer.push(matrix[down][i]);
                    }
                    if down == 0 {
                        break
                    }
                    down -= 1;
                }
                3 => {
                    for i in (up..=down).rev() {
                        answer.push(matrix[i][left]);
                    }
                    if left == matrix[0].len() {
                        break
                    }
                    left += 1;
                }
                _ => {}
            }
        }
        answer
    }
}

fn main() {
    assert_eq!(Solution::spiral_order(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]]), [1,2,3,6,9,8,7,4,5]);
    assert_eq!(Solution::spiral_order(vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12]]), vec![1,2,3,4,8,12,11,10,9,5,6,7]);
    assert_eq!(Solution::spiral_order(vec![vec![3],vec![2]]), vec![3, 2]);

}