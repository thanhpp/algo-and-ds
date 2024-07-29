pub struct Solution {}

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let (rows, cols) = (matrix.len(), matrix[0].len());

        // check by row 0
        for r in 0..rows {
            let (mut i, mut j) = (r, 0);
            while i < rows && j < cols {
                if matrix[i][j] != matrix[r][0] {
                    return false;
                }
                i += 1;
                j += 1;
            }
        }

        // check by col 0 (skip 0, 0)
        for c in 1..cols {
            let (mut i, mut j) = (0, c);
            while i < rows && j < cols {
                if matrix[i][j] != matrix[0][c] {
                    return false;
                }
                i += 1;
                j += 1;
            }
        }

        true
    }
}
