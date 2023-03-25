use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // 가로, 세로
        for i in 0..9 {
            let mut vertical  = HashSet::new();
            let mut horizontal = HashSet::new();
            for j in 0..9 {
                if board[i][j] != '.' && horizontal.contains(&board[i][j]) {
                    return false
                }
                if board[j][i] != '.' && vertical.contains(&board[j][i]) {
                    return false
                }
                horizontal.insert(board[i][j]);
                vertical.insert(board[j][i]);
            }
        }

        // 그리드
        for x in (0..9).step_by(3) {
            for y in (0..9).step_by(3) {
                let mut grid = HashSet::new();
                for i in x..x+3 {
                    for j in y..y+3 {
                        if board[i][j] != '.' && grid.contains(&board[i][j]) {
                            return false
                        }
                        grid.insert(board[i][j]);
                    }
                }
            }
        }


        true
    }
}