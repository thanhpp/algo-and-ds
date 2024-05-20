package gosol

func setZeroes(matrix [][]int) {
	var (
		// SPACE: O(1)
		rows, cols   = len(matrix), len(matrix[0])
		zeroFirstRow = false
	)

	for c := 0; c < cols; c++ {
		if matrix[0][c] == 0 {
			zeroFirstRow = true
			break
		}
	}

	// matrix[0][0] -> mark first col to be 0 or not
	for r := 0; r < rows; r++ {
		if matrix[r][0] == 0 {
			matrix[0][0] = 0
			break
		}
	}

	for r := 1; r < rows; r++ {
		for c := 1; c < cols; c++ {
			if matrix[r][c] != 0 {
				continue
			}

			matrix[0][c] = 0
			matrix[r][0] = 0

		}
	}

	// for r := 0; r  < rows; r++ {
	//     fmt.Println(matrix[r])
	// }

	for r := 1; r < rows; r++ {
		for c := 1; c < cols; c++ {
			if matrix[r][0] == 0 || matrix[0][c] == 0 {
				matrix[r][c] = 0
			}
		}
	}

	if matrix[0][0] == 0 {
		for r := 1; r < rows; r++ {
			matrix[r][0] = 0
		}
	}

	if zeroFirstRow {
		for c := 0; c < cols; c++ {
			matrix[0][c] = 0
		}
	}

	/*
		TIME: O(m * n)
	*/
}
