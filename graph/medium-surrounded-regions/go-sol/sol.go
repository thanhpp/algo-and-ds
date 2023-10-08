package gosol

func solve(board [][]byte) {
	var (
		maxRow = len(board)
		maxCol = len(board[0])
		unflip = make([][]bool, maxRow)
		dfs    func(r, c int)
	)
	dfs = func(r, c int) {
		if board[r][c] == 'X' {
			return
		}
		if unflip[r][c] {
			return
		}
		unflip[r][c] = true

		if r > 0 {
			dfs(r-1, c)
		}
		if r < maxRow-1 {
			dfs(r+1, c)
		}
		if c > 0 {
			dfs(r, c-1)
		}
		if c < maxCol-1 {
			dfs(r, c+1)
		}
	}

	for r := 0; r < maxRow; r++ {
		unflip[r] = make([]bool, maxCol)
	}

	for r := 0; r < maxRow; r++ {
		if board[r][0] == 'O' {
			dfs(r, 0)
		}
		if board[r][maxCol-1] == 'O' {
			dfs(r, maxCol-1)
		}
	}

	for c := 0; c < maxCol; c++ {
		if board[0][c] == 'O' {
			dfs(0, c)
		}
		if board[maxRow-1][c] == 'O' {
			dfs(maxRow-1, c)
		}
	}

	for r := 0; r < maxRow; r++ {
		for c := 0; c < maxCol; c++ {
			if board[r][c] == 'O' && !unflip[r][c] {
				board[r][c] = 'X'
			}
		}
	}
}
