struct Solution;

impl Solution {

    fn dfs(
        word: &Vec<char>,
        board: &Vec<Vec<char>>,
        stack: &mut Vec<(usize, usize, usize)>, 
        visited: &mut Vec<Vec<bool>>, 
        move_delta: &[(i8, i8); 4]
    ) -> bool {
        if stack.is_empty() {
            return false
        }

        let (x, y, z) = stack.pop().unwrap();

        if visited[x][y] {
            return false
        }

        if word.len() - 1 == z {
            return true
        }

        visited[x][y] = true;

        for &delta in move_delta {
            let nx = x  as i8 + delta.0;
            let ny = y  as i8 + delta.1;
            if 0 <= nx && nx < board.len() as i8 &&
                0 <= ny && ny < board[0].len() as i8 {
                let nx = nx as usize;
                let ny = ny as usize;
                if board[nx][ny] == word[z+1] {
                    stack.push((nx, ny, z+1));
                    if Self::dfs(word, board, stack, visited, move_delta) {
                        return true
                    }
                }
            }
        }
        visited[x][y] = false;
        false
    }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let move_delta = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut start = Vec::new();
        let word: Vec<char> = word.chars().into_iter().collect();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == word[0] {
                    start.push((i, j));
                }
            }
        }
        let mut visited = vec![vec![false; board[0].len()]; board.len()];
        let mut stack = Vec::new();
        for now in start {
            stack.push((now.0, now.1, 0));
            if Self::dfs(&word, &board, &mut stack, &mut visited, &move_delta) {
                return true
            }
        }
        false
    }
}

fn main() {
    assert_eq!(Solution::exist(
        vec![
            vec!['A','B','C','E'],
            vec!['S','F','C','S'],
            vec!['A','D','E','E'], 
            vec!['A','B','C','E'],
            vec!['S','F','C','S'],
            vec!['A','D','E','E']
            ],
            String::from("ABCCED")
        )
        , true
    );
    assert_eq!(Solution::exist(
            vec![
                vec!['A','B','C','E'],
                vec!['S','F','E','S'],
                vec!['A','D','E','E'],
            ]
            ,
            String::from("ABCESEEEFS")
        )
        , true
    );
}
