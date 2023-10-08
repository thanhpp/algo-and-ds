pub struct Solution;

// Solution: find path of unflipped node (start from O at the boundaries)
// Big(O): O(m * n) -> worst case visit each node once

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (max_row, max_col) = (board.len(), board[0].len());
        for r in 0..max_row {
            if board[r][0] == 'O' {
                Self::dfs((max_row, max_col), (r, 0), board);
            }
            if board[r][max_col - 1] == 'O' {
                Self::dfs((max_row, max_col), (r, max_col - 1), board)
            }
        }

        for c in 0..max_col {
            if board[0][c] == 'O' {
                Self::dfs((max_row, max_col), (0, c), board);
            }
            if board[max_row - 1][c] == 'O' {
                Self::dfs((max_row, max_col), (max_row - 1, c), board)
            }
        }

        for r in 0..max_row {
            for c in 0..max_col {
                match board[r][c] {
                    'U' => board[r][c] = 'O',
                    'O' => board[r][c] = 'X',
                    _ => {}
                }
            }
        }
    }

    pub fn dfs(
        (max_row, max_col): (usize, usize),
        (r, c): (usize, usize),
        board: &mut Vec<Vec<char>>,
    ) {
        board[r][c] = 'U';

        if r > 0 && board[r - 1][c] == 'O' {
            Self::dfs((max_row, max_col), (r - 1, c), board);
        }
        if r < max_row - 1 && board[r + 1][c] == 'O' {
            Self::dfs((max_row, max_col), (r + 1, c), board);
        }
        if c > 0 && board[r][c - 1] == 'O' {
            Self::dfs((max_row, max_col), (r, c - 1), board);
        }
        if c < max_col - 1 && board[r][c + 1] == 'O' {
            Self::dfs((max_row, max_col), (r, c + 1), board);
        }
    }
}
