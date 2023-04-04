struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (x, y) = (obstacle_grid.len(), obstacle_grid[0].len());
        if obstacle_grid[x-1][y-1] == 1 || obstacle_grid[0][0] == 1 {
            return 0
        }
        if x == 1 {
            for i in 0..y {
                if obstacle_grid[0][i] == 1 {
                    return 0
                }
            }
            return 1
        }
        if y == 1 {
            for i in 0..x {
                if obstacle_grid[i][0] == 1 {
                    return 0
                }
            }
            return 1
        }
        let mut m = vec![vec![1; y]; x];
        m[x-1][y-1] = 0;
        for i in 1..x {
            for j in 1..y {
                if obstacle_grid[i][j] == 0 {
                    let a = match obstacle_grid[i-1][j] {
                        1 => 0,
                        _ => m[i-1][j],
                    };
                    let b = match obstacle_grid[i][j-1] {
                        1 => 0,
                        _ => m[i][j-1],
                    };
                    m[i][j] = a + b;
                }
            }
        }
        for l in &m {
            println!("{:?}", l);
        }
        m[x-1][y-1]
    }
}

fn main() {
    // let r = Solution::unique_paths_with_obstacles(vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]]);
    // println!("{r}");
    // let r = Solution::unique_paths_with_obstacles(vec![vec![0,0],vec![0,1]]);
    // println!("{r}");
    // let r = Solution::unique_paths_with_obstacles(vec![vec![0], vec![0]]);
    // println!("{r}");
    // let r = Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0]]);
    // println!("{r}");
    let r = Solution::unique_paths_with_obstacles(vec![vec![0,0],vec![1,1],vec![0,0]]);
    println!("{r}");
}