package gosol

func orangesRotting(grid [][]int) int {
	var (
		maxRow, maxCol = len(grid), len(grid[0])
		rottens        = [][2]int{}
		fresh          = 0
	)

	for r := 0; r < maxRow; r++ {
		for c := 0; c < maxCol; c++ {
			if grid[r][c] == 2 {
				rottens = append(rottens, [2]int{r, c})
				fresh += 1
			}
			if grid[r][c] == 1 {
				fresh += 1
			}
		}
	}
	if len(rottens) == 0 && fresh == 0 {
		return 0
	}

	var minutes = -1
	for l := len(rottens); l != 0; l = len(rottens) {
		minutes += 1
		for i := 0; i < l; i++ {
			// pop head
			rotten := rottens[0]
			rottens = rottens[1:]

			r, c := rotten[0], rotten[1]
			fresh -= 1

			if r > 0 && grid[r-1][c] == 1 {
				grid[r-1][c] = 2
				rottens = append(rottens, [2]int{r - 1, c})
			}
			if r < maxRow-1 && grid[r+1][c] == 1 {
				grid[r+1][c] = 2
				rottens = append(rottens, [2]int{r + 1, c})
			}
			if c > 0 && grid[r][c-1] == 1 {
				grid[r][c-1] = 2
				rottens = append(rottens, [2]int{r, c - 1})
			}
			if c < maxCol-1 && grid[r][c+1] == 1 {
				grid[r][c+1] = 2
				rottens = append(rottens, [2]int{r, c + 1})
			}
		}
	}

	if fresh != 0 {
		return -1
	}

	return minutes
}
