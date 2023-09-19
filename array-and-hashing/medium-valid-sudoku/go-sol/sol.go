package gosol

func isValidSudoku(board [][]byte) bool {
    var (
        row, col, box [9][9]bool
    )

    for r := 0; r < 9; r++ {
        for c := 0; c < 9; c++ {
            if board[r][c] == '.' {
                continue
            }
            v := board[r][c] - 49 // "1" -> 0
            if row[r][v] {
                return false
            }
            if col[c][v] {
                return false
            }
            if box[r/3 + c/3 * 3][v] {
                return false
            }
            row[r][v] = true
            col[c][v] = true
            box[r/3 + c/3 * 3][v] = true
        }
    }

    return true
}
