package gosol

// Op1: store rows and cols only
// TIME: O(rows*cols)
// SPACE: O(rows + cols)
func setZeroes1(matrix [][]int) {
	rows, cols := len(matrix), len(matrix[0])

	mark := make(map[int]bool)

	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			if matrix[r][c] != 0 {
				continue
			}

			mark[r] = true
			mark[-c-1] = true // -c - 1 to get rid of the col 0
		}
	}

	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			if mark[r] || mark[-c-1] {
				matrix[r][c] = 0
			}
		}
	}
}
