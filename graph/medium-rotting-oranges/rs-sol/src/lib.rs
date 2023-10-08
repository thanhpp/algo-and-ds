pub struct Solution;

// Solution: BFS - consider each layer of the tree is a minute passed
// Big(O) = m*n (worst case - visit every node)

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let (max_row, max_col) = (grid.len(), grid[0].len());
        let mut rottens = std::collections::VecDeque::<(usize, usize)>::new();
        let mut fresh: i32 = 0;
        for r in 0..max_row {
            for c in 0..max_col {
                match grid[r][c] {
                    2 => rottens.push_back((r, c)),
                    1 => fresh += 1,
                    _ => {}
                }
            }
        }

        if rottens.is_empty() {
            if fresh == 0 {
                return 0;
            }
            return -1;
        }

        let mut min = -1; // already rotted
        fresh += rottens.len() as i32;
        while !rottens.is_empty() {
            let l = rottens.len();
            min += 1; // each minute
            for _ in 0..l {
                fresh -= 1;
                let (r, c) = match rottens.pop_front() {
                    Some(v) => v,
                    None => break,
                };
                grid[r][c] = 2;
                if r > 0 && grid[r - 1][c] == 1 {
                    grid[r - 1][c] = 2;
                    rottens.push_back((r - 1, c));
                }
                if r < max_row - 1 && grid[r + 1][c] == 1 {
                    grid[r + 1][c] = 2;
                    rottens.push_back((r + 1, c));
                }
                if c > 0 && grid[r][c - 1] == 1 {
                    grid[r][c - 1] = 2;
                    rottens.push_back((r, c - 1));
                }
                if c < max_col - 1 && grid[r][c + 1] == 1 {
                    grid[r][c + 1] = 2;
                    rottens.push_back((r, c + 1))
                }
            }
        }

        if fresh != 0 {
            return -1;
        }

        min
    }
}
