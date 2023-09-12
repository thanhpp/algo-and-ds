package gosol

// this is a better solution
// 1. It modify the given board -> reduce memory by not creating a marker map
// 2. It checks if the word is found and check the if it should continue the path before allocating more stack (enter a new function)
func exist2(board [][]byte, word string) bool {
	var (
		backtrack func(b [][]byte, r, c int, word string) bool
	)

	backtrack = func(b [][]byte, r, c int, word string) bool {
		if b[r][c] == word[0] && len(word) == 1 {
			return true
		}

		var (
			found = false
		)

		b[r][c] = 0
		// up
		if r > 0 && b[r-1][c] == word[1] {
			found = backtrack(b, r-1, c, word[1:])
		}
		// down
		if !found && r < len(b)-1 && b[r+1][c] == word[1] {
			found = backtrack(b, r+1, c, word[1:])
		}
		// left
		if !found && c > 0 && b[r][c-1] == word[1] {
			found = backtrack(b, r, c-1, word[1:])
		}
		// right
		if !found && c < len(b[0])-1 && b[r][c+1] == word[1] {
			found = backtrack(b, r, c+1, word[1:])
		}
		b[r][c] = word[0]

		return found
	}

	for r := range board {
		for c := range board[r] {
			if board[r][c] == word[0] {
				if backtrack(board, r, c, word) {
					return true
				}
			}
		}
	}

	return false
}
