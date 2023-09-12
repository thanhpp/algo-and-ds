pub struct Solution2 {}

impl Solution2 {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let word = word.chars().collect::<Vec<char>>();
        for r in 0..board.len() {
            for c in 0..board[r].len() {
                if board[r][c] == word[0] && Self::backtrack(&mut board, r, c, &word) {
                    return true;
                }
            }
        }

        false
    }

    pub fn backtrack(board: &mut Vec<Vec<char>>, r: usize, c: usize, word: &[char]) -> bool {
        if board[r][c] == word[0] && word.len() == 1 {
            return true;
        };

        let mut found = false;

        board[r][c] = ' ';
        // up
        if r > 0 && board[r - 1][c] == word[1] {
            found = Self::backtrack(board, r - 1, c, &word[1..]);
        }
        // down
        if !found && r < board.len() - 1 && board[r + 1][c] == word[1] {
            found = Self::backtrack(board, r + 1, c, &word[1..]);
        }
        // left
        if !found && c > 0 && board[r][c - 1] == word[1] {
            found = Self::backtrack(board, r, c - 1, &word[1..]);
        }
        // right
        if !found && c < board[0].len() - 1 && board[r][c + 1] == word[1] {
            found = Self::backtrack(board, r, c + 1, &word[1..]);
        }
        board[r][c] = word[0];

        found
    }
}
