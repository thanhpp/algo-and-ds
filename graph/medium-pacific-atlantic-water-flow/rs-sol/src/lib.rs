pub struct Solution {}

/*
    1 = pacific
    2 = atlantic
    3 = both ocean
*/

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let r_max = heights.len();
        let c_max = heights[0].len();
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut visited: Vec<Vec<u8>> = vec![vec![0; c_max]; r_max];

        for r in 0..r_max {
            // pacific
            Self::dfs(r, 0, r_max, c_max, 1, &heights, &mut visited);

            // atlantic
            Self::dfs(r, c_max - 1, r_max, c_max, 2, &heights, &mut visited);
        }

        for c in 0..c_max {
            // pacific
            Self::dfs(0, c, r_max, c_max, 1, &heights, &mut visited);

            // atlantic
            Self::dfs(r_max - 1, c, r_max, c_max, 2, &heights, &mut visited);
        }

        for r in 0..r_max {
            for c in 0..c_max {
                if visited[r][c] == 3 {
                    result.push(vec![r as i32, c as i32])
                }
            }
        }

        result
    }

    pub fn dfs(
        r: usize,
        c: usize,
        r_max: usize,
        c_max: usize,
        ocean: u8,
        heights: &Vec<Vec<i32>>,
        visited: &mut Vec<Vec<u8>>,
    ) {
        if visited[r][c] == 3 || visited[r][c] == ocean {
            return;
        }

        visited[r][c] += ocean;

        if r > 0 && heights[r - 1][c] >= heights[r][c] {
            Self::dfs(r - 1, c, r_max, c_max, ocean, heights, visited);
        }
        if r < r_max - 1 && heights[r + 1][c] >= heights[r][c] {
            Self::dfs(r + 1, c, r_max, c_max, ocean, heights, visited);
        }

        if c > 0 && heights[r][c - 1] >= heights[r][c] {
            Self::dfs(r, c - 1, r_max, c_max, ocean, heights, visited);
        }
        if c < c_max - 1 && heights[r][c + 1] >= heights[r][c] {
            Self::dfs(r, c + 1, r_max, c_max, ocean, heights, visited);
        }
    }
}
