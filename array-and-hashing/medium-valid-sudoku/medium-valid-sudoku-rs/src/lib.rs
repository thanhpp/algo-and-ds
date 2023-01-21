use std::collections::HashSet;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    if board.len() == 0 {
        return true;
    }

    // create hashset to check for rows, cols, and box duplication
    let mut row: HashSet<(u32, usize)> = HashSet::new(); // (num, row)
    let mut col: HashSet<(u32, usize)> = HashSet::new(); // (num, col)
    let mut hs_box: HashSet<(u32, usize)> = HashSet::new(); // (num, box) - box 0..9

    for r in 0..board.len() {
        // row
        for c in 0..board[r].len() {
            // col
            match board[r][c].to_digit(10) {
                None => continue, // not a number
                Some(num) => {
                    if !row.insert((num, r)) {
                        return false;
                    }
                    if !col.insert((num, c)) {
                        return false;
                    }
                    let box_num = (r / 3) + (c / 3);
                    if !hs_box.insert((num, box_num)) {
                        return false;
                    }
                }
            }
        }
    }

    true
}
