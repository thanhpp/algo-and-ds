pub struct Solution {}

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut max_area = 0;

        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                Self::cal_area(&mut grid, r, c, &mut 0, &mut max_area)
            }
        }

        max_area
    }

    fn cal_area(grid: &mut Vec<Vec<i32>>, r: usize, c: usize, area: &mut i32, max_area: &mut i32) {
        if grid[r][c] != 1 {
            return;
        }

        *area += 1;
        (*grid)[r][c] = 2;
        if area.gt(&max_area) {
            *max_area = *area
        }

        if r > 0 {
            Self::cal_area(grid, r - 1, c, area, max_area);
        }
        if r < grid.len() - 1 {
            Self::cal_area(grid, r + 1, c, area, max_area)
        }
        if c > 0 {
            Self::cal_area(grid, r, c - 1, area, max_area);
        }
        if c < grid[0].len() - 1 {
            Self::cal_area(grid, r, c + 1, area, max_area);
        }
    }
}
