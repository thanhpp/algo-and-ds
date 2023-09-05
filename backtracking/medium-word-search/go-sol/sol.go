package gosol

func exist(board [][]byte, word string) bool {
	var (
		m         = len(board)
		n         = len(board[0])
		mark      = make([][]bool, m)
		backtrack func(i, j, wordIdx int) bool
	)

	backtrack = func(i, j, wordIdx int) bool {
		if wordIdx >= len(word) {
			return true
		}

		if i >= len(board) || i < 0 {
			return false
		}

		if j >= len(board[0]) || j < 0 {
			return false
		}

		if mark[i][j] {
			return false // already checked
		}

		if board[i][j] != word[wordIdx] {
			return false
		}

		var (
			up, down, left, right bool
		)

		mark[i][j] = true
		up = backtrack(i, j-1, wordIdx+1)
		down = backtrack(i, j+1, wordIdx+1)
		left = backtrack(i-1, j, wordIdx+1)
		right = backtrack(i+1, j, wordIdx+1)
		mark[i][j] = false
		return up || down || left || right
	}

	for i := 0; i < m; i++ {
		mark[i] = make([]bool, n)
	}

	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if board[i][j] != word[0] {
				continue
			}
			if backtrack(i, j, 0) {
				return true
			}
		}
	}

	return false
}
