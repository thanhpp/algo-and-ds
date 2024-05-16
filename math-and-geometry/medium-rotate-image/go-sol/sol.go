package gosol

func rotate(matrix [][]int) {
	rows, cols := len(matrix), len(matrix[0])
	startRow, endRow := 0, rows-1
	startCol, endCol := 0, cols-1

	for startRow < endRow {
		r, c := startRow, startCol
		for i := 0; startRow+i < endRow; i++ {
			// top right
			matrix[r][c+i], matrix[r+i][endCol] = matrix[r+i][endCol], matrix[r][c+i]
			// bottom right
			matrix[r][c+i], matrix[endRow][endCol-i] = matrix[endRow][endCol-i], matrix[r][c+i]
			// bottom left
			matrix[r][c+i], matrix[endRow-i][c] = matrix[endRow-i][c], matrix[r][c+i]
		}

		startRow += 1
		endRow -= 1
		startCol += 1
		endCol -= 1
	}
}
