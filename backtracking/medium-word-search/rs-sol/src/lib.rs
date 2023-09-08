pub struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut mark: Vec<Vec<bool>> = vec![vec![false; board[0].len()]; board.len()];
        let word: Vec<char> = word.chars().collect();

        for r in 0..board.len() {
            for c in 0..board[0].len() {
                if Self::search(&board, r, c, &word, 0, &mut mark) {
                    return true;
                }
            }
        }

        false
    }

    pub fn search(
        board: &Vec<Vec<char>>,
        row: usize,
        col: usize,
        word: &[char],
        word_idx: usize,
        mark: &mut Vec<Vec<bool>>,
    ) -> bool {
        if word_idx >= word.len() {
            return true;
        }

        if row >= board.len() {
            return false;
        }

        if col >= board[0].len() {
            return false;
        }

        if mark[row][col] {
            return false;
        }

        if board[row][col] != word[word_idx] {
            return false;
        }

        mark[row][col] = true;

        let mut up: bool = false;
        let mut left: bool = false;

        if row != 0 {
            up = Self::search(board, row - 1, col, word, word_idx + 1, mark);
        }
        let down = Self::search(board, row + 1, col, word, word_idx + 1, mark);

        if col != 0 {
            left = Self::search(board, row, col - 1, word, word_idx + 1, mark);
        }
        let right = Self::search(board, row, col + 1, word, word_idx + 1, mark);

        mark[row][col] = false;

        up || down || right || left
    }
}
